use chrono::Duration;
use chrono::prelude::*;
use scylla::transport::errors::QueryError;
use chrono::Utc;
use std::sync::Arc;

use anyhow::Result;
use scylla::frame::value::Timestamp;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::data::{
    db::DbUserInfo, 
    request::{
        auth::UserLoginRequest, 
        user::create_user_request::CreateUserRequest
    }
};

const KS_NAME: &str = "user_data";
const USER_TAB_NAME: &str = "user_info";
const USER_CREDS_TAB_NAME: &str = "user_credentials";

const USER_ID: &str = "userid";
const USER_EMAIL: &str = "email";
const USER_PASSWORD_HASH: &str = "password";
const USER_IS_ACTIVE: &str = "active";


pub async fn insert_user(session: &Arc<Session>, user: &CreateUserRequest) -> Result<uuid::Uuid, QueryError> {
    let userid = uuid::Uuid::new_v4();

    let insert_user_info_cql: String = format!("INSERT INTO {}.{} 
    (userid, email, firstname, lastname, created_date, modified_date, active) VALUES (?, ?, ?, ?, ?, ?, ?)",KS_NAME,USER_TAB_NAME);

    let insert_user_creds_cql: String = format!("INSERT INTO {}.{} 
    (userid, email, password, active) VALUES (?, ?, ?, ?)",KS_NAME,USER_CREDS_TAB_NAME);

    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());

    session
        .query(insert_user_info_cql, (userid, &user.email, &user.firstname, &user.lastname,Timestamp(created),Timestamp(modified),false))
        .await?;
    session
        .query(insert_user_creds_cql, (userid, &user.email, &user.password_hash, false))
        .await?;

    Ok(userid)
}

pub async fn user_login(session: &Arc<Session>, login_request: &UserLoginRequest)-> Result<uuid::Uuid,String> {  
    let mut email = String::new();
    let mut pass = String::new();
    match &login_request.email {
        Some(e) => {email=e.to_string();}
        _ => return Ok(uuid::Uuid::default()),
    }
    match &login_request.password_hash {
        Some(p) => {pass=p.to_string();}
        _ => return Ok(uuid::Uuid::default()),
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

pub async fn create_all(session: &Arc<Session>) -> Result<()> {
    create_user_data_keyspace(session).await?;
    create_user_credentials_table(session).await?;
    create_user_info_table(session).await?;
    Ok(())
}

async fn create_user_data_keyspace(session: &Arc<Session>) -> Result<()> {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS user_data WITH replication = {'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3};";
    session
        .query(create_ks,&[])
        .await?;
    Ok(())
}

async fn create_user_credentials_table(session: &Arc<Session>) -> Result<()> {
    //"CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text)
    let create_table_cql =
        //"ALTER TABLE user_data.user_credentials ADD active boolean";
        "CREATE TABLE IF NOT EXISTS user_data.user_credentials ( email text PRIMARY KEY, userid UUID, password text, active boolean)";
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_user_info_table(session: &Arc<Session>) -> Result<()> {
    // "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)
    let create_table_cql =
        //"ALTER TABLE user_data.user_info  
        "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)";
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}