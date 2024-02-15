use axum::{routing::post, Router};

use self::services::{get_all_data,delete_data_by_id,update_data,new_data,get_data_by_id};

// use crate::axum_crud_task::exec::{get_all::get_all_data, get_by_id::get_data_by_id,update_data::update_data,delete_data::delete_data_by_id,new_data::new_data};

pub mod services;

pub fn exec_routes()->Router{
    Router::new()
    .route("/exec/get_all", post(get_all_data))
    .route("/exec/get_by_id/:id", post(get_data_by_id))
    .route("/exec/update_data", post(update_data))
    .route("/exec/delete_data/:id", post(delete_data_by_id))
    .route("/exec/new_data", post(new_data))
}