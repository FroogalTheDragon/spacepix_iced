mod apis;
mod errors;
mod parser;
mod urls;

use apis::Apod;
use parser::Parser;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let apod_data = Apod::get_apod_data().await.unwrap_or_else(|e| {
        println!("Failed to get APOD data: {}", e);
        Apod::default()
    });
    dbg!(apod_data);
}
