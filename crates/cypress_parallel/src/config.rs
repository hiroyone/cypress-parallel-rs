use once_cell::sync::OnceCell;
use std::{fmt::Error, path::PathBuf};
#[derive(Debug)]
pub struct Settings {
    pub thread_count: u16,
    pub test_suites_path: String,
    pub should_bail: bool,
    pub is_verbose: bool,
    pub weights_json: String,
    pub default_weight: u16,
    pub reporter: String,
    pub reporter_module_path: PathBuf,
    pub reporter_options: String,
    pub reporter_options_path: PathBuf,
    pub script: String,
    pub strict_mode: bool,
    pub script_arguments: Vec<String>,
}

pub static SETTINGS: OnceCell<Settings> = OnceCell::new();

impl Settings {
    pub fn global() -> &'static Self {
        SETTINGS.get_or_init(|| Self {
            thread_count: 2,
            test_suites_path: "cypress/integration".into(),
            should_bail: false,
            is_verbose: false,
            weights_json: "cypress/parallel-weights.json".into(),
            default_weight: 1,
            reporter: "".into(),
            reporter_module_path: "cypress-multi-reporters".into(),
            reporter_options: "".into(),
            reporter_options_path: "".into(),
            script: "".into(),
            strict_mode: true,
            script_arguments: Vec::from(["2".into()]),
        })
    }

    pub fn new(
        thread_count: u16,
        test_suites_path: String,
        should_bail: bool,
        is_verbose: bool,
        weights_json: String,
        default_weight: u16,
        reporter: String,
        reporter_module_path: PathBuf,
        reporter_options: String,
        reporter_options_path: PathBuf,
        script: String,
        strict_mode: bool,
        script_arguments: Vec<String>,
    ) -> Self {
        Self {
            thread_count,
            test_suites_path,
            should_bail,
            is_verbose,
            weights_json,
            default_weight,
            reporter,
            reporter_module_path,
            reporter_options,
            reporter_options_path,
            script,
            strict_mode,
            script_arguments,
        }
    }
}
