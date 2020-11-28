use tokio::{fs, io};
use url::Url;

use super::consts::{
    http::HTTP_CLIENT,
    paths::{NODE_DIST_URL, NODE_VERSION_INDEX_URL, TMP_DIR_PATH}
};
use super::types::GeneralError;
use super::utils::os::get_os_node_file_name;

#[cfg(test)]
mod tests;

pub async fn get_dist_index() -> reqwest::Result<serde_json::Value> {
    let json_response = HTTP_CLIENT.get(NODE_VERSION_INDEX_URL).send().await?;
    assert!(json_response.status().is_success());
    let json_response: serde_json::Value = json_response.json().await?;

    Ok(json_response)
}

pub async fn save_remote_file(version: &str) -> Result<String, GeneralError> {
    let filename = get_os_node_file_name(version);
    let url = format!(
        "{dist_url}{version}/{filename}",
        dist_url = NODE_DIST_URL,
        version = version,
        filename = filename.as_ref().unwrap(),
    );
    assert!(Url::parse(&url).is_ok());

    let package = HTTP_CLIENT.get(&url).send().await?;
    let package = package.bytes().await?;
    let mut package = package.as_ref();

    if !fs::metadata(TMP_DIR_PATH.to_owned()).await.is_ok() {
        fs::create_dir_all(TMP_DIR_PATH.to_owned()).await?;
    }

    let filepath = TMP_DIR_PATH.join(filename.as_ref().unwrap());
    let mut file = fs::File::create(&filepath).await?;
    let _ = io::copy(&mut package, &mut file).await?;

    let filepath = filepath.to_str().unwrap().to_string();
    Ok(filepath)
}
