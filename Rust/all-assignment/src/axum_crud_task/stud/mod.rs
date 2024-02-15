use axum::{routing::{post}, Router};

use self::services::{get_all_data,delete_data_by_id,get_data_by_id,new_data,update_data};

// use self::{get_all_data,get_data_by_id,update_data,delete_data::delete_data_by_id,new_data::new_data};

pub mod services;

pub fn stud_routes()->Router{
    Router::new()
    .route("/stud/get_all", post(get_all_data))
    .route("/stud/get_by_id/:id", post(get_data_by_id))
    .route("/stud/update_data", post(update_data))
    .route("/stud/delete_data/:id", post(delete_data_by_id))
    .route("/stud/new_data", post(new_data))
}