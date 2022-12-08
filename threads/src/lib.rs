use std::{collections::HashMap, path::{PathBuf, Path}, fs, io::Result, env};

use convert_case::{Casing, Case};
use cypress_parallel::Thread;
pub enum PackageManager {
    Yarn,
    Npm
}

/// Return yarn or npm which a user depends on.
fn get_package_manager() -> PackageManager {
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
fn create_reporter_options(string:&str) -> HashMap<&str, &str> {
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
fn create_reporter_config_file(path:&PathBuf) -> Result<()> {
    
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


pub fn create_command_arguments(thread:Thread) -> Result<Vec<String>, > {

    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    let package_variant = match get_package_manager() {
        PackageManager::Npm => "--",
        PackageManager::Yarn => ""
    };

    // Todo: it is different from the original implementation logic.
    let mut spec_files = thread.paths.into_iter().map(|path| path.to_string_lossy().to_string())
                                                            .collect::<Vec<String>>();

    let mut reporter = Vec::from(["--reporter".to_string(),
                                                settings["reporterModulePath"].to_string()]);

    let reporter_config_path;

    if settings.contains_key("reporterOptionsPath") {
        reporter_config_path = Path::new(settings["reporterOptionsPath"]).to_path_buf();
    } else {
        let cwd = env::current_dir()?;
        reporter_config_path = cwd.join("multi-reporter-config.json");
        
        create_reporter_config_file(&reporter_config_path).unwrap_or_else(|err| {
            panic!("Failed to create a report config file: {}", err);
        })
    }

    let reporter_config_path_param = String::from(format!("configFile={}", reporter_config_path.to_str().unwrap()));

    let mut reporter_options = Vec::from(["--reporter-options".to_string(), 
                                                        reporter_config_path_param ]);
    
    let mut child_options:Vec<String> = Vec::from(["run".to_string(),
    settings["script"].to_string(),
    package_variant.to_string(),
    "--spec".to_string()]);
    
    child_options.append(&mut spec_files);
    child_options.append(&mut reporter);
    child_options.append(&mut reporter_options);

    // Todo: it is different from the original implementation logic.
    child_options.append(&mut Vec::from([settings["scriptArguments"].to_string()]));

    Ok(child_options)
}