use axum::Router;
use tower_http::cors::Cors;

use super::{emp::emp_routes, exec::exec_routes, health_check::get_status_routes, middlewares::{self, get_middlewares}, stud::stud_routes};

pub fn all_routes()->Router{
    Router::new()
    .merge(get_status_routes())
    .merge(exec_routes())
    .merge(stud_routes())
    .merge(emp_routes())
    
}