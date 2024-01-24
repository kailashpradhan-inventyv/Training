use std::collections::HashMap;

use serde_json::{json, Value};

use crate::common::Employee;



pub fn emp_update_hashmap(){
    use std::fs::{read_to_string,write};
    use crate::common::Employee;
    use crate::common::Position;
    use crate::common::Skills;
    let content=read_to_string("JSON-data/Employee.json");
    let mut mid_rust:Vec<&Employee>=Vec::new();
    let mut jr_java:Vec<&Employee>=Vec::new();
    let mut sen_or_c:Vec<&Employee>=Vec::new();

    match content{
        Ok(data)=>{
            let deserialize_data:Vec<Employee>=serde_json::from_str(&data).expect("Data doesn't deserialized");
            
            for each_emp in &deserialize_data{
                if each_emp.position == Some(Position::Mid) && each_emp.skills.contains(&Skills::Rust){
                    mid_rust.push(each_emp);
                }   
                else if each_emp.position == Some(Position::Jr) && each_emp.skills.contains(&Skills::Java){
                    jr_java.push(each_emp);
                }
                else if each_emp.position == Some(Position::Sr) || each_emp.skills.contains(&Skills::CS){
                    sen_or_c.push(each_emp);
                }
            }

            let mut mid_hashmap=Vec::new();
            mid_hashmap=struct_to_map(mid_rust);


            let mut jr_hashmap=Vec::new();
            jr_hashmap=struct_to_map(jr_java);
            

            let mut sen_c_hashamp=Vec::new();
            sen_c_hashamp=struct_to_map(sen_or_c);
            
            let serialize_data1=serde_json::to_string_pretty(&mid_hashmap).expect("Serialize Issue");
            let serialize_data2=serde_json::to_string_pretty(&jr_hashmap).expect("Serialize Issue");
            let serialize_data3=serde_json::to_string_pretty(&sen_c_hashamp).expect("Serialize Issue");

            write("JSON-data/Task1.json",serialize_data1);
            write("JSON-data/Task2.json",serialize_data2);
            write("JSON-data/Task3.json",serialize_data3);        
        }
        Err(err)=>{
            println!("Unable to read File : {}",err);
        }
    }
}

fn struct_to_map(a:Vec<&Employee>)->Vec<HashMap<&str,Value>>{
    let mut vec_map=Vec::new();
    for i in 0..a.len(){
        let mut each_emp: HashMap<&str, Value> =HashMap::new();
        each_emp.insert("name", json!(a[i].name));
        each_emp.insert("age", json!(a[i].age));
        each_emp.insert("skills", json!(a[i].skills));
        if !a[i].position.is_none(){
            each_emp.insert("position", json!(a[i].position));
        }
        if !a[i].experiance.is_none(){
            each_emp.insert("experiance", json!(a[i].experiance));
        }
        vec_map.push(each_emp);
    }
    vec_map
}