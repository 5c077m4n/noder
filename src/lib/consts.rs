use std::{path::PathBuf, env::temp_dir};

pub const NODE_VERSION_INDEX_URL: &str = "https://nodejs.org/dist/index.json";
pub const NODE_DIST_URL: &str = "https://nodejs.org/dist/";

pub const TMP_DIR_NAME: &str = ".noder";
lazy_static! {
    pub static ref TMP_DIR_PATH: PathBuf = temp_dir().join(TMP_DIR_PATH.to_owned());
}
