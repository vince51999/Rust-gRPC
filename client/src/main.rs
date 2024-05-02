use tonic::transport::Channel;
use product::{product_client::ProductClient, Empty};

pub mod product {
    tonic::include_proto!("product");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Establish a connection to the server
  let channel = Channel::from_static("http://[::1]:8080")
  .connect()
  .await?;

    // Create a gRPC client
    let mut client = ProductClient::new(channel);
    loop {
      // Make gRPC calls
      let price_response = client.get_price(Empty {}).await?; // Call GetPrice with an Empty request
      println!("Price received: {:?}", price_response.into_inner().price);

      let sn_response = client.get_sn(Empty {}).await?; // Call GetSN with an Empty request
      println!("Serial number received: {:?}", sn_response.into_inner().sn);
      tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    Ok(())
}

// https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/