use actix_web::{web,App,HttpServer};

pub mod auth{

}
pub mod data{
    pub mod db{
        mod db_user_info;
        mod db_menu_item;
        mod db_menu_option;
        
        pub use db_user_info::DbUserInfo;
        pub use db_menu_item::DbMenuItem;
        pub use db_menu_option::DbMenuOption;
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
        pub mod menu{
            pub mod create_menu_request;
            pub mod create_menu_option_request;
            pub use create_menu_request::CreateMenuRequest;
            pub use create_menu_option_request::CreateOrUpdateMenuOptionRequest;
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
        pub mod menu{
            pub mod get_menu_response;
            pub use get_menu_response::MenuResponse;
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
    pub mod menu{
        mod scylla_menu_db_ops;
        pub use scylla_menu_db_ops::{
            insert_menu_item,
            insert_menu_option
        };
    }
}
pub mod handlers{
    mod user_handler;
    mod activation_handler;
    mod store_handler;

    pub use user_handler::{create as user_create,get as user_get,login as user_login};   
    pub use activation_handler::{activate as activate_user};  
    pub use store_handler::{create_store,get_store};   
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
               .service(handlers::create_store).app_data(web::Data::new(pool.clone()))
               
               .service(handlers::user_get).app_data(web::Data::new(pool.clone()))
               .service(handlers::user_login).app_data(web::Data::new(pool.clone()))
               .service(handlers::get_store).app_data(web::Data::new(pool.clone()))
            )
            .bind("0.0.0.0:8081")?
            .run()
            .await;
        }
        Err(e) => panic!("error connecting {}",e)
    }
    Ok(())
}