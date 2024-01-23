use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

#[derive(Deserialize,Debug,Serialize)]

struct Student{
    name:String,
    phone:String,
    email:String,
    city:String,
    address:String,
    marks:Vec<u16>,
    total_marks:Option<u16>,
    percentage:Option<f32>,
    grade:Option<String>
}

fn main() {
    let content=read_to_string("C:/Users/ASUS/Downloads/StudentData.json");
    match content{
        Ok(data)=>{
            let mut students_data:Vec<Student> = serde_json::from_str(&data).expect("Wrong Data");
            for student in &mut students_data{
                let total_marks:u16=student.marks.iter().sum();
                let total_subject=student.marks.iter().count();
                student.total_marks=Some(total_marks);
                student.percentage=Some(total_marks as f32/total_subject as f32);
                student.grade=
                    if student.percentage>=Some(90.0){
                        Some("A1".to_string())
                    }
                    else if student.percentage>=Some(80.0){
                        Some("A2".to_string())
                    }
                    else if student.percentage>=Some(70.0){
                        Some("B1".to_string())
                    }
                    else if student.percentage>=Some(60.0){
                        Some("B2".to_string())
                    }
                    else if student.percentage>=Some(50.0){
                        Some("C1".to_string())
                    }
                    else if student.percentage>=Some(40.0){
                        Some("C2".to_string())
                    }
                    else if student.percentage>=Some(35.0){
                        Some("D".to_string())
                    }
                    else{
                        Some("F".to_string())
                    }
            } 



            let updated_student_data = match serde_json::to_string_pretty(&students_data) {
                Ok(data) =>data,
                Err(e) => {
                    eprintln!("Failed to serialize JSON: {}", e);
                    return;
                }
            };
            write("C:/Users/ASUS/Downloads/new.json",updated_student_data);

        }

        Err(err)=>{
            println!("Unable to read json file {}",err);
        }
    }
}
