// struct Area{
//     length:f32,
//     breadth:f32,
// }
// impl Area{
//     fn area_of_rectangle(&self)->f32{
//         self.length*self.breadth        
//     }
//     fn area_of_circle(&self)->f32{
//         if self.length>=self.breadth{
//             3.14*(self.breadth/2.0)*(self.breadth/2.0)
//         }
//         else{
//             3.14*(self.length/2.0)*(self.length/2.0)
//         }
//     }
//     fn area_of_square(&self)->f32{
//         if self.length>=self.breadth{
//             self.breadth*self.breadth
//         }
//         else{
//             self.length*self.length
//         }        
//     }
// }

// fn main(){
//     let instance_of_area=Area{
//         length:20.0,
//         breadth:10.0
//     };

//     println!("Area of Circle {}",instance_of_area.area_of_circle());
//     println!("Area of Rectangle {}",instance_of_area.area_of_rectangle());
//     println!("Area of Square {}",instance_of_area.area_of_square());
// }


use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

#[derive(Debug, Deserialize, Serialize)]
struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    marks: Vec<u32>,
    #[serde(default)]
    percentage: Option<f32>,
    #[serde(default)]
    grade: Option<String>,
}

fn main() {
    // Read the JSON file
    let file_content = read_to_string("D:/Inventyv_Training/Training/rust-training/src/StudentData.json");
    match file_content {
        Ok(content) => {
            let mut students: Vec<Student> = serde_json::from_str(&content).expect("Failed to parse JSON");

            // Calculate percentage and grade for each student
            for student in &mut students {
                let total_score: u32 = student.marks.iter().sum();
                let calculated_percentage = (total_score as f32 / (student.marks.len() as f32 * 100.0)) * 100.0;

                student.percentage = Some(calculated_percentage);

                student.grade = if calculated_percentage >= 90.0 {
                    Some("A".to_string())
                } else if calculated_percentage >= 80.0 {
                    Some("B".to_string())
                } else if calculated_percentage >= 70.0 {
                    Some("C".to_string())
                } else if calculated_percentage >= 60.0 {
                    Some("D".to_string())
                } else {
                    Some("F".to_string())
                };
            }

            // Update the JSON file with calculated data
            // let serialized = serde_json::to_string_pretty(&students).unwrap();
            // write("D:/Inventyv_Training/Training/rust-training/src/StudentData.json", serialized).expect("Failed to write JSON");
            // println!("Data appended to the file.");


            // Update the JSON file with calculated data
            let updated_data = match serde_json::to_string_pretty(&students) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Failed to serialize JSON: {}", e);
                    return;
                }
            };

            if let Err(e) = write("D:/Inventyv_Training/Training/rust-training/src/StudentData.json", updated_data) {
                eprintln!("Failed to write JSON: {}", e);
                return;
            }

            println!("Data appended to the file.");
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}

