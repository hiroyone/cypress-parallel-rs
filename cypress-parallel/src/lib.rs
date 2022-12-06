use core::str;
use std::{path::{Path, PathBuf}, fs, io, collections::{HashMap, BTreeMap}};
use glob::{PatternError};

type TestSuitePaths = Vec<PathBuf>;

pub struct Thread<'a>(Vec<&'a PathBuf>, i32);
type Threads<'a> = Vec<Thread<'a>>;

type OrderedTestDist<'a> = BTreeMap<i32, &'a PathBuf>;

/// Get a list of file paths under the directory
///
/// # Errors
///
/// This function will return an error if the passed-in directory does not exist.
pub fn get_file_paths_by_dir_path (dir_path:&Path) -> Result<Vec<PathBuf>, io::Error>{
    let mut entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    
    entries.sort();
    return Ok(entries)
}

/// Get a list of file paths for a given glob pattern.
///
/// # Errors
///
/// This function will return an error if the given path does not exist.
pub fn get_file_paths_by_glob(pattern:&str) -> Result<TestSuitePaths, PatternError>{

    let mut entries = glob::glob(pattern).expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect::<Vec<PathBuf>>();

    entries.sort();
    return Ok(entries)

}

/// Get a list of test suite paths for a given test_suites_path passed in to an argument
///
/// # Errors
///
/// This function will return an error if the given path does not exist.
pub fn get_test_suites_paths()-> Result<Vec<PathBuf>, PatternError> {

    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();
    
    println!("Using pattern {} to find test suites", settings["test_suites_path"]);
    let file_list = get_file_paths_by_glob(settings["test_suites_path"])?;

    let file_len =  file_list.len();
    println!("{} test suites were found", file_len); 
    if settings.contains_key("isVerbose") {
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
/// This function will return an error if "weights_json" attribute does not exist in the config file.
pub fn distribute_tests_by_weight(test_suite_paths: &TestSuitePaths) -> Result<OrderedTestDist, std::io::Error> {
    
    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    // Retrieve execution weights from the config file
    let spec_weights_json = fs::read_to_string(settings["weights_json"])?;
    let spec_weights:HashMap<&str, i32> = serde_json::from_str(&spec_weights_json)?;

    // Create an ordered map for weights and test paths passed from the JSON file
    let mut ordered_test_dist: BTreeMap<i32, &PathBuf> = BTreeMap::new();
    test_suite_paths.into_iter().for_each(|file_path: &PathBuf| {
        let mut spec_weight:i32 = settings["defaultWeight"].parse().unwrap();
        for spec_path in spec_weights.keys() {
            if file_path.ends_with(spec_path) {
                spec_weight = spec_weights[spec_path];
                break;
            }
        };
        ordered_test_dist.insert(spec_weight, file_path);
        });
        
    Ok(ordered_test_dist)
}


/// distribute test and weights by threads 
///
/// # Panics
///
/// Panics if "threadCount" attribute does not exist in the config file.
pub fn distribute_tests_by_threads(ordered_test_dist: OrderedTestDist) -> Threads {
    
    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    let mut threads: Vec<Thread>=Vec::new();
    let thread_count = settings["threadCount"].parse().unwrap();

    for _ in 0..thread_count {
        threads.push(Thread(Vec::new(),0))
    }

    for (key, value) in ordered_test_dist.into_iter() {
        threads.sort_by(|a, b| a.1.cmp(&b.1));
        threads[0].0.push(value);
        threads[0].1 += key;
        }
    return threads
}