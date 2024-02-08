
use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize,Serialize)]

pub struct Employee{
    pub name:String,
    pub age:u8,
    pub position:Option<Position>,
    #[serde(rename="experiance(y)")]
    pub experiance:Option<u8>,
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
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
}

#[derive(Debug,Serialize)]
pub struct Cell{
    pub width:f32,
    pub height:f32,
    pub value:String
}

#[derive(Debug,Serialize)]

pub struct Row{
    pub row_height:f32,
    pub tot_cells:f32,
    pub row_cells:Vec<Cell>,
    pub row_width:f32
}


#[derive(Debug,Serialize)]


pub struct Table{
    pub total_height:f32,
    pub total_width:f32,
    pub all_row:Vec<Row>,
    pub no_row:f32,
}


#[derive(Debug, Deserialize)]

pub struct headerRow{
    pub fontSize:i32,
    pub title:Vec<String>
}
#[derive(Debug, Deserialize)]

pub struct dataRows{
    pub fontSize:i32,
    pub rows:Vec<Vec<String>>
}
#[derive(Debug, Deserialize)]

pub struct tableData{
    pub headerRow:headerRow,
    pub dataRows:dataRows,
    pub pageWidth:i32
}

#[derive(Debug,Serialize)]
pub enum data_type{
    HEADER,
    RECORD
}

#[derive(Debug,Serialize)]
pub struct tCell{
    pub data:String
}

#[derive(Debug,Serialize)]
pub struct tRow{
    pub cell_width:f32,
    pub total_cells:i32,
    pub row_width:f32,
    pub row_height:f32,
    pub row_type:data_type,
    pub row_cells:Vec<tCell>
}

#[derive(Debug,Serialize)]
pub struct tTable{
    pub tab_height: f32,
    pub tab_width: f32,
    pub all_rows:Vec<tRow>,
    pub tot_rows:i32
}


#[derive(Debug)]
pub struct Data{
    pub id:i32,
    pub name:String,
    pub timestamp:DateTime<Utc>
}


#[derive(Debug,Deserialize,Serialize,PartialEq)]
pub struct CustomerData{
    pub id:i32,
    pub name:String,
    pub skills:Vec<String>,
    pub status:String,
    pub language:language
}

#[derive(Debug,PartialEq)]
pub enum available{
    Chat,
    Call
}
#[derive(Debug,Deserialize,Serialize,PartialEq)]
pub enum language{
    English,
    Spanish
}
#[derive(Debug)]
pub struct request_data{
    pub skills:String,
    pub available:available,
    pub language:language,
    pub timestamp:DateTime<Utc>
}




