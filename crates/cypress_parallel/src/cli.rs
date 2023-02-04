use crate::config;
use crate::report;
use crate::test_suites;
use crate::threads;
use crate::utility;
use std::error::Error;
use std::fs;
use std::process;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let settings = config::Settings::global();
    log::debug!("Reading config parameters: {:?}", settings);
    let results_path = &settings.results_path;

    utility::clean_directory(results_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    let parallel_execution_duration =
        threads::parallel_execute_threads(test_weight_threads).await?;

    let test_results = utility::collect_cy_results(results_path)?;
    log::trace!("Test Result is: {:?}", test_results);

    let all_reports = report::bundle_all_reports(&test_results);
    let total_result = report::create_total_result(&test_results);
    log::trace!("The total result: {:?}", total_result);

    let spec_weights = utility::generate_spec_weights(&test_results, total_result.duration);
    log::trace!("The spec_weights: {:?}", spec_weights);
    let spec_weights_json = serde_json::to_string_pretty(&spec_weights).unwrap();
    fs::write(&settings.weights_json, spec_weights_json)?;

    let result_table = report::create_test_result_table(&total_result, &all_reports);

    // Todo: stub value
    let parallel_execution_duration = 300;
    utility::print_saved_time(total_result.duration, parallel_execution_duration);

    // Exits with the error state if any failure exists
    if total_result.failures > 0 {
        eprintln!("{} test failures!", total_result.failures);
        process::exit(1);
    }

    // Note: in the future, it would be better to implement checking missing test results.
    // check_missing_result()

    Ok(())
}
