use crate::lib::{remote_file_getter::save_remote_file, types::GeneralError};

use super::hash::generate_file_sha256;

#[tokio::test]
async fn test_file_sumcheck_generator() -> Result<(), GeneralError> {
    let new_file_name = save_remote_file("v14.15.1").await?;
    let sha256 = generate_file_sha256(&new_file_name).await?;

    assert!(
        !sha256.is_empty(),
        "The sumcheck for `{}` not supposed to be empty.",
        new_file_name
    );
    Ok(())
}
