use std::collections::{ HashMap};


use serde_json::{json, Value};


pub fn students_result_hashmap(){
    use std::fs::{read_to_string,write};
    use crate::common::Student;
    let file_content = read_to_string("JSON-data/StudentData.json");
    match file_content {
        Ok(content) => {
            let mut students: Vec<Student> = serde_json::from_str(&content).expect("Error"); 
                    for student in &mut students {
                        let total_score: u32 = student.marks.iter().sum();
                        let calculated_percentage = (total_score as f32 / (student.marks.len() as f32 * 100.0)) * 100.0;
        
                        student.percentage = Some(calculated_percentage);
        
                        match calculated_percentage {
                            90.0..=100.0=>{
                                student.grade=Some("A1".to_string());
                            }
                            80.0..=90.0=>{
                                student.grade=Some("A2".to_string());
                            }
                            70.0..=80.0=>{
                                student.grade=Some("B1".to_string());
                            }
                            60.0..=70.0=>{
                                student.grade=Some("B2".to_string());
                            }
                            50.0..=60.0=>{
                                student.grade=Some("C1".to_string());
                            }
                            40.0..=50.0=>{
                                student.grade=Some("C2".to_string());
                            }
                            35.0..=40.0=>{
                                student.grade=Some("D".to_string());
                            }
                            _=>{
                                student.grade=Some("F".to_string());
                            }
                            
                        }
                    }

                    
                    let mut vec_hash: Vec<HashMap<&str, Value>>=Vec::new();
                    for i in 0..students.len(){
                        let mut each_stud: HashMap<&str, Value>=HashMap::new();
                        each_stud.insert("name", Value::String(students[i].name.to_string()));
                        each_stud.insert("phone", Value::String(students[i].phone.to_string()));
                        each_stud.insert("email", Value::String(students[i].email.to_string()));
                        each_stud.insert("city", Value::String(students[i].city.to_string()));
                        each_stud.insert("address", Value::String(students[i].address.to_string()));
                        each_stud.insert("marks", json!(students[i].marks));
                        each_stud.insert("percentage", json!(students[i].percentage));
                        each_stud.insert("grade", json!(students[i].grade));
                        vec_hash.push(each_stud);             

                    }
                    println!("{:?}",vec_hash);


        
                    let updated_data =serde_json::to_string_pretty(&vec_hash).expect("msg");
                    write("JSON-Data/StudentData.json", updated_data);
                    println!("Data appended to the file.");
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}


