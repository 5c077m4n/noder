use tokio::fs;

use super::consts::{NODE_VERSION_INDEX_URL, NODE_DIST_URL, TMP_DIR_PATH};

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn get_dist_index() -> reqwest::Result<serde_json::Value> {
    let json_response = HTTP_CLIENT.get(NODE_VERSION_INDEX_URL)
        .send()
        .await?;
    assert!(json_response.status().is_success());
    let json_response: serde_json::Value = json_response
        .json()
        .await?;

    Ok(json_response)
}

pub async fn save_remote_file(version: &str, os_code: &str, arch: &str, ext: &str) -> reqwest::Result<String> {
    let filename = format!(
        "node-{version}-{os_code}-{arch}{ext}",
        version = version,
        os_code = os_code,
        arch = arch,
        ext = ext,
    );
    let url = format!(
        "{dist_url}{version}/{filename}",
        dist_url = NODE_DIST_URL,
        version = version,
        filename = filename,
    );

    let package = HTTP_CLIENT.get(&url)
        .send()
        .await?;
    assert!(package.status().is_success());
    let package = package
        .bytes()
        .await?;

    if !fs::metadata(TMP_DIR_PATH).await?.is_ok() {
        fs::create_dir_all(TMP_DIR_PATH).await?;
    }
    fs::copy(&package, TMP_DIR_PATH.join(&filename)).await?;

    Ok(filename)
}
