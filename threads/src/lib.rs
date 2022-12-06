use std::{collections::HashMap, path::PathBuf, fs, io::Result};

use convert_case::{Casing, Case};
pub enum PackageManager {
    Yarn,
    Npm
}

/// Return yarn or npm which a user depends on.
pub fn get_package_manager() -> PackageManager {
    // Todo: implement isYarn
    let is_yarn = true; 

    let package_manager;
    if is_yarn {
        package_manager=PackageManager::Yarn
    } else {
        package_manager=PackageManager::Npm
    }
    return package_manager
}

/// Create an option map for the report option string
pub fn create_reporter_options(string:&str) -> HashMap<&str, &str> {
    let options:Vec<&str> = string.split(',').collect();

    let mut option_map: HashMap<&str, &str> = HashMap::new();
    for value in options.into_iter() {
        let kv_array: Vec<&str> = value.split("=").collect();
        let key:&str=kv_array[0].trim();
        let value:&str=kv_array[1].trim();
        option_map.insert(key, value);
    }
    option_map
} 

/// Write Cypress reporting config to the json file 
///
/// # Errors
///
/// This function will return an error if writing the JSON to the file fails. 
pub fn create_reporter_config_file(path:&PathBuf) -> Result<()> {
    
    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    let mut reporter_enabled = Vec::from(["cypress-parallel-rs/json-stream.reporter.js"]);

    if settings.contains_key("reporter") {
        reporter_enabled.push(settings["reporter"])
    } else {
        reporter_enabled.push("cypress-parallel/simple-spec.reporter.js")
    }

    let mut option_name=String::new();
    if settings.contains_key("reporterOptions") {
        // Create a camel name + suffix 
        option_name.push_str(&settings["reporter"].to_case(Case::Camel));
        option_name.push_str("ReporterOptions");
    }

    fs::write(path, serde_json::json!({
        "reporterEnabled": reporter_enabled.join(","),
        option_name: create_reporter_options(settings["reporterOptions"])
    }).to_string())

}