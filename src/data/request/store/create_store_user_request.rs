use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateStoreUserRequest{
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
}

impl Default for CreateStoreUserRequest {
    fn default () -> CreateStoreUserRequest{ 
        CreateStoreUserRequest {
            email:Some(String::new()),
            password_hash:Some(String::new()),
            firstname: Some(String::new()),
            lastname:Some(String::new()),
        }
    } 
}
