/**
 * This is the main file for the Rust gRPC server.
 */

/**
 * Import the Arc and Mutex types for thread-safe reference counting
 * - Arc: is an atomic reference-counted pointer
 * - Mutex: is a mutual exclusion primitive
 * - net: module for networking
 */
use std::{ sync::{Arc, Mutex}, net::SocketAddr };
/**
 * Import the Server type to create a gRPC server
 * Import the Request, Response, and Status types for handling requests and responses
 */
use tonic::{ transport::Server, Request, Response, Status};
use rand::Rng;
use lazy_static::lazy_static;

extern crate chrono;
use chrono::Local;

// Import the generated product module
use product::{ Empty, ProductSnRequest, ProductsSnResponse, ProductPriceResponse, product_server::{Product, ProductServer} };
pub mod product {
    tonic::include_proto!("product");
}

// Define the product data structure
#[derive(Default)]
struct ProductData {
    price: i32,
    sn: i32,
}

/*
 Define a global variable for product data
 - lazy_static: is used to create a global variable that is initialized lazily when it is accessed for the first time
 */
lazy_static! {
    static ref PRODUCTS: Arc<Mutex<Vec<ProductData>>> = Arc::new(Mutex::new(Vec::new()));
    static ref SUBSCRIPTION_COUNT: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

// Define a struct to implement the Product service
pub struct ProductImpl;

impl ProductImpl {
    pub fn new() -> Self {
        Self {}
    }
}
/**
 * Implement the remote service trait for the Product service
 */
#[tonic::async_trait]
impl Product for ProductImpl {
    async fn get_price(
        &self,
        _request: Request<ProductSnRequest>,
    ) -> Result<Response<ProductPriceResponse>, Status> {
        loop {
            let count = {
                let count_lock = SUBSCRIPTION_COUNT.lock().unwrap();
                *count_lock
            };
            if count < 3 {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            } else {
                break;
            }
        }
        let sn = _request.get_ref().sn;
        let mut prod_price = 0;
        {
            let products = PRODUCTS.lock().unwrap();
            if sn < 0 || sn >= products.len() as i32 {
                return Err(Status::invalid_argument("Invalid serial number"));
            }
            prod_price = products[sn as usize].price;
        }
        let response = ProductPriceResponse { price: prod_price };
        Ok(Response::new(response))
    }
    async fn get_products_sn(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ProductsSnResponse>, Status> {
        loop {
            let count = {
                let count_lock = SUBSCRIPTION_COUNT.lock().unwrap();
                *count_lock
            };
            if count < 3 {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            } else {
                break;
            }
        }
        let mut sn_list = Vec::new();
        {
            let products = PRODUCTS.lock().unwrap();
            for product in products.iter() {
                sn_list.push(product.sn);
            }
        }
        let response = ProductsSnResponse { sn_list: sn_list };
        Ok(Response::new(response))
    }
}

// Import the generated offer module
use offer::{
    OfferRequest, OfferResponse, offer_server::{Offer, OfferServer}
};
pub mod offer {
    tonic::include_proto!("offer");
}

pub struct OfferImpl {}

impl OfferImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl Offer for OfferImpl {
    async fn confirm_offer(
        &self,
        _request: Request<OfferRequest>,
    ) -> Result<Response<OfferResponse>, Status> {
        // Get the price and serial number from the request
        let offer_price = _request.get_ref().price;
        let offer_sn = _request.get_ref().sn;
        let mut prod_price = 0;
        {
            // Lock the products for reading
            let products = PRODUCTS.lock().unwrap();
            // Get the price and serial number from the product data
            if offer_sn < 0 || offer_sn >= products.len() as i32 {
                return Err(Status::invalid_argument("Invalid serial number"));
            }
            prod_price = products[offer_sn as usize].price;
        }

        let mut confirm = false;

        // Check if the price is less than the offer price
        // For demonstration, let's assume the offer price is 100
        if prod_price <= offer_price {
            confirm = true;
        }

        // Return the confirmation
        let response = OfferResponse { confirmed: confirm };
        Ok(Response::new(response))
    }
}

use subscribe::{
    SubscribeRequest, SubscribeResponse, subscribe_server::{Subscribe, SubscribeServer}
};
pub mod subscribe {
    tonic::include_proto!("subscribe");
}

pub struct SubscribeImpl {}

impl SubscribeImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl Subscribe for SubscribeImpl {
    async fn subscribe(
        &self,
        _request: Request<SubscribeRequest>,
    ) -> Result<Response<SubscribeResponse>, Status> {
        let count;
        {
            let mut count_lock = SUBSCRIPTION_COUNT.lock().unwrap();
            *count_lock += 1;
            println!("Client subscribed. Total subscriptions: {}", *count_lock);
            count = *count_lock;
        }

        if count == 3 {
            println!("Starting price updates as 3 clients have subscribed.");
        }

        let response = SubscribeResponse { success: true };
        Ok(Response::new(response))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:8080".parse().unwrap();

    let date = Local::now();
    println!("Server started at: {}", date.format("%Y-%m-%d][%H:%M:%S"));

    // Initialize the products
    for i in 0..5 {
        let product_data = ProductData {
            price: rand::thread_rng().gen_range(10..=200),
            sn: i,
        };
        PRODUCTS.lock().unwrap().push(product_data);
    }

    //Create thread for services
    tokio::spawn(async move {
        Server::builder()
            .add_service(SubscribeServer::new(SubscribeImpl::new()))
            .add_service(ProductServer::new(ProductImpl::new()))
            .add_service(OfferServer::new(OfferImpl::new()))
            .serve(addr)
            .await
            .unwrap();
    });

    println!("Rust gRPC server listening on {}", addr);

    loop {
        let count = {
            let count_lock = SUBSCRIPTION_COUNT.lock().unwrap();
            *count_lock
        };
        if count < 3 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        } else {
            break;
        }
    }

    loop {
        let date = Local::now();
        println!("Update prices: {}", date.format("%Y-%m-%d][%H:%M:%S"));
        {
            /*
            Lock the products for writing, and update the price and serial number of each product
            - .lock(): Locks the mutex and returns a guard that releases the lock when dropped
            - .unwrap(): Unwraps the Result to get the value inside the Ok variant
             */
            let mut products = PRODUCTS.lock().unwrap();
            for i in 0..products.len(){
                // Update the product data
                let price = rand::thread_rng().gen_range(10..=200);
                let sn = i as i32;
                products[i] = ProductData { price, sn };
                println!("Product sn: {} - price: {}$", sn, price);
            }
        }
        println!("\n");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

    Ok(())
}