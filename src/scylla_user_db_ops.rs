use scylla::frame::value::Timestamp;
use chrono::Duration;
use chrono::prelude::*;
use scylla::transport::errors::QueryError;
use chrono::Utc;
use std::sync::Arc;

use anyhow::Result;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::user_info::UserInfo;

const KS_NAME: &str = "user_data";
const USER_TAB_NAME: &str = "user_info";

pub async fn insert_user(session: &Arc<Session>, email: &str) -> Result<uuid::Uuid, QueryError> {
    let userid = uuid::Uuid::new_v4();
    let row = UserInfo {
        userid: userid,
        email: Some(email.to_string()),
        firstname: Some("tom".to_string()),
        lastname: Some("bob".to_string()),
        created_date: Utc::now(),
        modified_date: Utc::now(),
        active:false
    };

    let insert_user_struct_cql: String = format!("INSERT INTO {}.{} \
    (userid, email, firstname, lastname, created_date, modified_date, active) VALUES (?, ?, ?, ?, ?, ?, ?)",KS_NAME,USER_TAB_NAME);

    let created = Duration::seconds(row.created_date.timestamp());
    let modified = Duration::seconds(row.modified_date.timestamp());

    let res = session
        .query(insert_user_struct_cql, (row.userid, row.email, row.firstname, row.lastname,Timestamp(created),Timestamp(modified),row.active))
        .await;
    match res {
        Ok(_) => Ok(userid),
        Err(e) => panic!("{}",e)
    }
}

pub async fn select_user(session: &Arc<Session>, userid: uuid::Uuid)-> Result<UserInfo,QueryError> {  
    let select_user_struct_cql: String = format!("SELECT userid, email, firstname, lastname, created_date, modified_date, active FROM {}.{} WHERE userid = {} LIMIT 1",KS_NAME,USER_TAB_NAME,userid);  
    
    let mut my_row: UserInfo = UserInfo::default();  
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
        //"ALTER TABLE user_data.user_credentials
        "CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text)";
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