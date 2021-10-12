use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct DbMenuItem{
    pub item_id: uuid::Uuid,

    pub item_name: Option<String>,
    pub item_desc: Option<String>,
    pub item_size: Option<String>,
    pub item_temp: Option<String>,
    pub item_cost: Option<i32>
}

impl Default for DbMenuItem {
    fn default () -> DbMenuItem { 
        DbMenuItem {
            item_id: uuid::Uuid::default(),

            item_name: None,
            item_desc: None,
            item_size: None,
            item_temp: None,
            item_cost: None
        }
    } 
}
