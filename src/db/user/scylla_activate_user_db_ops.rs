use futures::try_join;
use scylla::frame::value::Timestamp;
use scylla::transport::errors::QueryError;
use uuid::Uuid;
use chrono::Duration;
use chrono::Utc;
use std::sync::Arc;

use scylla::transport::session::{IntoTypedRows, Session};

pub async fn activate_user(session: &Arc<Session>, activation_code: &Uuid) -> Result<(),Box<dyn std::error::Error>> {
    let get_userid_cql: String = format!("SELECT {},{} FROM {}.{} WHERE {} = {} LIMIT 1",
        USER_ID,
        USER_EMAIL,
        KS_NAME,ACTIVATION_TAB_NAME,
        ACTIVATION_CODE,activation_code);

    let modified = Duration::seconds(Utc::now().timestamp());

    let mut userid = uuid::Uuid::default();
    let mut email = String::new();
    if let Some(rows) = session.query(get_userid_cql,&[]).await?.rows {
        for row in rows.into_typed::<(uuid::Uuid,Option<String>)>() {
            match row {
                Ok(r) => {
                    userid=r.0;
                    match r.1{
                        Some(em)=>email=em,
                        _=>return Err(format!("activation unsuccessful, please refresh activation code & try again.").into())
                    }
                }
                Err(e) => {
                    // log e
                    return Err(format!("activation unsuccessful, please refresh activation code & try again. {}",e).into())
                }
            }
        }
    }

    let update_user_creds_cql: String = format!("UPDATE {}.{} SET {} = ? WHERE {} = ?",
        KS_NAME,USER_CREDS_TAB_NAME,
        USER_IS_ACTIVE,
        USER_EMAIL);

    let update_user_info_cql: String = format!("UPDATE {}.{} SET {} = ?, {} = ? WHERE {} = ?",
        KS_NAME,USER_TAB_NAME,
        USER_IS_ACTIVE,
        MODIFIED_DATE,
        USER_ID);

    let user_creds_future = session.query(update_user_creds_cql, (true,email));
    let user_info_future = session.query(update_user_info_cql, (true,Timestamp(modified),userid));    

    try_join!(user_creds_future, user_info_future)?;

    Ok(())
}

pub async fn create_all(session: &Arc<Session>) -> Result<(), QueryError> {
    create_user_activation_table(session).await?;
    Ok(())
}

async fn create_user_activation_table(session: &Arc<Session>) -> Result<(), QueryError> {
    let create_table_cql = format!(
        //"ALTER TABLE {}.{} ADD {} text",KS_NAME,ACTIVATION_TAB_NAME,USER_EMAIL);
        "CREATE TABLE IF NOT EXISTS {}.{} ( {} UUID PRIMARY KEY, {} UUID, {} text)",KS_NAME,ACTIVATION_TAB_NAME,ACTIVATION_CODE,USER_ID,USER_EMAIL);
    session
        .query(create_table_cql,&[])
        .await?;
    Ok(())
}