use actix_web::HttpServer;
use cdrs_tokio::cluster::TcpConnectionPool;
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::cluster::session::Session;
use actix_web::{get,web,App,HttpResponse,Result};

mod db_ops;
mod user_info;
type CurrentSession = Session<RoundRobin<TcpConnectionPool>>;

#[get("/{name}")]
async fn greet() -> Result<HttpResponse> {
    //let name = req.match_info().get("name").unwrap_or("World");
    let mut no_compression: CurrentSession = db_ops::create_session().await;
    db_ops::insert_struct(&mut no_compression).await;
    let uinf = db_ops::select_struct(&mut no_compression).await;
    //format!("Hello {}!", &name)
    Ok(HttpResponse::Ok().json(uinf))
}

#[get("/{name}")]
async fn index(path: web::Path<String>) -> Result<HttpResponse> {
    //let name = path.into_inner();
    let mut no_compression: CurrentSession = db_ops::create_session().await;
    db_ops::insert_struct(&mut no_compression).await;
    let uinf = db_ops::select_struct(&mut no_compression).await;
    //format!("Hello {}!", &name)
    Ok(HttpResponse::Ok().json(uinf))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db_ops::create_all().await;
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}