use std::{
    collections::HashMap,
    fmt, fs,
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

use crate::config;

#[derive(Debug)]
pub struct Thread {
    pub paths: Vec<PathBuf>,
    pub weight: u16,
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
    let is_yarn = false;

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

    let mut reporter_options: HashMap<&str, &str> = HashMap::new();
    for value in options.into_iter() {
        let kv_array: Vec<&str> = value.split("=").collect();
        let key: &str = kv_array[0].trim();
        let value: &str = kv_array[1].trim();
        reporter_options.insert(key, value);
    }
    reporter_options
}

/// Write Cypress reporting config to the json file
///
/// # Errors
///
/// This function will return an error if writing the JSON to the file fails.
fn create_reporter_config_file(path: &PathBuf) -> Result<()> {
    log::debug!(
        "reporter-config.json does not exists in {:?}. New one will be created.",
        path
    );

    let settings = config::Settings::global();
    let reporter = &settings.reporter;
    let mut reporter_enabled: Vec<String> =
        Vec::from(["cypress-parallel/reporters/json-stream.reporter.js".into()]);

    reporter_enabled.push(reporter.to_string());

    let mut option_name = String::new();
    let mut reporter_options = HashMap::new();
    if &settings.reporter_options != "" {
        // Create a camel name + suffix
        option_name.push_str(&reporter.as_str().to_case(Case::Camel));
        option_name.push_str("ReporterOptions");
        reporter_options = create_reporter_options(&settings.reporter_options);
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
/// # Errors
///
/// This function will return an error if the current directory is not found.
fn create_command_arguments(thread: &Thread) -> Result<Vec<String>> {
    let settings = config::Settings::global();
    let package_variant = match get_package_manager() {
        PackageManager::Npm => "",
        PackageManager::Yarn => "--",
    };

    // Todo: glob esacpe may be better considered.
    let spec_files = thread
        .paths
        .iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(",");

    let mut reporter = Vec::from([
        "--reporter".to_owned(),
        settings.reporter_module_path.to_owned(),
    ]);

    let reporter_config_path = Path::new(&settings.reporter_options_path).to_path_buf();
    if !reporter_config_path.is_file() {
        create_reporter_config_file(&reporter_config_path)?;
    }
    let mut reporter_options = Vec::from([
        "--reporter-options".to_string(),
        format!(
            "configFile={}",
            reporter_config_path.to_string_lossy().to_string()
        ),
    ]);

    let mut command_arguments: Vec<String> = Vec::from([
        "run".to_string(),
        settings.script.to_owned(),
        package_variant.to_owned(),
        "--spec".to_owned(),
        spec_files,
    ]);
    command_arguments.append(&mut reporter);
    command_arguments.append(&mut reporter_options);
    command_arguments.append(&mut settings.script_arguments.to_owned());

    log::trace!("Command script: {:?}", command_arguments);

    Ok(command_arguments)
}

/// Execute test files asynchronously
///
/// # Panics
///
/// Panics if a process fails to start.
///
/// # Errors
///
/// This function will return an error if command_arguments fails to get created.
pub async fn execute_thread(thread: &Thread, index: u64) -> Result<ExitStatus> {
    let package_manager = get_package_manager();
    let command_arguments = create_command_arguments(thread)?;

    let ten_millis = time::Duration::from_millis(index);

    sleep(ten_millis).await;

    // Todo: display an error detail if exit_status > 0
    // Todo: test the command works for both npm and yarn (or npx would be better)
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
pub async fn parallel_execute_threads(test_weight_threads: Vec<Thread>) -> Result<u64> {
    log::info!("Execute threads in parallel.");
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

    let parallel_execution_duration = start.elapsed().as_secs();
    log::info!(
        "It took {} seconds to execute tests.",
        parallel_execution_duration
    );

    Ok(parallel_execution_duration)
}
