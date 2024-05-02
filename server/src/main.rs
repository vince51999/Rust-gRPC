use std::{
    sync::{Arc, Mutex},
    net::SocketAddr,
};
use tonic::{transport::Server, Request, Response, Status};
use rand::Rng;

// Import the generated product module
use product::{Empty, ProductSnResponse, ProductPriceResponse, product_server::{Product, ProductServer}};
pub mod product {
    tonic::include_proto!("product");
}

// Define the product data structure
#[derive(Default)]
struct ProductData {
    price: i32,
    sn: i32,
}

// Define a struct to hold the product data
#[derive(Default)]
pub struct ProductImpl {
    data: Arc<Mutex<ProductData>>,
}

impl ProductImpl {
    // Constructor to create a new instance with default values
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(ProductData::default())),
        }
    }

    // Method to set the price to a new value
    pub fn set_price(&mut self, price: i32) {
        let mut data = self.data.lock().unwrap();
        data.price = price;
    }

    // Method to set the serial number to a new value
    pub fn set_sn(&mut self, sn: i32) {
        let mut data = self.data.lock().unwrap();
        data.sn = sn;
    }

    // Spawn a background task to update the product data
    pub fn start_updating(&self) {
        let data = Arc::clone(&self.data);
        tokio::spawn(async move {
            loop {
                let price;
                let sn;
                
                {
                    let mut product_data = data.lock().unwrap();
                    // Generate a random integer price between 10 and 200
                    price = rand::thread_rng().gen_range(10..=200);
                    // Generate a random integer serial number between 0 and 300
                    sn = rand::thread_rng().gen_range(0..=300);
                    
                    // Update the product data
                    product_data.price = price;
                    product_data.sn = sn;
                    println!("{}, {}", sn, price);
                }
                
                // Sleep for some time before the next update
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        });
    }
}

#[tonic::async_trait]
impl Product for ProductImpl {
    async fn get_price(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ProductPriceResponse>, Status> {
        // Return the price from the product data
        let data = Arc::clone(&self.data);
        let product_data = data.lock().unwrap();
        let response = ProductPriceResponse { price: product_data.price };

        Ok(Response::new(response))
    }

    async fn get_sn(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ProductSnResponse>, Status> {
        // Return the serial number from the product data
        let data = Arc::clone(&self.data);
        let product_data = data.lock().unwrap();
        let response = ProductSnResponse { sn: product_data.sn };

        Ok(Response::new(response))
    }
}

// Import the generated offer module
use offer::{OfferRequest, OfferResponse, offer_server::{Offer, ProductServer}};
pub mod offer {
tonic::include_proto!("offer");
}
  
// Define a struct to hold the product data
#[derive(Default)]
pub struct OfferImpl {
    data: ProductImpl,
}

#[tonic::async_trait]
impl Offer for OfferImpl {
    // take with reference product_impl

    pub fn set_product(&mut self, data: &ProductImpl) {
        self.product = data;
        
    }


    async fn ConfirmOffer(
        &self,
        request: Request<OfferRequest>,
    ) -> Result<Response<OfferResponse>, Status> {
        // Return the price from the product data
        let price = request.into_inner().price;
        let sn = request.into_inner().sn;
        let response = OfferResponse { offer: price + sn };

        Ok(Response::new(response))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr: SocketAddr = "[::1]:8080".parse().unwrap();

  // Create an instance of ProductImpl
  let product_impl = ProductImpl::new();
  // Start updating the product data in a separate thread
  product_impl.start_updating();

// Create an instance of ProductImpl
let offer_impl = OfferImpl::new();
offer_impl.set_product(&product_impl);



  println!("Rust gRPC server listening on {}", addr);

  // Serve the GPRC server
  Server::builder()
      .add_service(ProductServer::new(product_impl))
      .serve(addr)
      .await?;

  Ok(())
}

// https://www.thorsten-hans.com/grpc-services-in-rust-with-tonic/