    use crate::common::{dataRows, data_type, headerRow, tCell, tRow, tTable, tableData};
use std::{collections::HashMap, fmt::Write, fs::{read_to_string, write}};

use lazy_static::lazy_static;
lazy_static!{
    static ref CHAR_WIDTH: HashMap<String, f32> = {
        let deser_data=read_to_string("JSON-data/fonrData.json");
        match deser_data{
            Ok(data)=>{
                let char_width:HashMap<String,f32>=serde_json::from_str(&data).expect("Error");
                char_width
            }
            Err(e)=>{
                panic!("Error reading file: {}",e);
            }
        }  
    };
}

impl tCell {
    fn new(data:String)->tCell{
        tCell{data}
    }
}

impl tRow {
    fn new(row_cells:Vec<tCell>,cell_width:f32,row_height:f32,data_type:data_type)->tRow{

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
    fn new(rows:Vec<tRow>)->tTable{
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


pub fn table_value_update(){
    let table_data=read_to_string("JSON-data/data.json");
    match table_data{
        Ok(data)=>{
            let des_data:tableData=serde_json::from_str(&data).expect("Deserialized Failed");
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
                    total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * hData.fontSize as f32;
                    if area<total_width{
                        total_height+=hData.fontSize as f32+pad_top+pad_bottom;
                        area+=cell_width as f32-pad_left-pad_right;
                        val.write_char('\n');
                    }
                    val.write_char(each_char);
                }
                // print!("{}",val);
                
                cells.push(tCell::new(val));
                
                if total_height>row_height{
                    row_height=total_height;
                }
            }   
            rows.push(tRow::new(cells,cell_width,row_height,data_type::HEADER));

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
                    cells.push(tCell::new(val));
                    // row_height=total_height;
                    // println!("{}",row_height)
                    if(total_height>row_height){
                        row_height=total_height;
                    }
                }
                // println!("{}",row_height);
                rows.push(tRow::new(cells,cell_width,row_height,data_type::RECORD));

            }
            let a=tTable::new(rows);
            let ser_data=serde_json::to_string_pretty(&a).expect("Serialized Fail");
            write("JSON-data/test.json", ser_data);
        }
        Err(e)=>{
            println!("Error {}",e);
        }
    }
}



// fn update_heights(datas:String,cell_width:f32,cell_data_font:i32,mut row_height:f32)->Vec<tRow>{

//     let pad_top=1.0;
//     let pad_bottom=1.0;
//     let pad_left=1.0;
//     let pad_right=1.0;
//     let mut total_width=0.0;
//     let mut total_height:f32=0.0;


//     let mut rows=Vec::new();   
//     let mut cells=Vec::new();
//     for data in datas{

//     }
//     total_height=cell_data_font as f32+pad_bottom+pad_top;
//     let mut total_width=0.0;
//     let mut area=cell_width as f32;
//     let mut val:String="".to_string();
        
//     for each_char in datas.chars(){
//         total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * cell_data_font as f32;
//         if area<total_width{
//             total_height+=cell_data_font as f32+pad_top+pad_bottom;
//             area+=cell_width as f32-pad_left-pad_right;
//             val.write_char('\n');
//         }
//         val.write_char(each_char);
//     }
    
//     cells.push(tCell::new(val));
        
//     if total_height>row_height{
//         row_height=total_height;
//     }
//     rows.push(tRow::new(cells,cell_width,row_height,data_type::HEADER));

//     rows

// }   
    


