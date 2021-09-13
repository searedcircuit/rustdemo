use uuid::Uuid;
use chrono::prelude::*;

use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct DbUserInfo{
    pub userid: Uuid,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
}

impl Default for DbUserInfo {
    fn default () -> DbUserInfo{ 
        DbUserInfo {
            userid: Uuid::nil(),
            email:Some(String::new()),
            firstname: Some(String::new()),
            lastname:Some(String::new()),
            active:false,
            created_date:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0,0),Utc),
            modified_date:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0,0),Utc)
        }
    } 
}
