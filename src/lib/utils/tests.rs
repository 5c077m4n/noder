use crate::lib::{
    remote_file_getter::{get_sumcheck_file, save_remote_file},
    types::GeneralError,
};

use super::hash::{generate_file_sha256, read_sumcheck_to_map};

#[tokio::test]
async fn test_file_sumcheck_generator() -> Result<(), GeneralError> {
    let new_file_name = save_remote_file("v14.15.1").await?;
    let sha256 = generate_file_sha256(&new_file_name).await?;

    assert!(
        !sha256.is_empty(),
        "The sumcheck for `{}` is not supposed to be empty.",
        new_file_name
    );
    Ok(())
}

#[tokio::test]
async fn test_hash_table_read_and_parse() -> Result<(), GeneralError> {
    let hashmap_text = get_sumcheck_file("v14.15.1").await?;
    let hashmap = read_sumcheck_to_map(&hashmap_text).await?;

    assert!(!hashmap.is_empty());
    Ok(())
}
