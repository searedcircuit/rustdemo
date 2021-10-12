use crate::data::db::{DbMenuOption,DbMenuItem};
use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct MenuResponse{
    pub store_id: uuid::Uuid,
    pub items: Vec<DbMenuItem>,
    pub options: Vec<DbMenuOption>
}

impl Default for MenuResponse {
    fn default () -> MenuResponse{ 
        MenuResponse {
            store_id: uuid::Uuid::default(),
            items:Vec::new(),
            options:Vec::new()
        }
    } 
}
