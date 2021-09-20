use std::sync::Arc;

use scylla::transport::session::Session;

use crate::data::response::UserLoginResponse;
use crate::db::common::{AUTH_CODE_INSERT, AUTH_REFRESH_CODE_INSERT};

pub async fn get_auth_codes(session: &Arc<Session>, userid: uuid::Uuid) -> Result<UserLoginResponse,Box<dyn std::error::Error>> {
    let access_code = uuid::Uuid::new_v4();
    let refresh_code = uuid::Uuid::new_v4();
    
    session.query(AUTH_CODE_INSERT, (access_code, userid)).await?;
    session.query(AUTH_REFRESH_CODE_INSERT, (refresh_code, userid)).await?;

    let codes = UserLoginResponse{access_code: access_code, refresh_code: refresh_code,userid:userid };
    Ok(codes)
}