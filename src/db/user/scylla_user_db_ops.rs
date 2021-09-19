use chrono::prelude::*;
use chrono::{Utc,Duration};
use tokio::try_join;
use scylla::transport::errors::QueryError;
use std::sync::Arc;

use scylla::frame::value::Timestamp;
use scylla::transport::session::{IntoTypedRows, Session};

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
        .query(insert_user_info_cql, (userid, &user.email, &user.firstname, &user.lastname,Timestamp(created),Timestamp(modified),false));
    let creds_future = session
        .query(insert_user_creds_cql, (userid, &user.email, &user.password_hash, false));
    let activation_future = session
        .query(insert_activation_code_cql, (activation_code, userid, &user.email));        

    try_join!(info_future,creds_future,activation_future)?;

    Ok(activation_code)
}

pub async fn user_login(session: &Arc<Session>, login_request: &UserLoginRequest)-> Result<uuid::Uuid,String> {  
    let mut email = String::new();
    let mut pass = String::new();
    match &login_request.email {
        Some(e) => email=e.to_string(),
        _ => return Err("email is required".to_string()),
    }
    match &login_request.password_hash {
        Some(p) => pass=p.to_string(),
        _ => return Err("password is required".to_string()),
    }
    let select_user_login_cql: String = format!("SELECT userid, password, active FROM {}.{} WHERE {} = '{}' LIMIT 1",KS_NAME,USER_CREDS_TAB_NAME,USER_EMAIL,email);

    let mut userid = uuid::Uuid::default();
    if let Some(rows) = session.query(select_user_login_cql,&[]).await
    .or_else(|q| Err(format!("query error: {}",q)))?.rows {
        for row in rows.into_typed::<(uuid::Uuid,String,bool)>() {
            let r = row.or_else(|e|Err(format!("invalid login: {}",e)))?;
            userid = r.0;
            let password = r.1;
            let active = r.2;
            if !active{return Err("account is not activated".to_string())}
            if !password.eq(&pass){return Err("invalid login".to_string())}
        }
    }
    Ok(userid)
}

pub async fn select_user(session: &Arc<Session>, userid: uuid::Uuid)-> Result<DbUserInfo,QueryError> {  
    let select_user_struct_cql: String = format!("SELECT userid, email, firstname, lastname, created_date, modified_date, active FROM {}.{} WHERE userid = {} LIMIT 1",KS_NAME,USER_TAB_NAME,userid);  
    
    let mut my_row: DbUserInfo = DbUserInfo::default();  
    if let Some(rows) = session.query(select_user_struct_cql,&[]).await?.rows {
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