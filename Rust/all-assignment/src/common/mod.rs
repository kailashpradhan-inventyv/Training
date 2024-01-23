
use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize,Serialize)]


pub struct Employee{
    name:String,
    age:u8,
    pub position:Option<Position>,
    #[serde(rename="experiance(y)")]
    experiance:Option<u8>,
    pub skills:Vec<Skills>,
}
#[derive(PartialEq,Debug,Serialize,Deserialize)]

pub enum Position{
    #[serde(rename="Jr. Software Developer")]
    Jr,
    #[serde(rename="Software Developer")]
    Mid,
    #[serde(rename="Sr. Software Developer")]
    Sr,
    #[serde(rename="Project Manager")]
    Pm,
    #[serde(rename="Team Lead")]
    Tl
}
#[derive(Debug,Serialize,PartialEq,Deserialize)]
pub enum Skills{
    Python,
    #[serde(rename="C#")]
    CS,
    Rust,
    Java
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
}

#[derive(Debug)]
pub struct Cell{
    pub width:u8,
    pub height:u8,
    pub value:u8
}

#[derive(Debug)]

pub struct Row{
    pub row_height:u8,
    pub tot_cells:u8,
    pub row_cells:Vec<Cell>,
    pub row_width:u8
}


#[derive(Debug)]


pub struct Table{
    pub total_height:u8,
    pub total_width:u8,
    pub all_row:Vec<Row>,
    pub no_row:u8,
}
