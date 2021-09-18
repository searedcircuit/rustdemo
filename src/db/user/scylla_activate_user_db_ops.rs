use scylla::transport::errors::QueryError;
use uuid::Uuid;
use chrono::Duration;
use chrono::Utc;
use std::sync::Arc;

use scylla::transport::session::{IntoTypedRows, Session};

const KS_NAME: &str = "user_data";
const ACTIVATION_TAB_NAME: &str = "activation";
const USER_TAB_NAME: &str = "user_info";
const USER_CREDS_TAB_NAME: &str = "user_credentials";

const ACTIVATION_CODE: &str = "activation_code";
const USER_ID: &str = "userid";
const USER_IS_ACTIVE: &str = "active";

const MODIFIED_DATE: &str = "modified_date";

pub async fn activate_user(session: &Arc<Session>, activation_code: &Uuid) -> Result<(),Box<dyn std::error::Error>> {
    let get_userid_cql: String = format!("SELECT {} FROM {}.{} WHERE {} = '{}' LIMIT 1",
        USER_ID,
        KS_NAME,ACTIVATION_TAB_NAME,
        ACTIVATION_CODE,activation_code);

    let modified = Duration::seconds(Utc::now().timestamp());

    let mut userid = uuid::Uuid::default();
    if let Some(rows) = session.query(get_userid_cql,&[]).await?.rows {
        for row in rows.into_typed::<(uuid::Uuid,)>() {
            match row {
                Ok(r) => userid=r.0,
                Err(e) => {
                    // log e
                    return Err("activation unsuccessful, please refresh activation code & try again".into())
                }
            }
        }
    }

    let update_user_creds_cql: String = format!("UPDATE {}.{} SET {} = {} AND {} = {} WHERE {} = {}",
        KS_NAME,USER_TAB_NAME,
        USER_IS_ACTIVE,true,
        USER_ID,userid,
        MODIFIED_DATE,modified);

    session.query(update_user_creds_cql, &[]).await?; // ?.map .map_err(|_| "error updating user, please try activating again");

    Ok(())
}

async fn create_user_activation_table(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_table_cql = format!("CREATE TABLE IF NOT EXISTS {}.{} ( {} UUID PRIMARY KEY, {} UUID)",KS_NAME,ACTIVATION_TAB_NAME,ACTIVATION_CODE,USER_ID);
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}