use std::sync::Arc;

use tonic::{Request,Response,Status};
mod model;
pub mod students {
    tonic::include_proto!("students");
}
use students::students_server::{Students,StudentsServer};
use students::{StudGetId,StudResp,GrpcStudData};

use crate::student::model::{StudData, STUD_DATA};

#[derive(Debug, Default)]
pub struct StudentService {}

#[tonic::async_trait]
impl Students for StudentService {
    async fn get_data(&self,request:Request<StudGetId>) -> Result<Response<StudResp>,Status>{
        println!("Got a request: {:?}", request);
        let read_stud_data=Arc::clone(&STUD_DATA);
        
        let req=request.into_inner();
        let stud_data;
        if let Some(stu_id) = read_stud_data.read().unwrap().iter().position(|stud|stud.id==req.id){
            stud_data=serde_json::to_string(&read_stud_data.read().unwrap().get(stu_id)).unwrap();

        }else{
            stud_data="None".to_string();
        }

        let reply = StudResp {
            data:stud_data,
            message:"Done".to_string(),
            key:5000
        };

        Ok(Response::new(reply))
    }

    async fn del_data(&self,request:Request<StudGetId>) -> Result<Response<StudResp>,Status>{
        println!("Got a request: {:?}", request);
        
        let read_stud_data=Arc::clone(&STUD_DATA);

        let mut write_lock=read_stud_data.write().unwrap();
        let req=request.into_inner();
        let reply;
        if let Some(pos)=write_lock.iter().position(|stud|stud.id==req.id){
            write_lock.remove(pos);
            reply = StudResp {
                data:"Data Deleted".to_string(),
                message:"Done".to_string(),
                key:5000
            };
        }
        else {
            reply = StudResp {
                data:"Not Found Data".to_string(),
                message:"Fail".to_string(),
                key:5000
            };
        }
        

        Ok(Response::new(reply))
    }

    
    async fn update_data(&self,request:Request<GrpcStudData>) -> Result<Response<StudResp>,Status>{
        println!("Got a request: {:?}", request);
        
        let read_stud_data=Arc::clone(&STUD_DATA);

        let mut write_lock=read_stud_data.write().unwrap();
        let req=request.into_inner();
        // request.into_inner().
        let reply;
        if let Some(pos)=write_lock.iter().position(|stud|stud.id==req.id){
            // write_lock.remove(pos);
            let total:i32= req.marks.iter().sum();
            let percentage=total as f32/req.marks.iter().count() as f32;
            let per=Some(percentage);
            let grade;
            match percentage{
                90.0..=100.0=>{
                    grade=Some("A1".to_string());
                }
                80.0..=90.0=>{
                    grade=Some("A2".to_string());
                }70.0..=80.0=>{
                    grade=Some("B1".to_string());
                }60.0..=70.0=>{
                    grade=Some("B2".to_string());
                }50.0..=60.0=>{
                    grade=Some("C1".to_string());
                }40.0..=50.0=>{
                    grade=Some("C2".to_string());
                }
                _=>{
                    grade=Some("F".to_string());
                }
            }
            let upd_data=StudData{
                id:req.id,
                name:req.name,
                phone:req.phone,
                email:req.email,
                city:req.city,
                address:req.address,
                marks:req.marks,
                percentage:per,
                grade
            };
            write_lock[pos]=upd_data;
            reply = StudResp {
                data:serde_json::to_string(&write_lock.get(pos)).unwrap(),
                message:"Good".to_string(),
                key:5000
            };
        }
        else {
            reply = StudResp {
                data:"Not Found Data".to_string(),
                message:"Fail".to_string(),
                key:5000
            };
        }
        Ok(Response::new(reply))
    }
    

    async fn new_data(&self,request:Request<GrpcStudData>) -> Result<Response<StudResp>,Status>{
        println!("Got a request: {:?}", request);
        
        let read_stud_data=Arc::clone(&STUD_DATA);

        let mut write_lock=read_stud_data.write().unwrap();
        let req=request.into_inner();
        // request.into_inner().
        let reply;
        if write_lock.iter().any(|stud|stud.id==req.id){
            // write_lock.remove(pos);
            reply = StudResp {
                data:"Duplicate ID".to_string(),
                message:"Fail".to_string(),
                key:5000
            };
        }
        else {
           


            let total:i32= req.marks.iter().sum();
            // let per=Some(total as f32/req.marks.iter().count() as f32);
            // let grade=Some("A".to_string());
            let percentage=total as f32/req.marks.iter().count() as f32;
            let per=Some(percentage);
            let grade;
            match percentage{
                90.0..=100.0=>{
                    grade=Some("A1".to_string());
                }
                80.0..=90.0=>{
                    grade=Some("A2".to_string());
                }70.0..=80.0=>{
                    grade=Some("B1".to_string());
                }60.0..=70.0=>{
                    grade=Some("B2".to_string());
                }50.0..=60.0=>{
                    grade=Some("C1".to_string());
                }40.0..=50.0=>{
                    grade=Some("C2".to_string());
                }
                _=>{
                    grade=Some("F".to_string());
                }
            }
            let n_data=StudData{
                id:req.id,
                name:req.name,
                phone:req.phone,
                email:req.email,
                city:req.city,
                address:req.address,
                marks:req.marks,
                percentage:per,
                grade
            };
            // write_lock[pos]=upd_data;
            write_lock.push(n_data.clone());
            reply = StudResp {
                data:serde_json::to_string(&n_data).unwrap(),
                message:"Success".to_string(),
                key:5000
            };
        }
        Ok(Response::new(reply))
    }
    

}


