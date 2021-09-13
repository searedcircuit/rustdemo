use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct UserLoginResponse{
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl Default for UserLoginResponse {
    fn default () -> UserLoginResponse{ 
        UserLoginResponse {
            access_token:Some(String::new()),
            refresh_token: Some(String::new()),
        }
    } 
}
