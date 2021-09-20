use scylla::transport::errors::QueryError;
use std::time::{Duration as TimeDuration};
use const_format::formatcp;
use scylla::speculative_execution::PercentileSpeculativeExecutionPolicy;
use scylla::load_balancing::RoundRobinPolicy;
use scylla::load_balancing::TokenAwarePolicy;
use scylla::transport::errors::NewSessionError;
use std::sync::Arc;

use scylla::transport::session::Session;
use scylla::SessionBuilder;

pub const CREATED_DATE: &str = "created_date";
pub const MODIFIED_DATE: &str = "modified_date";

// user
pub const USER_KS_NAME: &str = "user_data";
pub const USER_INFO_TAB_NAME: &str = "user_info";
pub const USER_CREDS_TAB_NAME: &str = "user_credentials";
pub const USER_EMAIL_MAP_TAB_NAME: &str = "email_userid_map";
pub const USER_ACTIVATION_TAB_NAME: &str = "user_activation";

pub const USER_ID: &str = "userid";
pub const USER_EMAIL: &str = "email";
pub const USER_PASSWORD_HASH: &str = "password_hash";
pub const USER_FIRSTNAME: &str = "firstname";
pub const USER_LASTNAME: &str = "lastname";
pub const USER_IS_ACTIVE: &str = "active";

pub const USER_ACTIVATION_CODE: &str = "activation_code";
pub const USER_ACTIVATION_TTL: i32 = 86400;

pub const USER_INFO_INSERT: &str = 
    formatcp!("INSERT INTO {USER_KS_NAME}.{USER_INFO_TAB_NAME} 
    ({USER_ID},{USER_EMAIL},{USER_FIRSTNAME},{USER_LASTNAME},{USER_IS_ACTIVE},{CREATED_DATE},{MODIFIED_DATE}) 
    VALUES (?, ?, ?, ?, ?, ?, ?)");

pub const USER_CREDS_INSERT: &str = 
    formatcp!("INSERT INTO {USER_KS_NAME}.{USER_CREDS_TAB_NAME} 
    ({USER_ID}, {USER_EMAIL}, {USER_PASSWORD_HASH}, {USER_IS_ACTIVE}) 
    VALUES (?, ?, ?, ?)");

pub const USER_EMAIL_MAP_INSERT: &str = 
    formatcp!("INSERT INTO {USER_KS_NAME}.{USER_EMAIL_MAP_TAB_NAME} 
    ({USER_EMAIL}, {USER_ID}) 
    VALUES (?, ?)");

pub const USER_ACTIVATION_INSERT: &str = 
    formatcp!("INSERT INTO {USER_KS_NAME}.{USER_ACTIVATION_TAB_NAME} 
    ({USER_ACTIVATION_CODE}, {USER_ID}) 
    VALUES (?, ?) USING TTL 86400");    

// auth
pub const AUTH_KS_NAME: &str = "session_auth_ks";

pub const AUTH_CODE_TAB_NAME: &str = "auth_code";
pub const AUTH_REFRESH_TAB_NAME: &str = "refresh_code";

pub const AUTH_CODE: &str = "auth_code";
pub const REFRESH_CODE: &str = "refresh_code";

pub const AUTH_CODE_TTL: i32 = 3600;
pub const AUTH_REFRESH_TTL: i32 = 2_592_000;

pub const AUTH_CODE_INSERT: &str = 
    formatcp!("INSERT INTO {AUTH_KS_NAME}.{AUTH_CODE_TAB_NAME} ({AUTH_CODE}, {USER_ID}) VALUES (?, ?)");

pub const AUTH_REFRESH_CODE_INSERT: &str = 
    formatcp!("INSERT INTO {AUTH_KS_NAME}.{AUTH_REFRESH_TAB_NAME} ({REFRESH_CODE}, {USER_ID}) VALUES (?, ?)");

const ADDRESS: &str = "localhost:9042";

pub async fn create_session() -> Result<Arc<Session>,NewSessionError> {
    //let uri = env::var("cassandra:9042").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    println!("Connecting to {} ...", ADDRESS);

    let robin = Box::new(RoundRobinPolicy::new());
    //let dc_robin = Box::new(DcAwareRoundRobinPolicy::new(local_dc));
    let load_balance_policy = Arc::new(TokenAwarePolicy::new(robin));

    let execution_policy = PercentileSpeculativeExecutionPolicy  {
        max_retry_count: 3,
        percentile: 99.0,
    };

    let session: Session = SessionBuilder::new()
        .known_node(ADDRESS)
        // .known_node("cassandra-0.cassandra.cass.svc.cluster.local")
        // .known_node("cassandra-1.cassandra.cass.svc.cluster.local")
        // .known_node("cassandra-2.cassandra.cass.svc.cluster.local")
        //.known_node("localhost")
        .connection_timeout(TimeDuration::from_secs(3))
        .load_balancing(load_balance_policy)
        .speculative_execution(Arc::new(execution_policy))
        .build().await?;
    Ok(Arc::new(session))
}

pub async fn create_tables(session: &Arc<Session>) -> Result<(),QueryError> {
    create_user_ks(session).await?;
    create_auth_ks(session).await?;

    create_user_info(session).await?;
    create_user_creds(session).await?;
    create_email_map(session).await?;
    create_activation(session).await?;
    create_auth_code(session).await?;
    create_refresh_code(session).await?;
    
    Ok(())
}

async fn create_user_ks(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_user_ks: &str = formatcp!("CREATE KEYSPACE IF NOT EXISTS {USER_KS_NAME} WITH replication = {{'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3}};");
    session
        .query(create_user_ks,&[])
        .await?;
    Ok(())
}

async fn create_auth_ks(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_auth_ks: &str = formatcp!("CREATE KEYSPACE IF NOT EXISTS {AUTH_KS_NAME} WITH replication = {{'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3}};");
    session
        .query(create_auth_ks,&[])
        .await?;
    Ok(())
}

async fn create_user_info(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_user_info_table_cql = 
        formatcp!(
        "CREATE TABLE IF NOT EXISTS {USER_KS_NAME}.{USER_INFO_TAB_NAME} 
        ( 
            {USER_ID} UUID PRIMARY KEY, 
            {USER_EMAIL} text, 
            {USER_FIRSTNAME} text, 
            {USER_LASTNAME} text, 
            {USER_IS_ACTIVE} boolean,
            {CREATED_DATE} timestamp,
            {MODIFIED_DATE} timestamp
        )");

    session
        .query(create_user_info_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_user_creds(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_user_creds_table_cql = formatcp!("CREATE TABLE IF NOT EXISTS {USER_KS_NAME}.{USER_CREDS_TAB_NAME} 
        ( 
            {USER_ID} UUID PRIMARY KEY, 
            {USER_EMAIL} text, 
            {USER_PASSWORD_HASH} text, 
            {USER_IS_ACTIVE} boolean
        )");

    session
        .query(create_user_creds_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_email_map(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_user_email_map_table_cql = formatcp!("CREATE TABLE IF NOT EXISTS {USER_KS_NAME}.{USER_EMAIL_MAP_TAB_NAME} 
        ( 
            {USER_EMAIL} text PRIMARY KEY, 
            {USER_ID} UUID
        )");

    session
        .query(create_user_email_map_table_cql,&[])
        .await?;
    Ok(())
}

async fn create_activation(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_user_activation_table_cql = formatcp!("CREATE TABLE IF NOT EXISTS {USER_KS_NAME}.{USER_ACTIVATION_TAB_NAME} 
        ( 
            {USER_ACTIVATION_CODE} UUID PRIMARY KEY, 
            {USER_ID} UUID
        )");

    session
        .query(create_user_activation_table_cql,&[])
        .await?;
    Ok(())    
}

async fn create_auth_code(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_auth_code_table_cql = formatcp!("CREATE TABLE IF NOT EXISTS {AUTH_KS_NAME}.{AUTH_CODE_TAB_NAME} 
        ( 
            {AUTH_CODE} UUID PRIMARY KEY, 
            {USER_ID} UUID
        )");

    session
        .query(create_auth_code_table_cql,&[])
        .await?;
    Ok(())      
}

async fn create_refresh_code(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_auth_refresh_code_table_cql = formatcp!("CREATE TABLE IF NOT EXISTS {AUTH_KS_NAME}.{AUTH_REFRESH_TAB_NAME} 
        ( 
            {REFRESH_CODE} UUID PRIMARY KEY, 
            {USER_ID} UUID
        )");

    session
        .query(create_auth_refresh_code_table_cql,&[])
        .await?;
    Ok(())      
}