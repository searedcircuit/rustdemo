use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct UserLoginResponse{
    pub access_code: uuid::Uuid,
    pub refresh_code: uuid::Uuid,
    pub userid: uuid::Uuid,
}

impl Default for UserLoginResponse {
    fn default () -> UserLoginResponse{ 
        UserLoginResponse {
            access_code:uuid::Uuid::default(),
            refresh_code: uuid::Uuid::default(),
            userid: uuid::Uuid::default()
        }
    } 
}
