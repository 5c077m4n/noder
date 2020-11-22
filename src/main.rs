#[macro_use]
extern crate lazy_static;

mod lib;

#[tokio::main]
async fn main() -> Result<(), lib::types::GeneralError> {
    let version_index = lib::remote_file_getter::get_dist_index().await?;
    println!("{:?}", version_index);

    Ok(())
}
