use reqwest;
use crate::lib::consts::NODE_VERSION_INDEX_URL;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn get_dist_index() -> reqwest::Result<()> {
    let json_response = HTTP_CLIENT.get(NODE_VERSION_INDEX_URL).send().await?.json().await?;

    Ok(())
}
pub fn get_remote_file(version: &str) {}
