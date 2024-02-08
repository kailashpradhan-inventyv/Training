
use crate::common::{Cell,Row,Table};
impl Cell {
    fn cell_data(width:f32,height:f32,value:String)->Cell{
        Cell{
            width,
            height,
            value
        }
    }
}

impl Row {
    fn new_row(cell:Vec<Cell>)->Row{
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
    fn create_table(rows:Vec<Row>)->Table{
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

    fn update_data(&mut self,row_index:usize,cell_no:usize,cell_height:f32,cell_width:f32){
            
        if row_index < self.all_row.len() && cell_no<self.all_row[row_index].tot_cells as usize{
            self.all_row[row_index].row_cells[cell_no].height=cell_height;
            self.all_row[row_index].row_cells[cell_no].width=cell_width;
        
            let mut height=0.0;
            let mut width=0.0;
            for cell in &self.all_row[row_index].row_cells {
                if cell.height > height {
                    height = cell.height;
                }
                width += cell.width;
            }
            
            self.all_row[row_index].row_height = height;
            self.all_row[row_index].row_width = width;

            
            self.total_height = 0.0;
            self.total_width = 0.0;

            for row in &self.all_row {
                if row.row_width > self.total_width {
                    self.total_width = row.row_width;
                }
                
                self.total_height += row.row_height;
            }
        } 
        else {
            println!("Wrong Input");
        }

    }
        
}

pub fn table_update(){
    let c1=Cell::cell_data(1.0, 2.0, "3".to_string());
    let c2=Cell::cell_data(1.0, 3.0, "4".to_string());
    let r1=Row::new_row(vec![c1,c2]);
    let c3=Cell::cell_data(1.0, 4.0, "5".to_string());
    let c4=Cell::cell_data(1.0, 5.0, "6".to_string());
    let r2=Row::new_row(vec![c3,c4]);
    let mut table=Table::create_table(vec![r1,r2]);
    println!("{:#?}",table);
    table.update_data(1, 1, 1.0,5.0);
    println!("{:#?}",table);
}

