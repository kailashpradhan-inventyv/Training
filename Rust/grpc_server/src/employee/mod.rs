use std::sync::Arc;

use tonic::{Request, Response, Status};

use crate::employee::model::{EmpData, EMP_DATA};

use self::employees::{employees_server::Employees, EmpGetId, EmpResp, GrpcEmpData};


pub mod model;
pub mod employees{
    tonic::include_proto!("employees");
}
#[derive(Debug,Default)]
pub struct EmployeeService{}
#[tonic::async_trait]
impl Employees for EmployeeService{
    async fn get_data(&self,request:Request<EmpGetId>) -> Result<Response<EmpResp>,Status>{
        println!("Got a request: {:?}", request);
        let read_data=Arc::clone(&EMP_DATA);
        let req=request.into_inner().id;
        let reply;
        let emp_data;
        if let Some(emp_id)=read_data.read().unwrap().iter().position(|emp|emp.id==req){
            emp_data=serde_json::to_string(&read_data.read().unwrap().get(emp_id)).unwrap();
    
        }
        else{
            emp_data="None".to_string();
        }
        reply = EmpResp {
            data:emp_data,
            message:"Done".to_string(),
            key:5000
        };
        
        Ok(Response::new(reply))
    }
    async fn del_data(&self,request:Request<EmpGetId>) -> Result<Response<EmpResp>,Status>{
        println!("Got a request: {:?}", request);
        let read_data=Arc::clone(&EMP_DATA);
        let mut write_access=read_data.write().unwrap();
        let req=request.into_inner().id;
        let reply;

        if let Some(pos)=write_access.iter().position(|emp|emp.id==req){
            write_access.remove(pos);
            reply = EmpResp {
                data:"Data Deleted".to_string(),
                message:"Done".to_string(),
                key:5000
            };
        }
        else {
            reply = EmpResp {
                data:"Not Found Data".to_string(),
                message:"Fail".to_string(),
                key:5000
            };
        }
        
        Ok(Response::new(reply))

    }async fn update_data(&self,request:Request<GrpcEmpData>) -> Result<Response<EmpResp>,Status>{
        println!("Got a request: {:?}", request);
        let read_data=Arc::clone(&EMP_DATA);
        let mut write_access=read_data.write().unwrap();
        let req=request.into_inner();
        let reply;
        let emp_data;
        if let Some(emp_id)=write_access.iter().position(|emp|emp.id==req.id){
            // write_access.remove(emp_id);
            // emp_data="Data Deleted".to_string();
            emp_data=serde_json::to_string(&read_data.read().unwrap().get(emp_id)).unwrap();
            let upd_data=EmpData{
                id:req.id,
                name:req.name,
                age:req.age,
                position:Some(req.position),
                experiance:Some(req.experiance),
                skills:req.skills
            };
            write_access[emp_id]=upd_data;
            reply = EmpResp {
                data:emp_data,
                message:"Done".to_string(),
                key:5000
            };
    
        }
        else{
            emp_data="ID Not Found".to_string();
            reply = EmpResp {
                data:emp_data,
                message:"Fail".to_string(),
                key:5000
            };
        }
        
        Ok(Response::new(reply))


    }async fn new_data(&self,request:Request<GrpcEmpData>) -> Result<Response<EmpResp>,Status>{
        println!("Got a request: {:?}", request);

        let read_data=Arc::clone(&EMP_DATA);
        let mut write_access=read_data.write().unwrap();
        let req=request.into_inner();
        let reply;
        
        if let Some(emp_id)=write_access.iter().position(|emp|emp.id==req.id){
            // write_access.remove(emp_id);
            // emp_data="Data Deleted".to_string();
            reply = EmpResp {
                data:"ID already exist".to_string(),
                message:"Fail".to_string(),
                key:5000
            };
        }
        else{
            let upd_data=EmpData{
                id:req.id,
                name:req.name,
                age:req.age,
                position:Some(req.position),
                experiance:Some(req.experiance),
                skills:req.skills
            };
            write_access.push(upd_data.clone());
            reply = EmpResp {
                data:serde_json::to_string(&upd_data).unwrap(),
                message:"Done".to_string(),
                key:5000
            };
        }
        
        Ok(Response::new(reply))


    }
}