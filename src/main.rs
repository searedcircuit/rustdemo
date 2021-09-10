use std::sync::Arc;
use actix_web::{get,post,web,App,HttpResponse,HttpServer,Result};

use scylla::Session;

mod scylla_db_ops;
mod scylla_user_db_ops;
mod scylla_store_db_ops;
mod user_info;

#[post("/{name}")]
async fn create(pool: web::Data<Arc<Session>>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();

    let email = format!("tom-{}@searedcircuit.com", uuid::Uuid::new_v4());

    let res = scylla_user_db_ops::insert_user(&poolconn,&email).await;
    match res {
        Ok(userid) =>     Ok(HttpResponse::Ok()            
            .content_type("application/json")
            .body(format!(r#"{{"userid":"{}"}}"#, userid))),
        Err(err) => Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err)))
    }
}

#[get("/{id}")]
async fn get(pool: web::Data<Arc<Session>>,id:web::Path<String>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let uid = id.into_inner();
    let res = uuid::Uuid::parse_str(&uid);
    match res {
        Ok(userid)=>{
            let ures = scylla_user_db_ops::select_user(&poolconn,userid).await;
            match ures {
                Ok(uinf) =>Ok(HttpResponse::Ok().json(uinf)),
                Err(e) =>             Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(format!(r#"{{"error":"{}"}}"#, e)))
            }
        },
        Err(e) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, e)))
    }
}

//fn main() -> Result<> {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let create_session_result = scylla_db_ops::create_session().await;
    match create_session_result {
        Ok(pool) => {
            let _created_userinfo = scylla_user_db_ops::create_all(&pool.clone()).await;
            let _created_storeinfo = scylla_store_db_ops::create_all(&pool.clone()).await;
            println!("db connect complete");
           HttpServer::new(move|| App::new()
           .service(create).app_data(web::Data::new(pool.clone()))
           .service(get).app_data(web::Data::new(pool.clone())))
               .bind("0.0.0.0:8080")?
               .run()
               .await?;
        }
        Err(_)=> panic!("error connecting")
    }
    Ok(())
}