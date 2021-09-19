use uuid::Uuid;
use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Result, body};
use scylla::Session;

use crate::db::user::{activate_user,insert_activation_code};

#[post("/user/activate/{web_activation_code}")]
pub async fn activate(pool: web::Data<Arc<Session>>,web_activation_code: web::Path<Uuid>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let activation_code = web_activation_code.into_inner();

    let res = activate_user(&poolconn, &activation_code).await;
    match res {
        Ok(_) =>     
            Ok(HttpResponse::Created().body(body::Body::Empty)),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}

#[post("/user/reactivate/{web_email}")]
pub async fn reactivate(pool: web::Data<Arc<Session>>,web_email: web::Path<String>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let email = web_email.into_inner();

    let res = insert_activation_code(&poolconn, &email).await;
    match res {
        Ok(code) =>     
            Ok(HttpResponse::Created()
            .content_type("application/json")
            .body(format!(r#"{{"activation_code":"{}"}}"#, code))),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}