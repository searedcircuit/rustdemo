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
    db::DbUserInfo, 
    request::{
        auth::UserLoginRequest, 
        user::create_user_request::CreateUserRequest
    }
};

pub async fn insert_user(session: &Arc<Session>, user: &CreateUserRequest) -> Result<uuid::Uuid, QueryError> {
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

pub async fn user_login(session: &Arc<Session>, login_request: &UserLoginRequest)-> Result<uuid::Uuid,Box<dyn std::error::Error>> {  
    let mut email = String::new();
    let mut pass = String::new();
    match &login_request.email {
        Some(e) => email=e.to_string(),
        _ => return Err("email is required".into()),
    }
    match &login_request.password_hash {
        Some(p) => pass=p.to_string(),
        _ => return Err("password is required".into()),
    }

    let select_user_id_cql: &str = formatcp!("SELECT {USER_ID} FROM {USER_KS_NAME}.{USER_EMAIL_MAP_TAB_NAME} WHERE {USER_EMAIL} = ? LIMIT 1");
    let mut userid = uuid::Uuid::default();
    if let Some(rows) = session.query(select_user_id_cql,(email,)).await?.rows{
        for row in rows.into_typed::<(uuid::Uuid,)>() {
            match row {
                Ok(r) => {
                    userid=r.0;
                }
                Err(e) => {
                    // log e
                    return Err(format!("Error locating user info. Does this account exist? {}",e).into())
                }
            }
        }
    }

    let select_user_login_cql: &str = formatcp!("SELECT {USER_PASSWORD_HASH}, {USER_IS_ACTIVE} FROM {USER_KS_NAME}.{USER_CREDS_TAB_NAME} WHERE {USER_ID} = ? LIMIT 1");

    if let Some(rows) = session.query(select_user_login_cql,(userid,)).await?.rows{
        for row in rows.into_typed::<(String,bool)>() {
            match row {
                Ok(r) => {
                    let password = r.0;
                    let active = r.1;
                    if !active{return Err("account is not activated".into());}
                    if !password.eq(&pass){return Err("invalid login".into());}
                }
                Err(e) => {
                    // log e
                    return Err(format!("Invalid login: {}",e).into())
                }
            }
        }
    }

    Ok(userid)
}

pub async fn select_user(session: &Arc<Session>, userid: uuid::Uuid)-> Result<DbUserInfo,QueryError> {  
    let select_user_struct_cql: &str = formatcp!("SELECT {USER_ID}, {USER_EMAIL}, {USER_FIRSTNAME}, {USER_LASTNAME}, {CREATED_DATE}, {MODIFIED_DATE}, {USER_IS_ACTIVE} FROM {USER_KS_NAME}.{USER_INFO_TAB_NAME} WHERE userid = ? LIMIT 1");  
    
    let mut my_row: DbUserInfo = DbUserInfo::default();  
    if let Some(rows) = session.query(select_user_struct_cql,(userid,)).await?.rows {
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