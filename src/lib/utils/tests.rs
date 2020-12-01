use crate::lib::{types::GeneralError, remote_file_getter::save_remote_file};

use super::hash::generate_file_sha256;

#[tokio::test]
async fn test_file_sumcheck_generator() -> Result<(), GeneralError> {
    let new_file_name = save_remote_file("v14.15.1").await?;
    let sha256 = generate_file_sha256(&new_file_name).await?;

    assert_ne!(sha256.len(), 0, "The sumcheck for {} is {}.", new_file_name, sha256);
    Ok(())
}
