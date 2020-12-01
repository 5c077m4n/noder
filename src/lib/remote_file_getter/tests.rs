use super::super::types::GeneralError;

#[tokio::test]
async fn test_get_dist_index() -> Result<(), GeneralError> {
    use super::get_dist_index;

    let json = get_dist_index().await?;
    match json {
        serde_json::Value::Array(_) => assert!(true),
        _ => assert!(false),
    }

    Ok(())
}

#[tokio::test]
async fn test_get_sumcheck_file() -> Result<(), GeneralError> {
    use super::get_sumcheck_file;

    let file_text: String = get_sumcheck_file("v14.15.1").await?;
    assert!(file_text.len() > 0);

    Ok(())
}

#[tokio::test]
async fn test_save_remote_file() -> Result<(), GeneralError> {
    use super::save_remote_file;

    let new_file_name = save_remote_file("v14.15.1").await?;
    assert!(new_file_name.len() > 0);

    Ok(())
}

#[tokio::test]
async fn test_sumcheck_file_download() -> Result<(), GeneralError> {
    use super::get_sumcheck_file;

    let sumcheck = get_sumcheck_file("v14.15.1").await?;
    assert!(sumcheck.len() > 0);

    Ok(())
}
