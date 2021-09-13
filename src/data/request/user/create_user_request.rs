use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateUserRequest{
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl Default for CreateUserRequest {
    fn default () -> CreateUserRequest{ 
        CreateUserRequest {
            email:Some(String::new()),
            password_hash:Some(String::new()),
            firstname: Some(String::new()),
            lastname:Some(String::new()),
        }
    } 
}
