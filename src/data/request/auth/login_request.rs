use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct UserLoginRequest{
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

impl Default for UserLoginRequest {
    fn default () -> UserLoginRequest{ 
        UserLoginRequest {
            email:Some(String::new()),
            password_hash: Some(String::new()),
        }
    } 
}
