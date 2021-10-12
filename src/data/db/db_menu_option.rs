use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct DbMenuOption{
    pub name: Option<String>,
    pub cost: Option<i32>
}

impl Default for DbMenuOption {
    fn default () -> DbMenuOption{ 
        DbMenuOption {
            name: None,
            cost: None
        }
    } 
}
