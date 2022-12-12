use std::{
    collections::HashMap,
    env, fmt, fs,
    io::Result,
    path::{Path, PathBuf},
    process::{ExitStatus, Stdio},
    time,
};

use convert_case::{Case, Casing};
use tokio::{
    process::Command,
    task::JoinHandle,
    time::{sleep, Instant},
};

pub struct Thread {
    pub paths: Vec<PathBuf>,
    pub weight: i32,
}

enum PackageManager {
    Yarn,
    Npm,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageManager::Yarn => write!(f, "yarn"),
            PackageManager::Npm => write!(f, "npm"),
        }
    }
}

/// Return yarn or npm which a user depends on.
fn get_package_manager() -> PackageManager {
    // Todo: implement isYarn
    let is_yarn = true;

    let package_manager;
    if is_yarn {
        package_manager = PackageManager::Yarn
    } else {
        package_manager = PackageManager::Npm
    }
    return package_manager;
}

/// Create an option map for the report option string
fn create_reporter_options(string: &str) -> HashMap<&str, &str> {
    let options: Vec<&str> = string.split(',').collect();

    let mut option_map: HashMap<&str, &str> = HashMap::new();
    for value in options.into_iter() {
        let kv_array: Vec<&str> = value.split("=").collect();
        let key: &str = kv_array[0].trim();
        let value: &str = kv_array[1].trim();
        option_map.insert(key, value);
    }
    option_map
}

/// Write Cypress reporting config to the json file
///
/// # Errors
///
/// This function will return an error if writing the JSON to the file fails.
fn create_reporter_config_file(path: &PathBuf) -> Result<()> {
    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    let mut reporter_enabled = Vec::from(["cypress-parallel-rs/json-stream.reporter.js"]);

    if settings.contains_key("reporter") {
        reporter_enabled.push(settings["reporter"])
    } else {
        reporter_enabled.push("cypress-parallel/simple-spec.reporter.js")
    }

    let mut option_name = String::new();
    let mut reporter_options = HashMap::new();
    if settings.contains_key("reporterOptions") {
        // Create a camel name + suffix
        option_name.push_str(&settings["reporter"].to_case(Case::Camel));
        option_name.push_str("ReporterOptions");
        reporter_options = create_reporter_options(settings["reporterOptions"]);
    }

    fs::write(
        path,
        serde_json::json!({
            "reporterEnabled": reporter_enabled.join(","),
            option_name: reporter_options
        })
        .to_string(),
    )
}

/// Create command arguments based on spec files and the config
///
/// # Panics
///
/// Panics if it fails to create a report config file
///
/// # Errors
///
/// This function will return an error if the current directory is not found.
fn create_command_arguments(thread: &Thread) -> Result<Vec<String>> {
    // Todo: Rewrite this once config part is implemented.
    let settings: HashMap<&str, &str> = HashMap::new();

    let package_variant = match get_package_manager() {
        PackageManager::Npm => "--",
        PackageManager::Yarn => "",
    };

    // Todo: it is different from the original implementation logic.
    let mut spec_files = thread
        .paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<String>>();

    let mut reporter = Vec::from([
        "--reporter".to_string(),
        settings["reporterModulePath"].to_string(),
    ]);

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

    let reporter_config_path_param = String::from(format!(
        "configFile={}",
        reporter_config_path.to_str().unwrap()
    ));

    let mut reporter_options =
        Vec::from(["--reporter-options".to_string(), reporter_config_path_param]);

    let mut child_options: Vec<String> = Vec::from([
        "run".to_string(),
        settings["script"].to_string(),
        package_variant.to_string(),
        "--spec".to_string(),
    ]);

    child_options.append(&mut spec_files);
    child_options.append(&mut reporter);
    child_options.append(&mut reporter_options);

    // Todo: it is different from the original implementation logic.
    child_options.append(&mut Vec::from([settings["scriptArguments"].to_string()]));

    Ok(child_options)
}

/// Execute test files asynchronously
///
/// # Panics
///
/// Panics if the function failed to create a command argument.
pub async fn execute_thread(thread: &Thread, index: u64) -> Result<ExitStatus> {
    let package_manager = get_package_manager();
    let command_arguments = create_command_arguments(thread)?;

    let ten_millis = time::Duration::from_millis(index);

    sleep(ten_millis).await;

    // Todo: display an error detail if exit_status > 0
    let cmd = Command::new(package_manager.to_string())
        .args(command_arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to start the process")
        .wait()
        .await?;

    Ok(cmd)
}

/// Execute test files asynchronously in parallel
///
/// # Panics
///
/// Panics if it fails to convert usize to u64.
///
/// # Errors
///
/// This function will return an error if running multiple threads fails.
pub async fn parallel_execute_threads(test_weight_threads: Vec<Thread>) -> Result<()> {
    let handles: Vec<JoinHandle<Result<ExitStatus>>> = test_weight_threads
        .into_iter()
        .enumerate()
        .map(|(index, thread)| {
            tokio::spawn(async move { execute_thread(&thread, index.try_into().unwrap()).await })
        })
        .collect();

    // Todo: Set a proper start and end
    let start = Instant::now();

    for handle in handles {
        // Todo: remove double questions and return errors
        handle.await??;
    }
    println!(
        "It took {} seconds to execute tests.",
        start.elapsed().as_secs()
    );

    Ok(())
}
