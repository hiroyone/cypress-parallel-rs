use crate::{config, threads::Thread};
use core::str;
use glob::PatternError;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

type TestSuitesPath = Vec<PathBuf>;
type OrderedTestDist = BTreeMap<u16, PathBuf>;

/// Get a list of file paths under the directory
///
/// # Errors
///
/// This function will return an error if the passed-in directory does not exist.
#[allow(dead_code)]
fn get_file_paths_by_dir_path(dir_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    return Ok(entries);
}

/// Get a list of file paths for a given glob pattern.
///
/// # Errors
///
/// This function will return an error if the given path does not exist.
fn get_file_paths_by_glob(pattern: &str) -> Result<TestSuitesPath, PatternError> {
    let mut entries = glob::glob(pattern)?
        .filter_map(Result::ok)
        .collect::<Vec<PathBuf>>();

    entries.sort();
    return Ok(entries);
}

/// Get a list of test suites path for a given test_suites_path passed in to an argument
///
/// # Errors
///
/// This function will return an error if the given path does not exist.
pub fn get_test_suites_path() -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let settings = config::Settings::global();
    let test_suites_path = &settings.test_suites_path;

    println!("Using pattern {} to find test suites", test_suites_path);
    let file_list = get_file_paths_by_glob(test_suites_path)?;

    let file_len = file_list.len();
    println!("{} test suites were found", file_len);
    if settings.is_verbose {
        println!("Paths to found suites");
        println!("{:?}", file_list);
    }

    Ok(file_list)
}

/// distribute tests by weight
///
/// # Panics
///
/// Panics if defaultWeight string is not able to parse into number.
///
/// # Errors
///
/// This function will return an error if "weightsJSON" attribute does not exist in the config file.
pub fn distribute_tests_by_weight(
    test_suites_path: TestSuitesPath,
) -> Result<OrderedTestDist, Box<dyn Error>> {
    let settings = config::Settings::global();
    let weights_json = &settings.weights_json;
    let default_weight = settings.default_weight;

    // Retrieve execution weights from the config file
    let spec_weights_json = fs::read_to_string(weights_json)?;
    let spec_weights: HashMap<&str, u16> = serde_json::from_str(&spec_weights_json)?;

    // Create an ordered map for weights and test paths passed from the JSON file
    let mut ordered_test_dist: BTreeMap<u16, PathBuf> = BTreeMap::new();
    test_suites_path.into_iter().for_each(|file_path: PathBuf| {
        let mut spec_weight = default_weight;
        for spec_path in spec_weights.keys() {
            if file_path.ends_with(spec_path) {
                spec_weight = spec_weights[spec_path];
                break;
            }
        }
        ordered_test_dist.insert(spec_weight, file_path);
    });

    Ok(ordered_test_dist)
}

/// distribute test and weights by threads
///
/// # Panics
///
/// Panics if "threadCount" attribute does not exist in the config file.
pub fn distribute_tests_by_threads(
    ordered_test_dist: OrderedTestDist,
) -> Result<Vec<Thread>, Box<dyn Error>> {
    let settings = config::Settings::global();

    let mut threads: Vec<Thread> = Vec::new();
    let thread_count = settings.thread_count;

    for _ in 0..thread_count {
        threads.push(Thread {
            paths: Vec::new(),
            weight: 0,
        })
    }

    for (key, value) in ordered_test_dist.into_iter() {
        threads.sort_by(|a, b| a.weight.cmp(&b.weight));
        threads[0].paths.push(value);
        threads[0].weight += key;
    }
    return Ok(threads);
}

pub fn get_test_weight_threads() -> Result<Vec<Thread>, Box<dyn Error>> {
    let test_suites_path = get_test_suites_path()?;
    let test_weight_map = distribute_tests_by_weight(test_suites_path)?;
    let threads = distribute_tests_by_threads(test_weight_map)?;
    Ok(threads)
}