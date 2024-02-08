use std::{collections::{HashMap}, fmt::Write};
use crate::common::{Cell,Row,Table,headerRow,dataRows,tableData};
impl Cell {
    fn cell_update(width:f32,height:f32,value:String)->Cell{
        Cell{
            width,
            height,
            value
        }
    }
}

impl Row {
    fn row_update(cell:Vec<Cell>)->Row{
        let mut height=0.0;
        let mut width=0.0;
        let total=cell.len();
        for i in 0..cell.len(){
            if height<=cell[i].height {
                height=cell[i].height;
            }
            width=cell[i].width+ width;
            
        }
        
        Row{
            row_height:height,
            tot_cells:total as f32,
            row_cells:cell,
            row_width:width,

        }     
    }
}

impl Table{
    fn table_update(rows:Vec<Row>)->Table{
        let mut width=0.0;
        let mut height =0.0;
        let tot_rows=rows.len();
        
        for i in 0..rows.len(){
            if width<=rows[i].row_width {
                width=rows[i].row_width;
            }
            height=rows[i].row_height+ height;
        }

        Table{
            total_height:height,
            total_width:width,
            all_row:rows,
            no_row:tot_rows as f32
        }
    }
}

pub fn table_value_update(){
    use std::fs::{read_to_string,write};
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
    let table_data=read_to_string("JSON-data/data.json");
    match table_data{
        Ok(data)=>{
            let des_data:tableData=serde_json::from_str(&data).expect("msg");
            let hData:headerRow=des_data.headerRow;
            let rData:dataRows=des_data.dataRows;
            let max_width=des_data.pageWidth;
            
            let cell_width:f32=(max_width-20.0)/rData.rows.len() as f32;
            let cell_data_font=rData.fontSize as f32;
            
            let mut rows=Vec::new();    
            let mut cells=Vec::new();


            for each_head_data in hData.title{
                let mut total_height:f32=cell_data_font;
                let mut total_width:f32=0.0;
                let mut area=cell_width;
                let mut val:String="".to_string();
                for each_char in each_head_data.chars(){
                    if area<total_width{
                        total_height+=cell_data_font;
                        area+=cell_width;
                        val.write_char('\n');
                    }
                    val.write_char(each_char);
                    total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * cell_data_font;
                }
                cells.push(Cell::cell_update(cell_width, total_height, val));


            }   
            rows.push(Row::row_update(cells));


            for each_row in rData.rows{
                let mut cells=Vec::new();
                for each_cell in each_row{
                    // let value_char=each_cell.chars();
                    // println!("{:?}",value_char);
                    let mut total_height:f32=cell_data_font;
                    let mut total_width:f32=0.0;
                    let mut area=cell_width;
                    let mut val:String="".to_string();
                    
                    for each_char in each_cell.chars(){
                        if area<total_width{
                            total_height+=cell_data_font;
                            area+=cell_width;
                            val.write_char('\n');
                        }
                        val.write_char(each_char);
                        total_width+=CHAR_WIDTH.get(&each_char.to_string()).unwrap() * cell_data_font;
                    }
                    cells.push(Cell::cell_update(cell_width, total_height, val));
                }
                rows.push(Row::row_update(cells));

            }
            let a=Table::table_update(rows);
            println!("{:?}",a);
            let ser_data=serde_json::to_string_pretty(&a).expect("msg");
            write("JSON-data/table_hash.json", ser_data);

        }
        Err(e)=>{
            print!("Failed to read {}",e);
        }
    } 
}
