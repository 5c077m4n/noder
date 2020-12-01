use std::{collections::HashMap, str};

use tokio::fs;
use sha2::{Sha256, Digest};

use super::super::types::GeneralError;

pub async fn read_sumcheck_to_map(sumcheck: &str) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    let sumcheck_lines_parts = sumcheck
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>());
    for line_parts in sumcheck_lines_parts {
        hashmap.insert(line_parts[1].to_owned(), line_parts[0].to_owned());
    }

    hashmap
}

pub async fn generate_file_sha256(file_path: &str) -> Result<String, GeneralError> {
    let file_content = fs::read(file_path).await?;
    let mut hasher = Sha256::new();
    hasher.update(file_content);

    let hash = hasher.finalize();
    let hash = hash.as_slice();
    let hash = str::from_utf8(hash)?;
    let hash = hash.to_owned();

    Ok(hash)
}