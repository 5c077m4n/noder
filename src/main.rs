#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod lib;

use clap::App;

#[tokio::main]
async fn main() -> Result<(), lib::types::GeneralError> {
    let cli_config = load_yaml!("../cli.yaml");
    let cli_config = App::from_yaml(cli_config).get_matches();

    let version_index = lib::remote_file_getter::get_dist_index().await?;

    Ok(())
}
