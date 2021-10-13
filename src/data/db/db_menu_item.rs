use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct DbMenuItem{
    pub item_id: uuid::Uuid,

    pub item_name: Option<String>,
    pub item_desc: Option<String>,
    pub item_size: Vec<String>,
    pub item_temp: Vec<String>,
    pub item_cost: Option<i32>
}

impl Default for DbMenuItem {
    fn default () -> DbMenuItem { 
        DbMenuItem {
            item_id: uuid::Uuid::default(),

            item_name: None,
            item_desc: None,
            item_size: Vec::new(),
            item_temp: Vec::new(),
            item_cost: None
        }
    } 
}
