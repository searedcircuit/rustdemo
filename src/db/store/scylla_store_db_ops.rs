use const_format::formatcp;
use chrono::prelude::*;
use chrono::{Utc,Duration};
use tokio::try_join;
use scylla::transport::errors::QueryError;
use std::sync::Arc;

use scylla::frame::value::Timestamp;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::db::common::{
    CREATED_DATE, 
    MODIFIED_DATE, 
    USER_ACTIVATION_INSERT, 
    USER_CREDS_INSERT, 
    USER_INFO_TAB_NAME, 
    USER_EMAIL_MAP_TAB_NAME,
    USER_CREDS_TAB_NAME, 
    USER_EMAIL,
    USER_FIRSTNAME,
    USER_LASTNAME, 
    USER_PASSWORD_HASH, 
    USER_EMAIL_MAP_INSERT, 
    USER_ID, 
    USER_INFO_INSERT, 
    USER_IS_ACTIVE, 
    USER_KS_NAME};
use crate::data::{    
    response::store::get_store_response::StoreResponse,
    request::store::create_store_request::CreateStoreRequest,    
};

pub async fn insert_store(session: &Arc<Session>, user: &CreateStoreRequest) -> Result<uuid::Uuid, QueryError> {
    let userid = uuid::Uuid::new_v4();
    let activation_code = uuid::Uuid::new_v4();

    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());

    // insert user info
    // insert user creds
    // insert user email map
    // insert user activation code

    let info_future = session
        .query(USER_INFO_INSERT, (userid, &user.email, &user.firstname, &user.lastname, false, Timestamp(created), Timestamp(modified)));
    let creds_future = session
        .query(USER_CREDS_INSERT, (userid, &user.email, &user.password_hash, false));
    let email_map_future = session
        .query(USER_EMAIL_MAP_INSERT, (&user.email, userid));        
    let activation_future = session
        .query(USER_ACTIVATION_INSERT, (activation_code, userid));        

    try_join!(info_future,creds_future,email_map_future,activation_future)?;

    Ok(activation_code)
}

pub async fn select_stores(session: &Arc<Session>, userlat: f64,userlng: f64)-> Result<Vec<StoreResponse>,QueryError> {  
    let select_store_struct_cql: &str = formatcp!("SELECT {USER_ID}, {USER_EMAIL}, {USER_FIRSTNAME}, {USER_LASTNAME}, {CREATED_DATE}, {MODIFIED_DATE}, {USER_IS_ACTIVE} FROM {USER_KS_NAME}.{USER_INFO_TAB_NAME} WHERE userid = ? LIMIT 1");  
    
    let mut stores: Vec<StoreResponse> = Vec::new();
    if let Some(rows) = session.query(select_store_struct_cql,(userid,)).await?.rows {
        for row in rows.into_typed::<(uuid::Uuid, String,String,String, Duration, Duration, bool)>() {
            if let Ok(r) = row {
                    my_row.userid = r.0;
                    my_row.email = Some(r.1);
                    my_row.firstname = Some(r.2);
                    my_row.lastname = Some(r.3);
                    my_row.created_date = Utc.timestamp(r.4.num_seconds(),0);
                    my_row.modified_date = Utc.timestamp(r.5.num_seconds(),0);
                    my_row.active = r.6;
                }
            }
        }
    Ok(my_row)
}