use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateMenuRequest{
    pub store_id: uuid::Uuid,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub size: Vec<String>,
    pub temp: Vec<String>,
    pub cost: Option<i32>
}

impl Default for CreateMenuRequest {
    fn default () -> CreateMenuRequest{ 
        CreateMenuRequest {
            store_id: uuid::Uuid::default(),
            name: None,
            desc: None,
            size: Vec::new(),
            temp: Vec::new(),
            cost: None
        }
    } 
}
