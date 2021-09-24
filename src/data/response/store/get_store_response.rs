use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct StoreResponse{
    pub place_id: Option<String>,
    pub formatted_address: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,    
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

impl Default for StoreResponse {
    fn default () -> StoreResponse{ 
        StoreResponse {
            place_id: None,
            formatted_address: None,
            name: None,
            description: None,
            lat: None,
            lng: None
        }
    } 
}
