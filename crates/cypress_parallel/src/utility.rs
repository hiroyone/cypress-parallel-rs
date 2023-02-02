use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Result, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct SpecWeight {
    pub time: u16,
    pub weight: u16,
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

pub type CyRunResults = HashMap<PathBuf, TestResult>;
type SpecWeights<'a> = HashMap<PathBuf, SpecWeight>;

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

pub fn generate_spec_weights(test_results: &CyRunResults, total_duration: u16) -> SpecWeights {
    let mut spec_weights: SpecWeights = HashMap::new();
    let total_weight: u16 = (test_results.len() * 10).try_into().unwrap();

    for (path, test_result) in test_results {
        let spec_weight = SpecWeight {
            time: test_result.duration,
            weight: (test_result.duration / total_duration) * total_weight,
        };
        spec_weights.insert(path.to_path_buf(), spec_weight);
    }

    spec_weights
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
