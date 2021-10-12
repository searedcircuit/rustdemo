use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct DbMenuOption{
    pub option_id: uuid::Uuid,
    pub name: Option<String>,
    pub cost: Option<i32>
}

impl Default for DbMenuOption {
    fn default () -> DbMenuOption{ 
        DbMenuOption {
            option_id: uuid::Uuid::default(),
            name: None,
            cost: None
        }
    } 
}
