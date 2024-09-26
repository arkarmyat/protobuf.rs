use tonic::{transport::Server, Request, Response, Status};
use std::error::Error;

// from proto package
use greeting::greeter_server::{Greeter, GreeterServer};
use greeting::{HelloReply, HelloRequest};
use employee::employee_server::{Employee, EmployeeServer};
use employee::{EmployeeRequest, EmployeeResponse};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}


#[derive(Debug, Default)]
pub struct MyEmployee {}

#[tonic::async_trait]
impl Employee for MyEmployee {
    async fn get_employee(
        &self,
        request: Request<EmployeeRequest>,
    ) -> Result<Response<EmployeeResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = EmployeeResponse {
            id: 1.to_string(),
            name:String::from("Apple")
        };

        Ok(Response::new(reply))
    }
}

pub mod greeting {
    tonic::include_proto!("greeting"); // The string specified here must match the proto package name
}

pub mod employee {
    tonic::include_proto!("employee"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let employee = MyEmployee::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(EmployeeServer::new(employee))
        .serve(addr)
        .await?;
    Ok(())
}
