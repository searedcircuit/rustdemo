use cdrs_tokio::query::QueryValues;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use chrono::prelude::*;
use chrono::serde::ts_seconds;

use cdrs_tokio::frame::AsBytes;
use cdrs_tokio::types::from_cdrs::FromCdrsByName;
use cdrs_tokio::types::prelude::*;
use cdrs_tokio::query_values;
use serde::{Deserialize,Serialize};

use cdrs_tokio_helpers_derive::*;

#[derive(Clone, Debug, IntoCdrsValue, TryFromRow, PartialEq,Serialize,Deserialize)]
pub struct UserInfo{
    pub userid: Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub active: bool,
    #[serde(with = "ts_seconds")]
    pub created_date: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub modified_date: DateTime<Utc>,
}

impl UserInfo {
    pub fn into_query_values(self) -> QueryValues {
        query_values!(
            "userid" => self.userid, 
            "email" => self.email, 
            "firstname" => self.firstname, 
            "lastname" => self.lastname,
            "active"=>self.active, 
            "created_date"=>self.created_date, 
            "modified_date"=>self.modified_date)
    }
}

impl Default for UserInfo {
    fn default () -> UserInfo{ 
        UserInfo {
            userid: Uuid::nil(),
            email:String::new(),
            firstname: String::new(),
            lastname:String::new(),
            active:false,
            created_date:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0,0),Utc),
            modified_date:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0,0),Utc)
        }
    } 
}
