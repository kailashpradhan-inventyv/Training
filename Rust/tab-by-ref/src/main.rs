

#[derive(Debug)]
struct Cell{
    width:u8,
    height:u8,
    value:u8
}

impl Cell {
    fn cell_data(width:u8,height:u8,value:u8)->Cell{
        Cell{
            width,
            height,
            value
        }
    }
}

#[derive(Debug)]

struct Row{
    row_height:u8,
    tot_cells:u8,
    row_cells:Vec<Cell>,
    row_width:u8
}

impl Row {

    fn new_row(cell:Vec<Cell>)->Row{
        let mut height=0;
        let mut width=0;
        let total=cell.len();
        for i in 0..cell.len(){
            if(height<=cell[i].height){
                height=cell[i].height;
            }
            width=cell[i].width+ width;
            
        }
        
        Row{
            row_height:height,
            tot_cells:total as u8,
            row_cells:cell,
            row_width:width,

        }     
    }
    
}


#[derive(Debug)]


struct Table{
    total_height:u8,
    total_width:u8,
    all_row:Vec<Row>,
    no_row:u8,
}

impl Table{
    fn create_table(rows:Vec<Row>)->Table{
        let mut width=0;
        let mut height =0;
        let tot_rows=rows.len();
        
        // for i in 0..rows.len(){
        //     if(width<=rows[i].row_width){
        //         width=rows[i].row_width;
        //     }
        //     height=rows[i].row_height+ height;
        // }

        for each_row in &rows{
            if width<=each_row.row_width{
                width=each_row.row_width;
            }
            height+=each_row.row_height;
        }

        Table{
            total_height:height,
            total_width:width,
            all_row:rows,
            no_row:tot_rows as u8
        }
    }

    fn update_data(&mut self,row_index:usize,cell_no:usize,cell_height:u8,cell_width:u8){
            
        if row_index < self.all_row.len() && cell_no<self.all_row[row_index].tot_cells as usize{
            self.all_row[row_index].row_cells[cell_no].height=cell_height;
            self.all_row[row_index].row_cells[cell_no].width=cell_width;
        
            let mut height=0;
            let mut width=0;
            for cell in &self.all_row[row_index].row_cells {
                if cell.height > height {
                    height = cell.height;
                }
                width += cell.width;
            }
            
            self.all_row[row_index].row_height = height;
            self.all_row[row_index].row_width = width;

           
            self.total_height = 0;
            self.total_width = 0;

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

fn main(){
    // let num_rows=2;
    // let num_cells=2;
    let c1=Cell::cell_data(1, 2, 3);
    let c2=Cell::cell_data(1, 3, 4);
    let r1=Row::new_row(vec![c1,c2]);
    let c3=Cell::cell_data(1, 4, 5);
    let c4=Cell::cell_data(1, 5, 6);
    let r2=Row::new_row(vec![c3,c4]);
    let mut table=Table::create_table(vec![r1,r2]);
    println!("{:#?}",table);
    table.update_data(1, 1, 1,5);
    println!("{:#?}",table);

}


