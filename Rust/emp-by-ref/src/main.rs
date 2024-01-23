use std::fs::{read_to_string,write};

use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize,Serialize)]
struct Employee{
    name:String,
    age:u8,
    skills:Vec<String>,
    position:Option<String>,
    #[serde(rename="experiance(y)")]
    experiance:Option<u8>
}
fn main() {
    let content=read_to_string("JSON-Data/Employee.json");
    
    let mut mid_rust:Vec<&Employee>=Vec::new();
    let mut jr_java:Vec<&Employee>=Vec::new();
    let mut sen_or_c:Vec<&Employee>=Vec::new();

    //println!("{:?}",content);
    match content{
        Ok(data)=>{
            let deserialize_data:Vec<Employee>=serde_json::from_str(&data).expect("Data doesn't deserialized");
            
            for each_emp in &deserialize_data{
                if each_emp.position==Some("Software Developer".to_string()) && each_emp.skills.contains(&"Rust".to_string()){
                    mid_rust.push(each_emp);
                }
                else if each_emp.position==Some("Jr. Software Developer".to_string()) && each_emp.skills.contains(&"Java".to_string()){
                    jr_java.push(each_emp);
                }
                else if each_emp.position==Some("Sr. Software Developer".to_string()) || each_emp.skills.contains(&"C".to_string()){
                    sen_or_c.push(each_emp);
                }
            }
        

                
            

            let serialize_data1=serde_json::to_string_pretty(&mid_rust).expect("Serialize Issue");
            let serialize_data2=serde_json::to_string_pretty(&jr_java).expect("Serialize Issue");
            let serialize_data3=serde_json::to_string_pretty(&sen_or_c).expect("Serialize Issue");

            write("JSON-Data/Task1.json",serialize_data1);
            write("JSON-Data/Task2.json",serialize_data2);
            write("JSON-Data/Task3.json",serialize_data3);


            
        }
        Err(err)=>{
            println!("Unable to read File : {}",err);
        }
    }
}

