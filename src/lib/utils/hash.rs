use std::{collections::HashMap, str};

use sha2::{Digest, Sha256};
use tokio::fs;

use crate::lib::types::GeneralError;

pub async fn read_sumcheck_to_map(sumcheck: &str) -> Result<HashMap<String, String>, GeneralError> {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    sumcheck.lines().for_each(|line| {
        let line_parts: Vec<&str> = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();
        hashmap.insert(line_parts[1].to_owned(), line_parts[0].to_owned());
    });

    Ok(hashmap)
}

pub async fn generate_file_sha256(file_path: &str) -> Result<Vec<u8>, GeneralError> {
    let file_content = fs::read(file_path).await?;
    let mut hasher = Sha256::new();
    hasher.update(file_content);

    let hash = hasher.finalize();
    let hash = hash.as_slice();
    let hash = hash.to_owned();

    Ok(hash)
}
