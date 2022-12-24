use crate::config;
use crate::test_suites;
use crate::threads;
use crate::utility;
use std::env;
use std::error::Error;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let setting = config::Settings::global();

    utility::clean_directory(&setting.result_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    threads::parallel_execute_threads(test_weight_threads).await?;

    let cwd = env::current_dir()?;
    let results_path = cwd.join("runner-results");
    let results_maps = utility::collect_cy_results(&results_path)?;

    println!("{:?}", results_maps);

    Ok(())
}
