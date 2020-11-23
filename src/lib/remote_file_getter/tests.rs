use super::super::types;

#[tokio::test]
async fn test_get_dist_index() -> Result<(), types::GeneralError> {
    use super::get_dist_index;

    let json = get_dist_index().await?;
    match json {
        serde_json::Value::Array(_) => assert!(true),
        _ => assert!(false),
    }

    Ok(())
}

#[tokio::test]
async fn test_save_remote_file() -> Result<(), types::GeneralError> {
    use super::save_remote_file;

    let new_file_name = save_remote_file("v14.15.1").await?;
    assert!(new_file_name.len() > 0);

    Ok(())
}
