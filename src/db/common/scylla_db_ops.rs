use futures::try_join;
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

// store
pub const STORE_KS_NAME: &str = "store_ks";

pub const STORE_INFO_TAB_NAME: &str = "store_info";
pub const STORE_LOC_MAP_TAB_NAME: &str = "store_loc_map";

pub const STORE_ID: &str = "store_id";
pub const PLACE_ID: &str = "place_id";
pub const STORE_NAME: &str = "store_name";
pub const STORE_DESCRIPTION: &str = "store_desc";
pub const FORMATTED_ADDRESS: &str = "formatted_addr";
pub const LATITUDE: &str = "lat";
pub const LONGITUDE: &str = "lng";
pub const SHORT_LAT: &str = "slat";
pub const SHORT_LNG: &str = "slng";
pub const STORE_IS_ACTIVE: &str = "active";

pub const STORE_INFO_INSERT: &str = 
    formatcp!("INSERT INTO 
        {STORE_KS_NAME}.{STORE_INFO_TAB_NAME} 
        (
            {STORE_ID},
            {PLACE_ID},
            {STORE_NAME},
            {STORE_DESCRIPTION},
            {FORMATTED_ADDRESS},
            {LATITUDE},
            {LONGITUDE},
            {STORE_IS_ACTIVE},
            {CREATED_DATE},
            {MODIFIED_DATE}
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

pub const STORE_INFO_SELECT: &str = 
    formatcp!("
        SELECT         
            {STORE_ID}, 
            {PLACE_ID}, 
            {STORE_NAME}, 
            {STORE_DESCRIPTION}, 
            {FORMATTED_ADDRESS}, 
            {LATITUDE}, 
            {LONGITUDE},
            {STORE_IS_ACTIVE} 
        FROM 
            {STORE_KS_NAME}.{STORE_INFO_TAB_NAME}         
        WHERE 
            {STORE_ID} IN (?)
        LIMIT 20;");

pub const STORE_LOC_MAP_INSERT: &str = 
    formatcp!("INSERT INTO 
        {STORE_KS_NAME}.{STORE_LOC_MAP_TAB_NAME} 
        (
            {STORE_ID},
            {PLACE_ID},
            {LATITUDE},
            {LONGITUDE},
            {SHORT_LAT},            
            {SHORT_LNG},
            {CREATED_DATE},
            {MODIFIED_DATE}
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)");      

pub const STORE_LOC_MAP_SELECT: &str = 
    formatcp!("
        SELECT
            {STORE_ID},
            {PLACE_ID},
            {LATITUDE},
            {LONGITUDE},
            {SHORT_LAT},            
            {SHORT_LNG},
            {CREATED_DATE},
            {MODIFIED_DATE}
        FROM            
            {STORE_KS_NAME}.{STORE_LOC_MAP_TAB_NAME} 
        WHERE
            {SHORT_LAT} IN (?,?,?) 
        AND 
            {SHORT_LNG} IN (?,?,?) 
        AND 
            ({LATITUDE},{LONGITUDE}) > (?,?)    
        AND 
            ({LATITUDE},{LONGITUDE}) < (?,?)
        LIMIT 20;");  
        
// store menu        
pub const MENU_ITEM_TAB_NAME:&str = "menu_item";

pub const ITEM_ID: &str = "item_id";
pub const ITEM_NAME: &str = "item_name";
pub const ITEM_DESC: &str = "item_desc";
pub const ITEM_SIZE: &str = "item_size";
pub const ITEM_TEMP: &str = "item_temp";
pub const ITEM_COST: &str = "item_cost";

pub const MENU_ITEM_INSERT: &str = 
    formatcp!("INSERT INTO 
        {STORE_KS_NAME}.{MENU_ITEM_TAB_NAME} 
        (
            {STORE_ID},
            {ITEM_ID},
            {ITEM_NAME},
            {ITEM_DESC},
            {ITEM_SIZE},
            {ITEM_TEMP},
            {ITEM_COST},   
            {CREATED_DATE},
            {MODIFIED_DATE}
        ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)");

pub const MENU_ITEM_SELECT: &str =
    formatcp!("
        SELECT 
            {STORE_ID},
            {ITEM_ID},
            {ITEM_NAME},
            {ITEM_DESC},
            {ITEM_SIZE},
            {ITEM_TEMP},
            {ITEM_COST},   
            {CREATED_DATE},
            {MODIFIED_DATE}
        FROM
            {STORE_KS_NAME}.{MENU_ITEM_TAB_NAME} 
        WHERE
            {STORE_ID} = ?");

pub const MENU_OPTION_TAB_NAME:&str = "menu_option";

pub const OPTION_ID: &str = "option_id";
pub const OPTION_NAME: &str = "option_name";
pub const OPTION_COST: &str = "option_cost";

pub const MENU_OPTION_INSERT: &str = 
    formatcp!("INSERT INTO 
        {STORE_KS_NAME}.{MENU_OPTION_TAB_NAME} 
        (
            {STORE_ID},
            {OPTION_ID},

            {OPTION_NAME},   
            {OPTION_COST},
            {CREATED_DATE},
            {MODIFIED_DATE}
        ) 
        VALUES (?, ?, ?, ?, ?, ?)");

pub const MENU_OPTION_SELECT: &str =
    formatcp!("
        SELECT 
            {STORE_ID},
            {OPTION_ID},

            {OPTION_NAME},   
            {OPTION_COST},
            {CREATED_DATE},
            {MODIFIED_DATE}
        FROM
            {STORE_KS_NAME}.{MENU_OPTION_TAB_NAME} 
        WHERE
            {STORE_ID} = ?");

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
    let c1 = create_user_ks(session);
    let c2 = create_auth_ks(session);
    let c3 = create_store_ks(session);

    let c4 = create_user_info(session);
    let c5 = create_user_creds(session);
    let c6 = create_email_map(session);

    let c7 = create_activation(session);
    let c8 = create_auth_code(session);
    let c9 = "victory!";
    let c10 = create_refresh_code(session);
    
    let c11 = create_store(session);
    let c12 = create_store_loc_map(session);

    let c13 = create_menu_item(session);
    let c14 = create_menu_option(session);
    
    try_join!(c1,c2,c3,c4,c5,c6,c7,c8, c10, c11, c12, c13, c14)?;    

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

async fn create_store_ks(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_store_ks: &str = formatcp!("CREATE KEYSPACE IF NOT EXISTS {STORE_KS_NAME} WITH replication = {{'class':'NetworkTopologyStrategy','datacenter1':1, 'replication_factor' : 3}};");
    session
        .query(create_store_ks,&[])
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

async fn create_store(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_store_info_table_cql = 
        formatcp!(
        "CREATE TABLE IF NOT EXISTS {STORE_KS_NAME}.{STORE_INFO_TAB_NAME}
        (
            {STORE_ID} UUID,
            {PLACE_ID} text,
            {STORE_NAME} text,
            {STORE_DESCRIPTION} text,
            {FORMATTED_ADDRESS} text,
            {LATITUDE} double,
            {LONGITUDE} double,
            {CREATED_DATE} timestamp,
            {MODIFIED_DATE} timestamp,
            {STORE_IS_ACTIVE} boolean,
            PRIMARY KEY ({STORE_ID})
        );");

    session
        .query(create_store_info_table_cql,&[]).await?;
    
    Ok(())
}

async fn create_menu_item(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_menu_item_table_cql = 
        formatcp!(
        "CREATE TABLE IF NOT EXISTS {STORE_KS_NAME}.{MENU_ITEM_TAB_NAME}
        (
            {STORE_ID} UUID PRIMARY KEY,
            {ITEM_ID} UUID,
            {ITEM_NAME} text,
            {ITEM_DESC} text,
            {ITEM_SIZE} text,
            {ITEM_TEMP} text,
            {ITEM_COST} int,            
            {CREATED_DATE} timestamp,
            {MODIFIED_DATE} timestamp
        );");

    session
        .query(create_menu_item_table_cql,&[]).await?;
    
    Ok(())
}

async fn create_menu_option(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_menu_option_table_cql = 
        formatcp!(
        "CREATE TABLE IF NOT EXISTS {STORE_KS_NAME}.{MENU_OPTION_TAB_NAME}
        (
            {STORE_ID} UUID PRIMARY KEY,
            {OPTION_ID} UUID,

            {OPTION_NAME} text,   
            {OPTION_COST} int,        
            {CREATED_DATE} timestamp,
            {MODIFIED_DATE} timestamp
        );");

    session
        .query(create_menu_option_table_cql,&[]).await?;
    
    Ok(())
}

async fn create_store_loc_map(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_store_info_table_cql = 
        formatcp!(
        "CREATE TABLE IF NOT EXISTS {STORE_KS_NAME}.{STORE_LOC_MAP_TAB_NAME}
        (
            {SHORT_LAT} smallint,
            {SHORT_LNG} smallint,
            {STORE_ID} UUID,
            {PLACE_ID} text,
            {LATITUDE} double,
            {LONGITUDE} double,
            {CREATED_DATE} timestamp,
            {MODIFIED_DATE} timestamp,
            PRIMARY KEY (({SHORT_LAT}, {SHORT_LNG}), {LATITUDE}, {LONGITUDE})
        ) WITH CLUSTERING ORDER BY ({LATITUDE} ASC, {LONGITUDE} ASC);");

    session
        .query(create_store_info_table_cql,&[]).await?;
    
    Ok(())
}