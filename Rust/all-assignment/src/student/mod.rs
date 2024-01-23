
pub fn students_result(){
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
        
                    let updated_data =serde_json::to_string_pretty(&students).expect("msg");
                    write("JSON-Data/StudentData.json", updated_data);
                    println!("Data appended to the file.");
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}


