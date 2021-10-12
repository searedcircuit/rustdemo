use chrono::{Utc,Duration};
use scylla::transport::errors::QueryError;
use std::sync::Arc;
use tokio::try_join;

use scylla::frame::value::Timestamp;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::db::common::{
    STORE_INFO_INSERT,
    STORE_LOC_MAP_INSERT,
    STORE_INFO_SELECT,
    STORE_LOC_MAP_SELECT};
use crate::data::{    
    response::store::get_store_response::StoreResponse,
    request::store::create_store_request::CreateStoreRequest,    
};

pub async fn insert_store(session: &Arc<Session>, store: &CreateStoreRequest) -> Result<(), QueryError> {
    let store_id = uuid::Uuid::new_v4();

    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());
    let slat = f64::round(store.lat.unwrap() * 10.0) as i16;
    let slng = f64::round(store.lng.unwrap() * 10.0) as i16;

    let create_store_future = session.query(
        STORE_INFO_INSERT, 
        (
            store_id,
            &store.place_id,
            &store.name,
            &store.description,
            &store.formatted_address,
            store.lat,
            store.lng,
            store.active,
            Timestamp(created),
            Timestamp(modified)
        ));

    let create_loc_map_future = session.query(
        STORE_LOC_MAP_INSERT, 
        (
            store_id,
            &store.place_id,
            store.lat,
            store.lng,
            slat,
            slng,
            Timestamp(created),
            Timestamp(modified)
        ));    
    
    try_join!(create_store_future,create_loc_map_future)?;    
        
    Ok(())
}

pub async fn select_stores(session: &Arc<Session>, userlat: f64,userlng: f64)-> Result<Vec<StoreResponse>,Box<dyn std::error::Error>> {      
    const RANGE:f64 = 0.1_f64;
    
    let slat = f64::round(userlat * 10.0) as i16;
    let slng = f64::round(userlng * 10.0) as i16;
    let slatmin = f64::round((userlat-RANGE) * 10.0) as i16;
    let slngmin = f64::round((userlng-RANGE) * 10.0) as i16;
    let slatmax = f64::round((userlat+RANGE) * 10.0) as i16;
    let slngmax = f64::round((userlng+RANGE) * 10.0) as i16;
    let (ulatmin,ulatmax,ulngmin,ulngmax) 
        = (userlat-RANGE,userlat+RANGE,userlng-RANGE,userlng+RANGE);        

    let mut storeids: Vec<String> = Vec::new();
    if let Some(rows) = session.query(
        STORE_LOC_MAP_SELECT,
        (slatmin,slat,slatmax,slngmin,slng,slngmax,ulatmin,ulngmin,ulatmax,ulngmax))
        .await?.rows {
        for row in rows.into_typed::<(uuid::Uuid,)>() {
            match row {
                Ok(r) => {
                    storeids.push(r.0.to_string());
                }
                Err(e) => {
                    // log e
                    return Err(format!("Error locating storeids. {}",e).into())
                }
            };
        }
    }

    let storeids_str = storeids.join(",");
    let select_store_cql_updated = STORE_INFO_SELECT.replace("?", &storeids_str);

    let mut stores: Vec<StoreResponse> = Vec::new();
    if let Some(rows) = session.query(
        select_store_cql_updated,&[])
        .await?.rows {
        for row in rows.into_typed::<(uuid::Uuid, String,String,String,String,f64,f64, bool)>() {
            match row {
                Ok(r) => {
                    let store = StoreResponse{
                        store_id: r.0,
                        place_id: Some(r.1),
                        name: Some(r.2),
                        description: Some(r.3),
                        formatted_address: Some(r.4),
                        lat: Some(r.5),
                        lng: Some(r.6),
                        active:r.7
                    };
                    stores.push(store);
                }
                Err(e) => {
                    // log e
                    return Err(format!("Error locating stores. {}",e).into())
                }
            };
        }
    }
    Ok(stores)
}