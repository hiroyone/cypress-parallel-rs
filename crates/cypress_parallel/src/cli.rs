use std::{path::Path, error::Error};
use crate::utility::clean_directory;
use crate::test_suite::get_test_suites_path;

pub async fn start() -> Result<(), Box<dyn Error>> {

    // Todo: configure the proper path
    let dir_path = Path::new("sample_dir");
    clean_directory(dir_path)?;

    let test_suites_path = get_test_suites_path()?;

    Ok(())
}