use cdrs_tokio::cluster::ConnectionPool;
use chrono::Utc;
use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use std::sync::Arc;

use cdrs_tokio::cluster::session::{new as new_session, Session};
use cdrs_tokio::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder};

use cdrs_tokio::query::*;
use cdrs_tokio::retry::{DefaultRetryPolicy};

use cdrs_tokio::load_balancing::RoundRobin;
use cdrs_tokio::types::prelude::*;

use cdrs_tokio::transport::TransportTcp;
use crate::user_info::UserInfo;

const KS_NAME: &str = "user_data";
const USER_TAB_NAME: &str = "user_info";
type CurrentSession = Session<RoundRobin<ConnectionPool<TransportTcp>>>;

const USER:&str = "user";
const PASSWORD: &str = "password";
const ADDRESS: &str = "cassandra:9042";

pub async fn create_session() -> Arc<CurrentSession> {
    let auth:StaticPasswordAuthenticatorProvider = StaticPasswordAuthenticatorProvider::new(&USER, &PASSWORD);    

    let node = NodeTcpConfigBuilder::new(
        ADDRESS,
        Arc::new(auth),
    )
    .build();
    let cluster_config = ClusterTcpConfig(vec![node]);

    let no_compression = new_session(
        &cluster_config,
        RoundRobin::new(),
        Box::new(DefaultRetryPolicy::default()),
    )
    .await
    .expect("error creating connection pool");
    
    Arc::new(no_compression)
}

pub async fn insert_struct(session: &Arc<CurrentSession>, email: &str) -> uuid::Uuid {
    let userid = uuid::Uuid::new_v4();
    let row = UserInfo {
        userid: userid,
        email: email.to_string(),
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
    userid
}

pub async fn select_struct(session: &Arc<CurrentSession>, userid: uuid::Uuid)->UserInfo {  
    let select_user_struct_cql: String = format!("SELECT * FROM {}.{} WHERE userid = {} LIMIT 1",KS_NAME,USER_TAB_NAME,userid);  
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

pub async fn create_all(session: &Arc<CurrentSession>){
    create_keyspace(session).await;
    create_table(session).await;
    create_table2(session).await;
}

async fn create_keyspace(session: &Arc<CurrentSession>) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS user_data WITH replication = {'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3};";
    session
        .query(create_ks)
        .await
        .expect("Keyspace creation error");
}

async fn create_table(session: &Arc<CurrentSession>) {
    //"CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text)
    let create_table_cql =
        //"ALTER TABLE user_data.user_credentials
        "CREATE TABLE IF NOT EXISTS user_data.user_credentials ( userid UUID PRIMARY KEY, password text, email text)
            WITH bloom_filter_fp_chance = 0.01
            AND caching = {'keys': 'NONE', 'rows_per_partition': '100'}
            AND comment = ''
            AND compaction = {'class': 'org.apache.cassandra.db.compaction.SizeTieredCompactionStrategy', 'max_threshold': '32', 'min_threshold': '4'}
            AND compression = {'chunk_length_in_kb': '64', 'class': 'org.apache.cassandra.io.compress.LZ4Compressor'}
            AND crc_check_chance = 1.0
            AND default_time_to_live = 0
            AND gc_grace_seconds = 864000
            AND max_index_interval = 2048
            AND memtable_flush_period_in_ms = 0
            AND min_index_interval = 128
            AND speculative_retry = '99percentile'";
    session
        .query(create_table_cql)
        .await
        .expect("Table creation error");
}

async fn create_table2(session: &Arc<CurrentSession>) {
    // "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)
    let create_table_cql =
        //"ALTER TABLE user_data.user_info  
        "CREATE TABLE IF NOT EXISTS user_data.user_info ( userid UUID PRIMARY KEY, lastname text, firstname text, email text, created_date timestamp, modified_date timestamp, active boolean)      
            WITH bloom_filter_fp_chance = 0.01
            AND caching = {'keys': 'NONE', 'rows_per_partition': '100'}
            AND comment = ''
            AND compaction = {'class': 'org.apache.cassandra.db.compaction.SizeTieredCompactionStrategy', 'max_threshold': '32', 'min_threshold': '4'}
            AND compression = {'chunk_length_in_kb': '64', 'class': 'org.apache.cassandra.io.compress.LZ4Compressor'}
            AND crc_check_chance = 1.0            
            AND default_time_to_live = 0
            AND gc_grace_seconds = 864000
            AND max_index_interval = 2048
            AND memtable_flush_period_in_ms = 0
            AND min_index_interval = 128
            AND speculative_retry = '99percentile'";
    session
        .query(create_table_cql)
        .await
        .expect("Table creation error");
}