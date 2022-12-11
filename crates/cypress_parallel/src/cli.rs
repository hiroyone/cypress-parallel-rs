use crate::test_suites;
use crate::utility::clean_directory;
use std::{error::Error, path::Path};

pub async fn start() -> Result<(), Box<dyn Error>> {
    // Todo: configure the proper path
    let dir_path = Path::new("sample_dir");
    clean_directory(dir_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;

    Ok(())
}
