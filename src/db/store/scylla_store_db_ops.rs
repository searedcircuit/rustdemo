use scylla::frame::value::Timestamp;
use chrono::Duration;
use chrono::prelude::*;
use scylla::transport::errors::QueryError;
use chrono::Utc;
use std::sync::Arc;

use anyhow::Result;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::data::{
    db::db_user_info::DbUserInfo, 
    request::store::create_store_user_request::CreateStoreUserRequest
};

const STORE_KS_NAME: &str = "store_data";
const STORE_USER_CREDS_TAB_NAME: &str = "store_user_credentials";
const STORE_USER_TAB_NAME: &str = "store_user_info";
const STORE_TAB_NAME: &str = "store_info";

pub async fn insert_store_user(session: &Arc<Session>, store_user: &mut CreateStoreUserRequest) -> Result<uuid::Uuid, QueryError> {
    let insert_user_struct_cql: String = format!("INSERT INTO {}.{} \
    (store_userid, email, firstname, lastname, created_date, modified_date, active) VALUES (?, ?, ?, ?, ?, ?, ?)",STORE_KS_NAME,STORE_USER_TAB_NAME);
    let userid = uuid::Uuid::new_v4();

    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());

    let res = session
        .query(insert_user_struct_cql, (userid, &store_user.email, &store_user.firstname, &store_user.lastname,Timestamp(created),Timestamp(modified),false))
        .await;
    match res {
        Ok(_) => Ok(userid),
        Err(e) => panic!("{}",e)
    }
}

pub async fn select_store_user(session: &Arc<Session>, userid: uuid::Uuid)-> Result<DbUserInfo,QueryError> {  
    let select_user_struct_cql: String = format!("SELECT store_userid, email, firstname, lastname, created_date, modified_date, active FROM {}.{} WHERE store_userid = {} LIMIT 1",STORE_KS_NAME,STORE_USER_TAB_NAME,userid);
    
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
    create_store_data_keyspace(session).await?;
    create_store_user_credentials_table(session).await?;
    create_store_user_info_table(session).await?;
    create_store_info_table(session).await?;
    Ok(())
}

async fn create_store_data_keyspace(session: &Arc<Session>) -> Result<()> {
    let create_ks = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH replication = {{'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3}};",STORE_KS_NAME);
    session
        .query(create_ks,&[])
        .await?;
    Ok(())
}

async fn create_store_user_credentials_table(session: &Arc<Session>) -> Result<()> {
    //"CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text)
    let create_table_cql =
        //"ALTER TABLE user_data.user_credentials
        format!("CREATE TABLE IF NOT EXISTS {}.{} 
            ( 
                store_userid UUID PRIMARY KEY, 
                password text, 
                email text
            )",STORE_KS_NAME,STORE_USER_CREDS_TAB_NAME);
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_store_user_info_table(session: &Arc<Session>) -> Result<()> {
    // "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)
    let create_table_cql =
        //"ALTER TABLE user_data.user_info  
        format!("CREATE TABLE IF NOT EXISTS {}.{} 
            ( 
                store_userid UUID PRIMARY KEY, 
                firstname text, 
                lastname text, 
                email text, 
                created_date timestamp, 
                modified_date timestamp, 
                active boolean
            )",STORE_KS_NAME,STORE_USER_TAB_NAME);
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_store_info_table(session: &Arc<Session>) -> Result<()> {
    // "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)
    let create_table_cql =
        //"ALTER TABLE user_data.user_info  
        format!("CREATE TABLE IF NOT EXISTS {}.{} 
            ( 
                store_userid UUID PRIMARY KEY, 
                storeid UUID,
                storename text, 
                lat double,
                lng double,
                created_date timestamp, 
                modified_date timestamp, 
                active boolean
            )",
            STORE_KS_NAME,
            STORE_TAB_NAME);
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}