use chrono::Utc;
use std::sync::Arc;
use cdrs_tokio::cluster::NodeTcpConfig;
use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::query::*;

use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use cdrs_tokio::cluster::session::{new as new_session, Session};
use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs_tokio::retry::DefaultRetryPolicy;
use cdrs_tokio::types::prelude::*;
use crate::user_info::UserInfo;


const KS_NAME: &str = "user_data";
const USER_TAB_NAME: &str = "user_info";
type CurrentSession = Session<RoundRobin<TcpConnectionPool>>;

const USER:&str = "user";
const PASSWORD: &str = "password";

pub async fn create_session() -> Session<RoundRobin<TcpConnectionPool>> {
    let auth:StaticPasswordAuthenticatorProvider = StaticPasswordAuthenticatorProvider::new(&USER, &PASSWORD);
    let node:NodeTcpConfig = NodeTcpConfigBuilder::new("host.docker.internal:9042", Arc::new(auth)).build();
    let cluster_config:ClusterTcpConfig = ClusterTcpConfig(vec![node]);
    new_session(
        &cluster_config,
        RoundRobin::new(),
        Box::new(DefaultRetryPolicy::default()),
    )
    .await
    .expect("session should be created")
}

pub async fn insert_struct(session: &mut CurrentSession) {
    let row = UserInfo {
        userid: uuid::Uuid::new_v4(),
        email: String::from("tom@gmail.com"),
        firstname: String::from("tom"),
        lastname: String::from("bob"),
        created_date: Utc::now(),
        modified_date: Utc::now(),
        active:false
    };
    let insert_user_struct_cql: String = format!("INSERT INTO {}.{} \
    (userid, email, firstname, lastname, created_date, modified_date, active) VALUES (?, ?, ?, ?, ?, ?, ?)",KS_NAME,USER_TAB_NAME);
    session
        .query_with_values(&insert_user_struct_cql, row.into_query_values())
        .await
        .expect("insert");
}

pub async fn select_struct(session: &mut CurrentSession)->UserInfo {  
    let select_user_struct_cql: String = format!("SELECT * FROM {}.{}",KS_NAME,USER_TAB_NAME);  
    let rows = session
        .query(&select_user_struct_cql)
        .await
        .expect("query")
        .body()
        .expect("get body")
        .into_rows()
        .expect("into rows");
    let mut my_row: UserInfo = UserInfo::default();
    for row in rows {
        my_row = UserInfo::try_from_row(row).expect("into UserInfo");
        //println!("struct got: {:#?}", my_row);
    }
    my_row
}

pub async fn create_all(){
    let mut no_compression: CurrentSession = create_session().await;
    create_keyspace(&mut no_compression).await;
    create_table(&mut no_compression).await;
    create_table2(&mut no_compression).await;
}

async fn create_keyspace(session: &mut CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS user_data WITH replication = {'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3};";
    session
        .query(create_ks)
        .await
        .expect("Keyspace creation error");
}

async fn create_table(session: &mut CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text);";
    session
        .query(create_table_cql)
        .await
        .expect("Table creation error");
}

async fn create_table2(session: &mut CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean);";
    session
        .query(create_table_cql)
        .await
        .expect("Table creation error");
}