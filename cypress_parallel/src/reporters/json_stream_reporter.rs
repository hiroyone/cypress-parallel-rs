use std::{time::{Duration, Instant}, path::Path, fs, io::Error};

use regex::Regex;
use serde_json::{Map, Value, Number, json, to_string_pretty};

type Statistics = Map<String, Value>;

/// Create a clean statistics from the base statistics 
fn clean_statistics() -> Statistics {

    // Todo: configure stats
    let mut stats:Statistics = Map::new();

    // Todo: Set a proper start and end
    let start = Instant::now();
    let end = Instant::now().checked_add(Duration::new(1,0)).unwrap();

    // Todo: Set a proper file
    let file = String::new();

    stats.insert("duration".to_owned(), 
                Value::Number(Number::from(end.duration_since(start).as_secs())));
    stats.insert("file".to_owned(), Value::String(file));

    stats
}


/// Write statistics to a specified file path
///
/// # Panics
///
/// Panics if serialization of json failed
///
/// # Errors
///
/// This function will return an error if writing json to a file failed.
fn write_stats_file(statistics:Statistics, result_path:&Path) -> Result<(), Error>{
    
    let file_name = statistics["file"].as_str().unwrap();

    // replace forward and backward slash with _ to generate filename
    let re = Regex::new(r"\\|\/").unwrap();
    let file_name = re.replace_all(file_name, "_");
    let file_name = format!("{}.json", file_name); 

    if result_path.is_dir() {
        fs::create_dir_all(result_path)?;
    } 

    let spec_result_path = Path::join(result_path, file_name);
    let stats_json = to_string_pretty(&statistics).unwrap();

    fs::write(spec_result_path,  stats_json)
}

pub fn json_stream_custom() {
    // Todo: configure variables
    let statistics = Statistics::new();
    let result_path = Path::new("todo");
    
    clean_statistics();
    write_stats_file(statistics, result_path).unwrap_or_else(|err| {
        panic!("Failed to write the statistics to a file: {}", err);
    });
    todo!();
}