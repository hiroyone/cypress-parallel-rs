use crate::utility;
use serde_json::Map;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct TotalResult {
    pub tests: u16,
    pub passes: u16,
    pub failures: u16,
    pub duration: u16,
    pub pending: u16,
}

#[derive(Default, Debug)]
pub struct Report {
    pub path: PathBuf,
    pub duration: u16,
    pub tests: u16,
    pub passes: u16,
    pub failures: u16,
    pub pending: u16,
}

pub fn bundle_all_reports(test_results: &utility::CyRunResults) -> Vec<Report> {
    let mut all_reports: Vec<Report> = Vec::new();

    for (path, test_result) in test_results {
        let single_report = Report {
            path: path.to_path_buf(),
            // Todo: Format the time
            duration: test_result.duration,
            tests: test_result.tests,
            passes: test_result.passes,
            failures: test_result.failures,
            pending: test_result.pending,
        };
        all_reports.push(single_report);
    }

    all_reports
}

pub fn create_total_result(test_results: &utility::CyRunResults) -> TotalResult {
    let mut total_result: TotalResult = Default::default();

    for (_path, test_result) in test_results {
        // Todo: The value of test_result.tests may not be what is expected.
        total_result.tests += test_result.tests;
        total_result.passes += test_result.passes;
        total_result.failures += test_result.failures;
        total_result.duration += test_result.duration;
        total_result.pending += test_result.pending;
    }
    total_result
}

/// Create a test result table
pub fn create_test_result_table(
    total_result: &TotalResult,
    all_reports: &Vec<Report>,
) -> Map<String, Value> {
    // User Table Module for cli Printing
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

    all_reports.into_iter().for_each(|report| {
        // Convert report into vec
        let report_vec = Vec::from([
            // Todo: Format the time
            report.duration.into(),
            report.tests.into(),
            report.passes.into(),
            report.failures.into(),
            report.pending.into(),
        ]);
        result_table.insert("report".into(), Value::Array(report_vec));
    });

    // Add the aggregated total report
    let total_result = Vec::from([
        // Todo: Format the time
        total_result.duration.into(),
        total_result.tests.into(),
        total_result.passes.into(),
        total_result.failures.into(),
        total_result.pending.into(),
    ]);

    result_table.insert("result".into(), Value::Array(total_result));

    result_table
}
