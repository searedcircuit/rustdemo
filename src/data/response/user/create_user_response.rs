use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateUserResponse{
    pub email_auth_token: Option<String>,
}

impl Default for CreateUserResponse {
    fn default () -> CreateUserResponse{ 
        CreateUserResponse {
            email_auth_token:Some(String::new()),
        }
    } 
}
