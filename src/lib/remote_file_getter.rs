use reqwest;
use serde::{Deserialize, Serialize};

use crate::lib::consts::NODE_VERSION_INDEX_URL;

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

#[derive(Deserialize, Serialize)]
pub struct VersionData {
    pub date: String,
    pub files: Vec<String>,
    pub lts: bool,
    pub modules: String,
    pub npm: String,
    pub openssl: String,
    pub security: bool,
    pub uv: String,
    pub v8: String,
    pub version: String,
    pub zlib: String,
}

pub async fn get_dist_index() -> reqwest::Result<Vec<VersionData>> {
    let json_response: Vec<VersionData> = HTTP_CLIENT.get(NODE_VERSION_INDEX_URL)
        .send()
        .await?
        .json()
        .await?;

    Ok(json_response)
}
pub async fn get_remote_file(version: &str) {}
