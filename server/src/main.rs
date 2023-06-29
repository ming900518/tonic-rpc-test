use service::{
    apis_server::{Apis, ApisServer},
    TestRequest, TestResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod service {
    tonic::include_proto!("service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:1370".parse()?;
    let apis = MyApis::default();

    Server::builder()
        .add_service(ApisServer::new(apis))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MyApis {}

#[tonic::async_trait]
impl Apis for MyApis {
    async fn test(&self, request: Request<TestRequest>) -> Result<Response<TestResponse>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        println!("{:?}", request);

        let reply = TestResponse {
            return_value: format!("Hello {}!", request.value),
        };

        Ok(Response::new(reply))
    }
}
