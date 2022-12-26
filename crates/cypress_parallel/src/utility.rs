use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Result, Write},
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Debug)]
struct SpecWeight {
    time: Duration,
    weight: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResult {
    pub suites: u16,
    pub tests: u16,
    pub passes: u16,
    pub pending: u16,
    pub failures: u16,
    pub start: String,
    pub duration: u16,
    pub file: PathBuf,
}

type CyRunResults = HashMap<PathBuf, TestResult>;
type SpecWeights<'a> = HashMap<&'a str, SpecWeight>;
type TotalWeight = u64;

/// Remove all files in the directory only if the directory exists. Do nothing if not.
///
/// # Errors
///
/// This function will return an error if the directory operation fails.
pub fn clean_directory(dir_path: &Path) -> Result<()> {
    if dir_path.is_dir() {
        fs::remove_dir_all(&dir_path)?;
        fs::create_dir_all(dir_path)?;
    } else {
        fs::create_dir_all(dir_path)?;
    }
    log::debug!("The directory is cleaned: {:?}", dir_path);
    Ok(())
}

/// Create a file with directory
///
/// # Errors
///
/// This function will return an error if creating a directory or a file fails.
pub fn create_file_with_dir(weights_json: &Path) -> Result<()> {
    if let Some(parent_dir) = weights_json.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let mut file = fs::File::create(weights_json)?;
    file.write_all(b"{}")?;
    Ok(())
}

/// Create weights for parallel computing based on the ratio of an execution time to the total time
#[allow(dead_code)]
fn generate_weights(
    spec_weights: &mut SpecWeights,
    total_duration: Duration,
    total_weight: TotalWeight,
) {
    for (_, spec_weight) in spec_weights.iter_mut() {
        let ratio = (spec_weight.time.as_millis() / total_duration.as_millis()) as u64;
        spec_weight.weight = ((ratio * total_weight) as f64).floor() as u64;
    }
}

#[test]
fn generate_weights_test() {
    let mut spec_weights: SpecWeights = HashMap::from([(
        "sample",
        SpecWeight {
            time: Duration::from_millis(500),
            weight: 0,
        },
    )]);

    let total_weight: TotalWeight = 1000;

    generate_weights(&mut spec_weights, Duration::from_millis(100), total_weight);

    assert_eq!(spec_weights["sample"].weight, 5000);
}

/// Gather Cypress results from the result directory
///
/// # Errors
///
/// This function will return an error if the directory path does not exist.
pub fn collect_cy_results(results_path: &Path) -> Result<CyRunResults> {
    let mut results: CyRunResults = HashMap::new();

    for entry in fs::read_dir(results_path)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_cy_results(&path)?;
        } else {
            let content = fs::read_to_string(&path)?;
            let content: TestResult = serde_json::from_str(&content)?;
            results.insert(path, content);
        }
    }

    return Ok(results);
}
