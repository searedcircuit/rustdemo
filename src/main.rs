use actix_web::{web,App,HttpServer};

pub mod auth{

}
pub mod data{
    pub mod db{
        mod db_user_info;
        pub use db_user_info::DbUserInfo;
    }
    pub mod request{
        pub mod auth{
            pub mod login_request;
            pub use login_request::UserLoginRequest;
        }
        pub mod store{
            pub mod create_store_user_request;
            pub use create_store_user_request::CreateStoreUserRequest;
        }
        pub mod user{
            pub mod create_user_request;
            pub use create_user_request::CreateUserRequest;
        }
    }
    pub mod response{
        pub mod create_user_response;
        pub mod login_response;

        pub use create_user_response::CreateUserResponse;
        pub use login_response::UserLoginResponse;
    }
}
pub mod db{
    pub mod common{
        mod scylla_db_ops;
        pub use scylla_db_ops::create_session;
    }
    pub mod user{
        mod scylla_user_db_ops;
        pub use scylla_user_db_ops::create_all as user_create_all;
        pub use scylla_user_db_ops::insert_user;
        pub use scylla_user_db_ops::select_user;
        pub use scylla_user_db_ops::user_login;
    }
    pub mod store{
        mod scylla_store_db_ops;
        pub use scylla_store_db_ops::create_all as store_create_all;
        pub use scylla_store_db_ops::insert_store_user;
        pub use scylla_store_db_ops::select_store_user;
    }
}
pub mod handlers{
    mod user_handler;
    mod store_user_handler;

    pub use user_handler::{create as user_create,get as user_get,login as user_login};    
    pub use store_user_handler::{create as store_create, get as store_get}; 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let create_session_result = db::common::create_session().await;
    if let Ok(pool) = create_session_result {
        let _created_userinfo = db::user::user_create_all(&pool.clone()).await;
        let _created_storeinfo = db::store::store_create_all(&pool.clone()).await;
        println!("db connect complete");
       HttpServer::new(move|| App::new()
       .service(handlers::user_create).app_data(web::Data::new(pool.clone()))
       .service(handlers::user_get).app_data(web::Data::new(pool.clone()))
       .service(handlers::user_login).app_data(web::Data::new(pool.clone()))
       
       .service(handlers::store_create).app_data(web::Data::new(pool.clone()))
       .service(handlers::store_get).app_data(web::Data::new(pool.clone()))
    )
           .bind("0.0.0.0:8081")?
           .run()
           .await?;
    } else {
        panic!("error connecting")
    }
    Ok(())
}