use axum::{routing::post, Router};

use self::services::{delete_data_by_id, get_all_data, get_data_by_id, new_data, update_data};

pub mod services;

pub fn emp_routes()->Router{
    Router::new()
    .route("/emp/get_all", post(get_all_data))
    .route("/emp/get_by_id/:id", post(get_data_by_id))
    .route("/emp/update_data", post(update_data))
    .route("/emp/delete_data/:id", post(delete_data_by_id))
    .route("/emp/new_data", post(new_data))
}