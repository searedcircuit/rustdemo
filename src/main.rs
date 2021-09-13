use actix_web::{web,App,HttpServer};

mod db;
mod data;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let create_session_result = db::common::scylla_db_ops::create_session().await;
    if let Ok(pool) = create_session_result {
        let _created_userinfo = db::user::scylla_user_db_ops::create_all(&pool.clone()).await;
        let _created_storeinfo = db::store::scylla_store_db_ops::create_all(&pool.clone()).await;
        println!("db connect complete");
       HttpServer::new(move|| App::new()
       .service(handlers::user_handler::create).app_data(web::Data::new(pool.clone()))
       .service(handlers::user_handler::get).app_data(web::Data::new(pool.clone()))
       
       .service(handlers::store_user_handler::create).app_data(web::Data::new(pool.clone()))
       .service(handlers::store_user_handler::get).app_data(web::Data::new(pool.clone()))
    )
           .bind("0.0.0.0:8081")?
           .run()
           .await?;
    } else {
        panic!("error connecting")
    }
    Ok(())
}