struct Area{
    length:f32,
    breadth:f32,
}
impl Area{
    fn Area_of_Rectangle(&self)->f32{
        self.length*self.breadth        
    }
    fn Area_of_Circle(&self)->f32{
        if self.length>=self.breadth{
            3.14*(self.breadth/2.0)*(self.breadth/2.0)
        }
        else{
            3.14*(self.length/2.0)*(self.length/2.0)
        }
    }
    fn Area_of_Square(&self)->f32{
        if self.length>=self.breadth{
            self.breadth*self.breadth
        }
        else{
            self.length*self.length
        }        
    }
}

fn main(){
    let instance_of_area=Area{
        length:20.0,
        breadth:10.0
    };

    println!("Area of Circle {}",instance_of_area.Area_of_Circle());
    println!("Area of Rectangle {}",instance_of_area.Area_of_Rectangle());
    println!("Area of Square {}",instance_of_area.Area_of_Square());



}