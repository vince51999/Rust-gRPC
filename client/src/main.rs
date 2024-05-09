use tonic::transport::Channel;
use rand::Rng;

use product::{product_client::ProductClient, Empty};
pub mod product {
    tonic::include_proto!("product");
}
use offer::{offer_client::OfferClient, OfferRequest};
pub mod offer {
  tonic::include_proto!("offer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Establish a connection to the server
  let channel = Channel::from_static("http://[::1]:8080")
  .connect()
  .await?;

  // Create a gRPC client services
  let mut client_product = ProductClient::new(channel.clone());
  let mut client_offer = OfferClient::new(channel.clone());

  let purchases = 10;
  let mut count = 0;

  while count < purchases {
    let price = (client_product.get_price(Empty {}).await?).into_inner().price;
    let sn = (client_product.get_sn(Empty {}).await?).into_inner().sn;
    println!("Recived product with sn: {:?} and price: {:?}", sn, price);
    
    let new_price = rand::thread_rng().gen_range(10..=200);
    let offer_request = OfferRequest {
      price: ( new_price ) as i32,
      sn: sn,
    };
    let confirmed = (client_offer.confirm_offer(offer_request).await?).into_inner().confirmed;
    println!("Server response: {:?} for offer: {:?}\n", confirmed, new_price);
    if confirmed {
      count += 1;
      println!("Purchases: {:?} / {:?}\n", count, purchases);
    }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
  }
  
  Ok(())
}