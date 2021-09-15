use std::sync::Arc;

use actix_web::{get,post, web,HttpResponse,Result};
use scylla::Session;

use crate::data::request::auth::UserLoginRequest;
use crate::db::user::{insert_user,select_user,user_login};
use crate::data::request::user::CreateUserRequest;

#[post("/user/create")]
pub async fn create(pool: web::Data<Arc<Session>>,user_content: web::Json<CreateUserRequest>) -> Result<HttpResponse> {
let poolconn = pool.get_ref();
    let mut user = user_content.into_inner();

    let res = insert_user(&poolconn,&mut user).await;
    match res {
        Ok(userid) =>     
            Ok(HttpResponse::Ok()            
            .content_type("application/json")
            .body(format!(r#"{{"userid":"{}"}}"#, userid))),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err)))
    }
}

#[post("/user/login")]
async fn login(pool: web::Data<Arc<Session>>,login_content: web::Json<UserLoginRequest>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let login = login_content.into_inner();

    let res = user_login(poolconn,&login).await;
    match res {
        Ok(userid)=>
            Ok(HttpResponse::Ok().json(userid)),
        Err(e) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, e)))
    }
}

#[get("/user/{id}")]
async fn get(pool: web::Data<Arc<Session>>,id:web::Path<String>) -> Result<HttpResponse> {
    let poolconn = pool.get_ref();
    let uid = id.into_inner();
    let res = uuid::Uuid::parse_str(&uid);
    match res {
        Ok(userid)=>{
            let ures = select_user(&poolconn,userid).await;
            match ures {
                Ok(uinf) =>
                    Ok(HttpResponse::Ok().json(uinf)),
                Err(e) =>             
                    Ok(HttpResponse::BadRequest()
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