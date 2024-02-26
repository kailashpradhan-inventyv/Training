use wasm_bindgen::prelude::*;

mod model;
use crate::model::{dataRows, data_type, headerRow, tCell, tRow, tTable, tableData};
use std::fmt::Write;

use crate::model::CHAR_WIDTH;
#[wasm_bindgen]
pub fn table_value_update(datafile:String)->String{
            let des_data:tableData=serde_json::from_str(&datafile).expect("Deserialized Failed");
            let hData:headerRow=des_data.headerRow;
            let rData:dataRows=des_data.dataRows;
            let pad_top=1.0;
            let pad_bottom=1.0;
            let pad_left=1.0;
            let pad_right=1.0;

            let cell_width:f32=(des_data.pageWidth-20) as f32/hData.title.len() as f32;
            let cell_data_font=rData.fontSize as f32;
            
            let mut rows=Vec::new();   
            let mut cells=Vec::new();

            // Header Data
            let mut total_height:f32=0.0;
            let mut row_height=hData.fontSize as f32+pad_bottom+pad_top;
            for each_head_data in hData.title{
                total_height=hData.fontSize as f32+pad_bottom+pad_top;
                let mut total_width=0.0;
                let mut area=cell_width as f32;
                let mut val:String="".to_string();
                
                for each_char in each_head_data.chars(){
                    println!("{:?}",CHAR_WIDTH.get(&each_char.to_string()).unwrap());
                    total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * hData.fontSize as f32;
                    if area<total_width{
                        total_height+=hData.fontSize as f32+pad_top+pad_bottom;
                        area+=cell_width as f32-pad_left-pad_right;
                        val.write_char('\n');
                    }
                    val.write_char(each_char);
                }
                // print!("{}",val);
                
                cells.push(tCell::c_new(val));
                
                if total_height>row_height{
                    row_height=total_height;
                }
            }   
            rows.push(tRow::r_new(cells,cell_width,row_height,data_type::HEADER));

            // println!("{:?}",rows);

            // Record Data

            for each_row in rData.rows{
                let mut cells=Vec::new();
                // let mut row_height=0.0;
                let mut row_height=hData.fontSize as f32+pad_bottom+pad_top;

                for each_cell in each_row{
                    let mut total_height=rData.fontSize as f32+pad_bottom+pad_top;
                    let mut total_width:f32=0.0;
                    let mut area=cell_width as f32;
                    let mut val:String="".to_string();
                    for each_char in each_cell.chars(){
                        total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * cell_data_font as f32;
                        if area<total_width{
                            total_height+=cell_data_font+pad_bottom+pad_top;
                            area+=cell_width as f32-pad_left-pad_right;
                            val.write_char('\n');
                        }
                        val.write_char(each_char);
                    }
                    cells.push(tCell::c_new(val));
                    if total_height>row_height {
                        row_height=total_height;
                    }
                }
                // println!("{}",row_height);
                rows.push(tRow::r_new(cells,cell_width,row_height,data_type::RECORD));

            }
            let a=tTable::t_new(rows);
            let ser_data=serde_json::to_string_pretty(&a).expect("Serialized Fail");
            ser_data
}
