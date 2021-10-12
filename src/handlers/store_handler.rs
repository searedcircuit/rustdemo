use std::sync::Arc;

use actix_web::{HttpResponse, Result, body, get, post, web};
use scylla::Session;

use crate::{
    data::request::{
        store::create_store_request::CreateStoreRequest,
        menu::{
            CreateMenuRequest,
            CreateOrUpdateMenuOptionRequest
        }        
    }, 
    db::{
        store::{
            insert_store,
            select_stores
        },
        menu::{
            insert_menu_item,
            insert_menu_option
        }
    },
    
};

#[post("/stores/create")]
pub async fn create_store(pool: web::Data<Arc<Session>>,store_content: web::Json<CreateStoreRequest>) -> Result<HttpResponse> {
let poolconn = pool.get_ref();
    let mut store = store_content.into_inner();

    let res = insert_store(&poolconn,&mut store).await;
    match res {
        Ok(()) =>     
            Ok(HttpResponse::Created().body(body::Body::Empty)),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}

#[post("/stores/menu/create")]
pub async fn create_menu_item(pool: web::Data<Arc<Session>>,menu_content: web::Json<CreateMenuRequest>) -> Result<HttpResponse> {
let poolconn = pool.get_ref();
    let mut menu = menu_content.into_inner();

    let res = insert_menu_item(&poolconn,&mut menu).await;
    match res {
        Ok(()) =>     
            Ok(HttpResponse::Created().body(body::Body::Empty)),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}

#[post("/stores/menu/options/create")]
pub async fn create_menu_option(pool: web::Data<Arc<Session>>,menu_opt_content: web::Json<CreateOrUpdateMenuOptionRequest>) -> Result<HttpResponse> {
let poolconn = pool.get_ref();
    let mut menu_opt = menu_opt_content.into_inner();

    let res = insert_menu_option(&poolconn,&mut menu_opt).await;
    match res {
        Ok(()) =>     
            Ok(HttpResponse::Created().body(body::Body::Empty)),
        Err(err) => 
            Ok(HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"{}"}}"#, err.to_string())))
    }
}

#[get("/stores/find/{lat}/{lng}")]
async fn get_store(pool: web::Data<Arc<Session>>,loc:web::Path<(f64,f64)>) -> Result<HttpResponse> {
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

#[get("/stores/find/{lat}/{lng}")]
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

