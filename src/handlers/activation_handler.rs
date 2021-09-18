use uuid::Uuid;
use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Result, body};
use scylla::Session;

use crate::db::user::activate_user;

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