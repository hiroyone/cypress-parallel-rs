use crate::config;
use crate::report;
use crate::test_suites;
use crate::threads;
use crate::utility;
use std::error::Error;
use std::fs;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let settings = config::Settings::global();
    log::debug!("Reading config parameters: {:?}", settings);

    let results_path = &settings.results_path;
    utility::clean_directory(results_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    threads::parallel_execute_threads(test_weight_threads).await?;

    let test_results = utility::collect_cy_results(results_path)?;
    log::trace!("Test Result is: {:?}", test_results);

    let all_reports = report::bundle_all_reports(&test_results);
    let total_result = report::create_total_result(&test_results);
    let spec_weights = utility::generate_spec_weights(&test_results, total_result.duration);
    let spec_weights_json = serde_json::to_string_pretty(&spec_weights).unwrap();
    fs::write(&settings.weights_json, spec_weights_json)?;

    let result_table = report::create_test_result_table(&total_result, &all_reports);
    // Todo: should be printed into the cli without debug
    log::trace!("The result table is: {:?}", result_table);

    Ok(())
}
