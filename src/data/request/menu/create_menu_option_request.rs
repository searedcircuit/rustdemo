use crate::data::db::DbMenuOption;
use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateOrUpdateMenuOptionRequest{
    pub store_id: uuid::Uuid,
    pub options: Vec<DbMenuOption>,
}

impl Default for CreateOrUpdateMenuOptionRequest {
    fn default () -> CreateOrUpdateMenuOptionRequest{ 
        CreateOrUpdateMenuOptionRequest {
            store_id: uuid::Uuid::default(),
            options:Vec::new()
        }
    } 
}
