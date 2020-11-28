use tokio::fs::File;

use super::consts::paths::CACHE_DIR_PATH;

#[cfg(test)]
mod tests;

pub async fn extract_tar(file_path: &str) -> Result<(), std::io::Error> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar = File::open(file_path).await?.try_into_std().unwrap();
    let tar = GzDecoder::new(tar);
    let mut archive = Archive::new(tar);
    archive.unpack(CACHE_DIR_PATH.as_path())
}

pub async fn extract_zip(file_path: &str) -> Result<(), std::io::Error> {
    use zip::ZipArchive;

    let file = File::open(file_path).await?.into_std().await;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let arch_file = zip.by_index(i)?;
        println!("{}", arch_file.name());
    }
    unimplemented!();

    Ok(())
}
