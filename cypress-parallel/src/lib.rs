use std::{path::{Path, PathBuf}, fs, io, collections::HashMap};
use glob::{PatternError};

/// Get a list of file paths under the directory
///
/// # Errors
///
/// This function will return an error if the passed-in directory does not exist.
fn get_file_paths_by_dir_path (dir_path:&Path) -> Result<Vec<PathBuf>, io::Error>{
    let mut entries = fs::read_dir(".")?
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
fn get_file_paths_by_glob(pattern:&str) -> Result<Vec<PathBuf>, PatternError>{

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
fn get_test_suites_paths()-> Result<Vec<PathBuf>, PatternError> {
    let mut settings = HashMap::new();
    // Todo: Rewrite this once config part is implemented.
    settings.insert("test_suites_path", "");
    
    println!("Using pattern {} to find test suites", settings["test_suites_path"]);
    let file_list = get_file_paths_by_glob(settings["test_suites_path"]);

    println!("{} test suites were found", file_list.len()); 
    if (settings.contains_key("isVerbose")) {
      println!("Paths to found suites");
      println!("{:?}", file_list);
    }

    return file_list;
}