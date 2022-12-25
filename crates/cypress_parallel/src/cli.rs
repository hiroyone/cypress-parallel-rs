use crate::config;
use crate::test_suites;
use crate::threads;
use crate::utility;
use std::error::Error;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let settings = config::Settings::global();
    log::debug!("Reading config parameters: {:?}", settings);

    let results_path = &settings.results_path;
    utility::clean_directory(results_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    threads::parallel_execute_threads(test_weight_threads).await?;

    let results_maps = utility::collect_cy_results(results_path)?;

    log::debug!("{:?}", results_maps);

    Ok(())
}
