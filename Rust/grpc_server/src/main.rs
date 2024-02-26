use tonic::transport::Server;

use crate::{employee::{employees::employees_server::EmployeesServer, EmployeeService}, student::{students::students_server::StudentsServer, StudentService}};

mod student;
mod employee;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    println!("{:?}",addr);
    let student_service  =StudentService::default();
    let emp_service  =EmployeeService::default();

    Server::builder()
        .add_service(StudentsServer::new(student_service))
        .add_service(EmployeesServer::new(emp_service))
        .serve(addr)
        .await?;

    Ok(())
}