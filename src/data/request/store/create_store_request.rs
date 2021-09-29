use serde::{Deserialize,Serialize};

#[derive(Clone,PartialEq,Serialize,Deserialize)]
pub struct CreateStoreRequest{
    pub place_id: Option<String>,
    pub formatted_address: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,    
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub active: bool,
}

impl Default for CreateStoreRequest {
    fn default () -> CreateStoreRequest{ 
        CreateStoreRequest {
            place_id: None,
            formatted_address: None,
            name: None,
            description: None,
            lat: None,
            lng: None,
            active: false,
        }
    } 
}
