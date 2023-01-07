use serde_json::Map;
use serde_json::Value;

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

    let test_results = utility::collect_cy_results(results_path)?;

    log::trace!("{:?}", test_results);

    let result_table = create_test_result_table(&test_results);
    log::trace!("{:?}", result_table);

    Ok(())
}

/// Create a test result table
fn create_test_result_table(test_results: &utility::CyRunResults) -> Map<String, Value> {
    let mut result_table: Map<String, Value> = Map::new();

    // Add the header line of the table
    result_table.insert(
        "head".into(),
        Value::Array(Vec::from([
            Value::String("Spec".into()),
            Value::String("Time".into()),
            Value::String("Tests".into()),
            Value::String("Passing".into()),
            Value::String("Failing".into()),
            Value::String("Pending".into()),
        ])),
    );

    // Insert the style
    let mut style_head_map = Map::new();
    style_head_map.insert("head".into(), "blue".into());
    result_table.insert("style".into(), Value::Object(style_head_map));

    // Insert the col Width
    let col_widths = Vec::from([50.into(), 8.into(), 7.into(), 9.into(), 9.into(), 9.into()]);
    result_table.insert("colWidths".into(), Value::Array(col_widths));

    result_table
}
