use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct StoreResponse{
    pub store_id: uuid::Uuid,
    pub place_id: Option<String>,
    pub formatted_address: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,    
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub slat: Option<i8>,
    pub slng: Option<i8>,
    pub active: bool
}

impl Default for StoreResponse {
    fn default () -> StoreResponse{ 
        StoreResponse {
            store_id: Uuid::nil(),
            place_id: None,
            formatted_address: None,
            name: None,
            description: None,
            lat: None,
            lng: None,
            slat: None,
            slng: None,
            active:false
        }
    } 
}
