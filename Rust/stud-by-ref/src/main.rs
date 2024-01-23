use serde::{Deserialize, Serialize};
use std::{fs::{read_to_string, write}};

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    marks: Vec<u32>,
    percentage: Option<f32>,
    grade: Option<String>,
}

fn main() {
    let file_content = read_to_string("./JSON-Data/StudentData.json");
    match file_content {
        Ok(content) => {
            let mut students: Vec<Student> = serde_json::from_str(&content).expect("Error");
           
                
                    for student in &mut students {
                        let total_score: u32 = student.marks.iter().sum();
                        let calculated_percentage = (total_score as f32 / (student.marks.len() as f32 * 100.0)) * 100.0;
        
                        student.percentage = Some(calculated_percentage);
        
                        // student.grade = if calculated_percentage >= 90.0 {
                        //     Some("A".to_string())
                        // } else if calculated_percentage >= 80.0 {
                        //     Some("B".to_string())
                        // } else if calculated_percentage >= 70.0 {
                        //     Some("C".to_string())
                        // } else if calculated_percentage >= 60.0 {
                        //     Some("D".to_string())
                        // } else {
                        //     Some("F".to_string())
                        // };

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
        
                    let updated_data =serde_json::to_string_pretty(&students).expect("msg");
                       
        

                    write("./JSON-Data/StudentData.json", updated_data);
        
                    println!("Data appended to the file.");

                
            

        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

