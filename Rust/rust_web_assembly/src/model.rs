use std::collections::HashMap;

use serde::{Deserialize,Serialize};
use std::fs::read_to_string;
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

use lazy_static::lazy_static;

lazy_static!{
    #[derive(Debug)]
    pub static ref CHAR_WIDTH: HashMap<String, f32> = {
                let mut char_width=HashMap::new();
                char_width.insert("0".to_string(), 0.5);
                char_width.insert("1".to_string(), 0.5);
                char_width.insert("2".to_string(), 0.5);
                char_width.insert("3".to_string(), 0.5);
                char_width.insert("4".to_string(), 0.5);
                char_width.insert("5".to_string(), 0.5);
                char_width.insert("6".to_string(), 0.5);
                char_width.insert("7".to_string(), 0.5);
                char_width.insert("8".to_string(), 0.5);
                char_width.insert("9".to_string(), 0.5);
                char_width.insert("".to_string(), 0.0);
                char_width.insert(" ".to_string(), 0.25);
                char_width.insert(".".to_string(), 0.25);
                char_width.insert("!".to_string(), 0.333);
                char_width.insert("\"".to_string(), 0.555);
                char_width.insert("#".to_string(), 0.5);
                char_width.insert("$".to_string(), 0.5);
                char_width.insert("%".to_string(), 1.0);
                char_width.insert("&".to_string(), 0.83300006);
                char_width.insert("'".to_string(), 0.27800003);
                char_width.insert("(".to_string(), 0.333);
                char_width.insert(")".to_string(), 0.333);
                char_width.insert("*".to_string(), 0.5);
                char_width.insert("+".to_string(), 0.57000005);
                char_width.insert(":".to_string(), 0.25);
                char_width.insert("-".to_string(), 0.333);
                char_width.insert(",".to_string(), 0.333);
                char_width.insert(";".to_string(), 0.333);
                char_width.insert("<".to_string(), 0.57000005);
                char_width.insert("=".to_string(), 0.57000005);
                char_width.insert(">".to_string(), 0.57000005);
                char_width.insert("?".to_string(), 0.5);
                char_width.insert("@".to_string(), 0.93000007);
                char_width.insert("A".to_string(), 0.72200006);
                char_width.insert("E".to_string(), 0.66700006);
                char_width.insert("F".to_string(), 0.611);
                char_width.insert("G".to_string(), 0.77800006);
                char_width.insert("H".to_string(), 0.77800006);
                char_width.insert("I".to_string(), 0.38900003);
                char_width.insert("J".to_string(), 0.5);
                char_width.insert("K".to_string(), 0.77800006);
                char_width.insert("L".to_string(), 0.66700006);
                char_width.insert("M".to_string(), 0.94400007);
                char_width.insert("N".to_string(), 0.72200006);
                char_width.insert("O".to_string(), 0.77800006);
                char_width.insert("P".to_string(), 0.611);
                char_width.insert("Q".to_string(), 0.77800006);
                char_width.insert("R".to_string(), 0.72200006);
                char_width.insert("S".to_string(), 0.55600005);
                char_width.insert("T".to_string(), 0.66700006);
                char_width.insert("U".to_string(), 0.72200006);
                char_width.insert("V".to_string(), 0.72200006);
                char_width.insert("W".to_string(), 1.0);
                char_width.insert("X".to_string(), 0.72200006);
                char_width.insert("Y".to_string(), 0.72200006);
                char_width.insert("Z".to_string(), 0.66700006);
                char_width.insert("]".to_string(), 0.333);
                char_width.insert("/".to_string(), 0.27800003);
                char_width.insert("[".to_string(), 0.333);
                char_width.insert("^".to_string(), 0.58100003);
                char_width.insert("_".to_string(), 0.5);
                char_width.insert("`".to_string(), 0.333);
                char_width.insert("a".to_string(), 0.5);
                char_width.insert("b".to_string(), 0.55600005);
                char_width.insert("c".to_string(), 0.44400004);
                char_width.insert("d".to_string(), 0.55600005);
                char_width.insert("e".to_string(), 0.44400004);
                char_width.insert("f".to_string(), 0.333);
                char_width.insert("g".to_string(), 0.5);
                char_width.insert("h".to_string(), 0.55600005);
                char_width.insert("i".to_string(), 0.27800003);
                char_width.insert("j".to_string(), 0.333);
                char_width.insert("k".to_string(), 0.55600005);
                char_width.insert("l".to_string(), 0.27800003);
                char_width.insert("m".to_string(), 0.83300006);
                char_width.insert("n".to_string(), 0.55600005);
                char_width.insert("o".to_string(), 0.5);
                char_width.insert("p".to_string(), 0.55600005);
                char_width.insert("q".to_string(), 0.55600005);
                char_width.insert("r".to_string(), 0.44400004);
                char_width.insert("s".to_string(), 0.38900003);
                char_width.insert("t".to_string(), 0.333);
                char_width.insert("u".to_string(), 0.55600005);
                char_width.insert("v".to_string(), 0.5);
                char_width.insert("w".to_string(), 0.72200006);
                char_width.insert("x".to_string(), 0.5);
                char_width.insert("y".to_string(), 0.5); 
                char_width.insert("z".to_string(), 0.44400004);  
                char_width.insert("{".to_string(), 0.39400002);  
                char_width.insert("|".to_string(), 0.22000001);  
                char_width.insert("}".to_string(), 0.39400002);  
                char_width.insert("~".to_string(), 0.52000004);  
                char_width.insert("B".to_string(), 0.66700006);  
                char_width.insert("C".to_string(), 0.72200006);  
                char_width.insert("D".to_string(), 0.72200006);  
                char_width
    };
} 

impl tCell {
    pub fn c_new(data:String)->tCell{
        tCell{data}
    }
}

impl tRow {
    pub fn r_new(row_cells:Vec<tCell>,cell_width:f32,row_height:f32,data_type:data_type)->tRow{

        let total=row_cells.len() as i32;
        let row_width=cell_width*total as f32;
            
        tRow{
            cell_width,
            total_cells:total,
            row_width,
            row_height,
            row_type:data_type,
            row_cells
        }
    }
}

impl tTable{
    pub fn t_new(rows:Vec<tRow>)->tTable{
        let mut width:f32=0.0;
        let mut height:f32 =0.0;
        let tot_rows=rows.len();
        
        for i in 0..rows.len(){
            if width<=rows[i].row_width {
                width=rows[i].row_width;
            }
            height=rows[i].row_height+ height;
        }

        tTable{
            tab_height:height,
            tab_width:width,
            all_rows:rows,
            tot_rows:tot_rows as i32
        }
    }
}








