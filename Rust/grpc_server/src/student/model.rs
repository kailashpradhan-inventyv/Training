use std::{fs::read_to_string, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Clone,PartialEq)]
pub struct StudData {
    pub id:i32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<i32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
}
lazy_static!{
    pub static ref STUD_DATA:Arc<RwLock<Vec<StudData>>>={
        let content=read_to_string("json_data/student.json").expect("Unable to read");
        let stud_data:Vec<StudData>=serde_json::from_str(&content).expect("Deserialize Fail");
        Arc::new(RwLock::new(stud_data))
    };
}


