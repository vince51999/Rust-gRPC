use tonic::transport::Channel;
use rand::Rng;

use product::{product_client::ProductClient, Empty, ProductSnRequest};
pub mod product {
    tonic::include_proto!("product");
}
use offer::{offer_client::OfferClient, OfferRequest};
pub mod offer {
  tonic::include_proto!("offer");
}
use subscribe::{subscribe_client::SubscribeClient, SubscribeRequest, UnsubscribeRequest};
pub mod subscribe {
    tonic::include_proto!("subscribe");
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
  let mut client_subscribe = SubscribeClient::new(channel.clone());

  let subscribe_response = client_subscribe.subscribe(SubscribeRequest {}).await?;
    if !subscribe_response.into_inner().success {
        println!("Subscription failed");
        return Ok(());
    }
    println!("Subscribed successfully");

  let purchases = 10;
  let mut count = 0;

  while count < purchases {
    let sn_list = (client_product.get_products_sn(Empty {}).await?).into_inner().sn_list;

    let sn = sn_list[rand::thread_rng().gen_range(0..sn_list.len())];
    let price = (client_product.get_price(ProductSnRequest { sn: sn }).await?).into_inner().price;

    println!("The price for product with sn: {:?} is: {:?}$", sn, price);
    
    let new_price = rand::thread_rng().gen_range(10..=200);
    let offer_request = OfferRequest {
      price: ( new_price ) as i32,
      sn: sn,
    };
    let confirmed = (client_offer.confirm_offer(offer_request).await?).into_inner().confirmed;
    println!("Server response: {:?} for offer: {:?}$\n", confirmed, new_price);
    if confirmed {
      count += 1;
      println!("Purchases: {:?} / {:?}\n", count, purchases);
    }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
  }

  let subscribe_response = client_subscribe.unsubscribe(UnsubscribeRequest {}).await?;
  if !subscribe_response.into_inner().success {
    println!("Unsubscription failed");
    return Ok(());
  }
  
  
  Ok(())
}