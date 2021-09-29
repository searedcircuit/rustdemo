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
            pub mod create_store_request;
            pub use create_store_request::CreateStoreRequest;
        }
        pub mod user{
            pub mod create_user_request;
            pub use create_user_request::CreateUserRequest;
        }
    }
    pub mod response{
        pub mod user{
            pub mod create_user_response;
            pub mod login_response;

            pub use create_user_response::CreateUserResponse;
            pub use login_response::UserLoginResponse;
        }
        pub mod store{
            pub mod get_store_response;
            pub use get_store_response::StoreResponse;
        }
    }
}
pub mod db{
    pub mod auth{
        mod scylla_auth_db_ops;
        pub use scylla_auth_db_ops::get_auth_codes;
    }
    pub mod common{
        mod scylla_db_ops;
        pub use scylla_db_ops::*;
    }    
    pub mod user{
        mod scylla_user_db_ops;
        pub use scylla_user_db_ops::insert_user;
        pub use scylla_user_db_ops::select_user;
        pub use scylla_user_db_ops::user_login;

        mod scylla_activate_user_db_ops;
        pub use scylla_activate_user_db_ops::{activate_user,insert_activation_code};
    }
    pub mod store{
        mod scylla_store_db_ops;
        pub use scylla_store_db_ops::{insert_store,select_stores};

    }
}
pub mod handlers{
    mod user_handler;
    mod activation_handler;
    mod store_handler;

    pub use user_handler::{create as user_create,get as user_get,login as user_login};   
    pub use activation_handler::{activate as activate_user};  
    pub use store_handler::{create as store_create,get as store_get};   
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let res = db::common::create_session().await;
    match res {
        Ok(pool) => {
            let _created = db::common::create_tables(&pool.clone()).await.expect("table creation failed.");
            println!("db connect complete");
            
            let _srv = HttpServer::new(move|| App::new()
               .service(handlers::user_create).app_data(web::Data::new(pool.clone()))
               .service(handlers::activate_user).app_data(web::Data::new(pool.clone()))
               .service(handlers::store_create).app_data(web::Data::new(pool.clone()))
               
               .service(handlers::user_get).app_data(web::Data::new(pool.clone()))
               .service(handlers::user_login).app_data(web::Data::new(pool.clone()))
               .service(handlers::store_get).app_data(web::Data::new(pool.clone()))
            )
            .bind("0.0.0.0:8081")?
            .run()
            .await;
        }
        Err(e) => panic!("error connecting {}",e)
    }
    Ok(())
}