use const_format::formatcp;
use futures::try_join;
use uuid::Uuid;
use chrono::Duration;
use chrono::Utc;
use std::sync::Arc;
use scylla::frame::value::Timestamp;

use scylla::transport::session::{IntoTypedRows, Session};

use crate::db::common::{MODIFIED_DATE, USER_KS_NAME,USER_INFO_TAB_NAME,USER_CREDS_TAB_NAME,USER_ACTIVATION_TAB_NAME, USER_ID, USER_EMAIL, USER_IS_ACTIVE, USER_ACTIVATION_CODE, USER_ACTIVATION_INSERT};

pub async fn insert_activation_code(session: &Arc<Session>, email: &String) -> Result<uuid::Uuid,Box<dyn std::error::Error>> {
    let activation_code = uuid::Uuid::new_v4();
    let select_user_id_cql: &str = formatcp!("SELECT userid FROM {USER_KS_NAME}.{USER_CREDS_TAB_NAME} WHERE {USER_EMAIL} = ? LIMIT 1");

    let mut userid = uuid::Uuid::default();
    if let Some(rows) = session.query(select_user_id_cql,(&email,)).await?.rows {
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
    
    session.query(USER_ACTIVATION_INSERT, (activation_code, userid)).await?;    

    Ok(activation_code)
}

pub async fn activate_user(session: &Arc<Session>, activation_code: &Uuid) -> Result<(),Box<dyn std::error::Error>> {
    let get_userid_cql: &str = formatcp!("SELECT {USER_ID} FROM {USER_KS_NAME}.{USER_ACTIVATION_TAB_NAME} WHERE {USER_ACTIVATION_CODE} = ? LIMIT 1");

    let modified = Duration::seconds(Utc::now().timestamp());

    let mut userid = uuid::Uuid::default();
    if let Some(rows) = session.query(get_userid_cql,(activation_code,)).await?.rows {
        for row in rows.into_typed::<(uuid::Uuid,)>() {
            match row {
                Ok(r) => {
                    userid=r.0;
                }
                Err(e) => {
                    // log e
                    return Err(format!("activation unsuccessful, please refresh activation code & try again. {}",e).into())
                }
            }
        }
    }

    let update_user_creds_cql: &str = formatcp!("UPDATE {USER_KS_NAME}.{USER_CREDS_TAB_NAME} SET {USER_IS_ACTIVE} = ? WHERE {USER_ID} = ?");

    let update_user_info_cql: &str = formatcp!("UPDATE {USER_KS_NAME}.{USER_INFO_TAB_NAME} SET {USER_IS_ACTIVE} = ?, {MODIFIED_DATE} = ? WHERE {USER_ID} = ?");

    let user_creds_future = session.query(update_user_creds_cql, (true,userid));
    let user_info_future = session.query(update_user_info_cql, (true,Timestamp(modified),userid));    

    try_join!(user_creds_future, user_info_future)?;

    Ok(())
}