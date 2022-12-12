use crate::test_suites;
use crate::threads;
use crate::utility::clean_directory;
use std::error::Error;
use std::path::Path;

pub async fn start() -> Result<(), Box<dyn Error>> {
    // Todo: configure the proper path
    let dir_path = Path::new("sample_dir");
    clean_directory(dir_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    threads::parallel_execute_threads(test_weight_threads).await?;

    Ok(())
}
