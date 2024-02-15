

pub fn emp_sorting(){
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
                else if each_emp.position == Some(Position::Sr) && each_emp.skills.contains(&Skills::CS){
                    sen_or_c.push(each_emp);
                }
            }

            let serialize_data1
            =serde_json::to_string_pretty(&mid_rust).expect("Serialize Issue");
            let serialize_data2=serde_json::to_string_pretty(&jr_java).expect("Serialize Issue");
            let serialize_data3=serde_json::to_string_pretty(&sen_or_c).expect("Serialize Issue");

            write("JSON-data/Task1.json",serialize_data1);
            write("JSON-data/Task2.json",serialize_data2);
            write("JSON-data/Task3.json",serialize_data3);        
        }
        Err(err)=>{
            println!("Unable to read File : {}",err);
        }
    }
}



#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, result};

    use crate::common::Employee;

    #[test]
    fn task1_check(){
    let output=read_to_string("JSON-data/Task1.json").unwrap();
    // let task_data:Vec<Employee>=serde_json::from_str(&content).expect("fail");
    let my_result=r#"[
        {"skills": [
            "Rust",
            "C#"
          ],
          "position": "Software Developer",
          "name": "Arjun",
          "age": 27
        },
        {
          "position": "Software Developer",
          "skills": [
            "Rust",
            "Java"
          ],
          "age": 26,
          "name": "Ishaan"
        }
      ]"#;
    // let serialize_data1=serde_json::to_string_pretty(task_data).unwrap();

        assert_eq!(my_result,output);


    }
}