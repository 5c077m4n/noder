#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    Ok(())
}
