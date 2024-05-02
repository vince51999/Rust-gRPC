use tonic::transport::Channel;
use rand::Rng;
use product::{product_client::ProductClient, Empty};
use offer::{offer_client::OfferClient, OfferRequest};

pub mod product {
    tonic::include_proto!("product");
}
pub mod offer {
  tonic::include_proto!("offer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Establish a connection to the server
  let channel = Channel::from_static("http://[::1]:8080")
  .connect()
  .await?;

    // Create a gRPC client
    let mut clientProduct = ProductClient::new(channel.clone());
    let mut clientOffer = OfferClient::new(channel.clone());
    loop {
      // Make gRPC calls
      let price = (clientProduct.get_price(Empty {}).await?).into_inner().price; // Call GetPrice with an Empty request
      let sn = (clientProduct.get_sn(Empty {}).await?).into_inner().sn; // Call GetSN with an Empty request
      println!("Recived product with sn: {:?} and price: {:?}", sn, price);
      
      let new_price = rand::thread_rng().gen_range(10..=200);
      let offer_request = OfferRequest {
        price: ( new_price ) as i32,
        sn: sn,
      };
      let confirmed = (clientOffer.confirm_offer(offer_request).await?).into_inner().confirmed; // Call ConfirmOffer with an OfferRequest
      println!("Server response: {:?} for offer: {:?}\n", confirmed, new_price);
      tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    Ok(())
}

// https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/