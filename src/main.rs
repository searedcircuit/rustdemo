use std::sync::Arc;
use cdrs_tokio::cluster::ConnectionPool;
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Result};

use cdrs_tokio::cluster::session::Session;

use cdrs_tokio::load_balancing::RoundRobin;

use cdrs_tokio::transport::TransportTcp;

mod db_ops;
mod user_info;
type CurrentSession = Session<RoundRobin<ConnectionPool<TransportTcp>>>;

#[post("/{name}")]
async fn create(pool: web::Data<Arc<CurrentSession>>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();

    let email = format!("tom-{}@searedcircuit.com", uuid::Uuid::new_v4());

    let userid = db_ops::insert_struct(&poolconn,&email).await;
    Ok(HttpResponse::Ok()            
    .content_type("application/json")
    .body(format!(r#"{{"userid":"{}"}}"#, userid)))
}

#[get("/{id}")]
async fn get(pool: web::Data<Arc<CurrentSession>>,id:web::Path<String>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let uid = id.into_inner();
    let res = uuid::Uuid::parse_str(&uid);
    match res {
        Ok(userid)=>{
            let uinf = db_ops::select_struct(&poolconn,userid).await;
            Ok(HttpResponse::Ok().json(uinf))
        },
        Err(e) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, e)))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let pool: Arc<CurrentSession> = db_ops::create_session().await;
   db_ops::create_all(&pool.clone()).await;
    println!("db connect complete");
   HttpServer::new(move|| App::new()
   .service(create).app_data(web::Data::new(pool.clone()))
   .service(get).app_data(web::Data::new(pool.clone())))
       .bind("0.0.0.0:8080")?
       .run()
       .await
}