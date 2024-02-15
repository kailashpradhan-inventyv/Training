use std::{fs::read_to_string, sync::{Arc, RwLock}};

use lazy_static::lazy_static;

use crate::{axum_crud_task::routes::all_routes, common::{CustomerData, EmpData, StudData}};

use self::middlewares::get_middlewares;
mod routes;
mod exec;
mod stud;
mod emp;
mod health_check;
mod middlewares;
lazy_static!{
    pub static ref MASTER_DATA:Arc<RwLock<Vec<CustomerData>>>={
        let des_data=read_to_string("JSON-data/Master_Data.json").expect("Unable to Read");
        let master_data:Vec<CustomerData>=serde_json::from_str(&des_data).expect("Deserialize Fail");
        Arc::new(RwLock::new(master_data))
    };
    pub static ref EMP_DATA:Arc<RwLock<Vec<EmpData>>>={
        let content=read_to_string("JSON-data/emp_data.json").expect("Unable to read");
        let emp_data:Vec<EmpData>=serde_json::from_str(&content).expect("Deserialize Fail");
        Arc::new(RwLock::new(emp_data))
    };

    pub static ref STUD_DATA:Arc<RwLock<Vec<StudData>>>={
        let content=read_to_string("JSON-data/stud_data.json").expect("Unable to read");
        let stud_data:Vec<StudData>=serde_json::from_str(&content).expect("Deserialize Fail");
        Arc::new(RwLock::new(stud_data))
    };
}
pub async fn axum_server(){
    let app=all_routes();
    let app=get_middlewares(app);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}