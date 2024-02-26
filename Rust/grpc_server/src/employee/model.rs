use std::{fs::read_to_string, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize,Clone)]

pub struct EmpData{
    pub id:i32,
    pub name:String,
    pub age:i32,
    pub position:Option<String>,
    #[serde(rename="experiance(y)")]
    pub experiance:Option<i32>,
    pub skills:Vec<String>,
}




lazy_static!{
    pub static ref EMP_DATA:Arc<RwLock<Vec<EmpData>>>={
        let content=read_to_string("json_data/employee.json").expect("Unable to read");
        let emp_data:Vec<EmpData>=serde_json::from_str(&content).expect("Deserialize Fail");
        Arc::new(RwLock::new(emp_data))
    };
}