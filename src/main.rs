#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;

mod lib;

use clap::App;
use lib::types::GeneralError;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), GeneralError> {
    let cli_config = load_yaml!("../cli.yaml");
    let cli_config = App::from_yaml(cli_config).get_matches();

    let version_index = lib::remote_file_getter::get_dist_index().await?;

    if let Some(_matches) = cli_config.subcommand_matches("list") {
        match version_index {
            Value::Array(version_list) => {
                let version_list: Vec<&Value> = version_list
                    .iter()
                    .map(|v| v.get("version").unwrap())
                    .collect();
            }
            _ => {}
        }
    }

    Ok(())
}
