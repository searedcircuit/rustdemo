use actix_web::{get,web,App,HttpResponse,HttpServer,Result};

use std::sync::Arc;

use cdrs_tokio::authenticators::NoneAuthenticatorProvider;
use cdrs_tokio::cluster::session::{new as new_session, Session};
use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionManager};
use cdrs_tokio::query::*;
use cdrs_tokio::query_values;
use cdrs_tokio::retry::{DefaultRetryPolicy, NeverReconnectionPolicy};

use cdrs_tokio::frame::AsBytes;
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::types::from_cdrs::FromCdrsByName;
use cdrs_tokio::types::prelude::*;

use cdrs_tokio::transport::TransportTcp;
use cdrs_tokio_helpers_derive::*;

mod db_ops;
mod user_info;
type CurrentSession = Session<TransportTcp, TcpConnectionManager, RoundRobin<TcpConnectionManager>>;

#[get("/{name}")]
async fn index(pool: web::Data<&mut CurrentSession>,path: web::Path<String>) -> Result<HttpResponse> {
    let mut poolconn = pool.get_ref();
    
    //let name = path.into_inner();
    //db_ops::insert_struct(&mut pool).await;
    let uinf = db_ops::select_struct(&mut poolconn).await;
    //format!("Hello {}!", &name)
    Ok(HttpResponse::Ok().json(uinf))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db_ops::create_all().await;
    let pool: CurrentSession = db_ops::create_session().await;
    HttpServer::new(|| App::new().service(index).app_data(web::Data::new(&pool)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}