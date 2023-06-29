use std::{thread::sleep, time::Duration};

use service::{apis_client::ApisClient, TestRequest};

pub mod service {
    tonic::include_proto!("service");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ApisClient::connect("http://[::1]:1370").await?;

    loop {
        let request = tonic::Request::new(TestRequest {
            value: "Rust User".into(),
        });

        let response = client.test(request).await?;

        println!("{:?}", response);

        sleep(Duration::from_secs(1));
    }

    Ok(())
}
