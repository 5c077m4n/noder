use serde_json::Value;

use super::consts::{NODE_VERSION_INDEX_URL, NODE_DIST_URL};

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn get_dist_index() -> reqwest::Result<Value> {
    let json_response: Value = HTTP_CLIENT.get(NODE_VERSION_INDEX_URL)
        .send()
        .await?
        .json()
        .await?;

    Ok(json_response)
}
pub async fn get_remote_file(version: &str, os_code: &str, arch: &str, ext: &str) {
    let url = format!(
        "{dist_url}{version}/node-{version}-{os_code}-{arch}.tar.{ext}",
        dist_url = NODE_DIST_URL,
        version = version,
        os_code = os_code,
        arch = arch,
        ext = ext,
    );
}
