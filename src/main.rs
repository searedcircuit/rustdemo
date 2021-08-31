use std::sync::Arc;
use cdrs_tokio::cluster::ConnectionPool;
use actix_web::{get,web,App,HttpResponse,HttpServer,Result};

use cdrs_tokio::cluster::session::Session;

use cdrs_tokio::load_balancing::RoundRobin;

use cdrs_tokio::transport::TransportTcp;

mod db_ops;
mod user_info;
type CurrentSession = Session<RoundRobin<ConnectionPool<TransportTcp>>>;

#[get("/{name}")]
async fn index(pool: web::Data<Arc<CurrentSession>>,path: web::Path<String>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    
    let name = path.into_inner();
    //db_ops::insert_struct(&poolconn).await;
    let uinf = db_ops::select_struct(&poolconn).await;
    //format!("Hello {}!", &name)
    Ok(HttpResponse::Ok().body(format!("Hello {}",&name)))
    //Ok(HttpResponse::Ok().json(uinf))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool: Arc<CurrentSession> = db_ops::create_session().await;
    db_ops::create_all(&pool.clone()).await;
    println!("db connect complete");
    HttpServer::new(move|| App::new().service(index).app_data(web::Data::new(pool.clone())))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}