use std::sync::Arc;

use actix_web::{get,post, web,HttpResponse,Result};
use scylla::Session;

use crate::{
    data::request::store::create_store_request::CreateStoreRequest, 
    db::store::{insert_store,select_stores
    }
};

#[post("/stores/create")]
pub async fn create(pool: web::Data<Arc<Session>>,store_content: web::Json<CreateStoreRequest>) -> Result<HttpResponse> {
let poolconn = pool.get_ref();
    let mut store = store_content.into_inner();

    let res = insert_store(&poolconn,&mut store).await;
    match res {
        Ok(activation_code) =>     
            Ok(HttpResponse::Ok()            
            .content_type("application/json")
            .body(format!(r#"{{"activation_code":"{}"}}"#, activation_code))),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}

#[get("/user/{id}")]
async fn get(pool: web::Data<Arc<Session>>,loc:web::Path<(f64,f64)>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let (lat,lng) = loc.into_inner();
    let ures = select_stores(&poolconn,lat,lng).await;
    match ures {
        Ok(uinf) =>
            Ok(HttpResponse::Ok().json(uinf)),
        Err(e) =>             
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, e.to_string())))
    }
}