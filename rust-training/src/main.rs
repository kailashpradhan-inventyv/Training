struct Area{
    length:f32,
    breadth:f32,
    radius:f32
}
impl Area{
    fn Area_of_Rectangle(&self)->f32{
        self.length*self.breadth        
    }
    fn Area_of_Circle(&self)->f32{
        3.14*self.radius*self.radius        
    }
    fn Area_of_Square(&self)->f32{
        self.length*self.length        
    }
}

fn main(){
    let instance_of_area=Area{
        length:10.0,
        breadth:20.0,
        radius:2.0
    };

    println!("Area of Circle {}",instance_of_area.Area_of_Circle());
    println!("Area of Rectangle {}",instance_of_area.Area_of_Rectangle());
    println!("Area of Square {}",instance_of_area.Area_of_Square());



}