use uuid::Uuid;
use chrono::prelude::*;

use serde::{Deserialize,Serialize};


#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct UserInfo{
    pub userid: Uuid,
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub active: bool,
    pub created_date: DateTime<Utc>,
    pub modified_date: DateTime<Utc>,
}

// impl UserInfo {
//     pub fn into_query_values(self) -> QueryValues {
//         query_values!(
//             "userid" => self.userid, 
//             "email" => self.email, 
//             "firstname" => self.firstname, 
//             "lastname" => self.lastname,
//             "active"=>self.active, 
//             "created_date"=>self.created_date, 
//             "modified_date"=>self.modified_date)
//     }
// }

impl Default for UserInfo {
    fn default () -> UserInfo{ 
        UserInfo {
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
