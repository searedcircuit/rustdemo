use futures::future::try_join_all;
use chrono::{Utc,Duration};
use scylla::transport::errors::QueryError;
use tokio::try_join;
use std::sync::Arc;

use scylla::frame::value::Timestamp;
use scylla::transport::session::{IntoTypedRows, Session};

use crate::db::common::{
    MENU_ITEM_INSERT,
    MENU_OPTION_INSERT,
    MENU_ITEM_SELECT,
    MENU_OPTION_SELECT};
use crate::data::{   
    db::{
        DbMenuItem,
        DbMenuOption
    }, 
    request::menu::{
        create_menu_request::CreateMenuRequest,
        create_menu_option_request::CreateOrUpdateMenuOptionRequest
    },
    response::menu::get_menu_response::MenuResponse
};

pub async fn insert_menu_item(session: &Arc<Session>, item: &CreateMenuRequest) -> Result<(), QueryError> {
    let item_id = uuid::Uuid::new_v4();
    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());

    session.query(
        MENU_ITEM_INSERT, 
        (
            item.store_id,
            item_id,
            &item.name,
            &item.desc,
            &item.size.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            &item.temp.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            &item.cost,
            Timestamp(created),
            Timestamp(modified)
        )).await?; 

    Ok(())
}

pub async fn insert_menu_option(session: &Arc<Session>, option: &CreateOrUpdateMenuOptionRequest) -> Result<(), QueryError> {
    let now = Utc::now();
    let created = Duration::seconds(now.timestamp());
    let modified = Duration::seconds(now.timestamp());

    let mut lst = Vec::new();
    for opt in &option.options {
        let opt_id = uuid::Uuid::new_v4();
        let fut = session.query(
            MENU_OPTION_INSERT, 
            (
                option.store_id,
                opt_id,
                &opt.name,
                &opt.cost,
                Timestamp(created),
                Timestamp(modified)
            )); 
        lst.push(fut);
    }

    try_join_all(lst).await?;    

    Ok(())
}

pub async fn select_menu(session: &Arc<Session>, store_id: uuid::Uuid)-> Result<MenuResponse,Box<dyn std::error::Error>> {      
    let mut menu: MenuResponse = MenuResponse::default();
    menu.store_id = store_id;

    let items_future = session.query(
        MENU_ITEM_SELECT,(store_id,));
    let options_future = session.query(
        MENU_OPTION_SELECT, (store_id,));

    let (items_res,options_res) = try_join!(items_future,options_future)?;

    if let Some(rows) = items_res.rows {
        for row in rows.into_typed::<(uuid::Uuid, uuid::Uuid,String,String,String,String,i32,Duration,Duration)>() {
            match row {                
                Ok(r) => {
                    let item = DbMenuItem{           
                        item_id: r.1,

                        item_name: Some(r.2),
                        item_desc: Some(r.3),
                        item_size: r.4.split(",").map(|s| s.to_string()).collect(),
                        item_temp: r.5.split(",").map(|s| s.to_string()).collect(),
                        item_cost: Some(r.6)
                    };
                    menu.items.push(item);
                }
                Err(e) => {
                    // log e
                    return Err(format!("Error locating stores. {}",e).into())
                }
            };
        }
    }

    if let Some(rows) = options_res.rows {
        for row in rows.into_typed::<(uuid::Uuid, uuid::Uuid,String,i32,Duration,Duration)>() {
            match row {                
                Ok(r) => {
                    let option = DbMenuOption{
                        
                        option_id: r.1,
                        name: Some(r.2),
                        cost: Some(r.3)
                    };
                    menu.options.push(option);
                }
                Err(e) => {
                    // log e
                    return Err(format!("Error locating stores. {}",e).into())
                }
            };
        }
    }

    Ok(menu)
}