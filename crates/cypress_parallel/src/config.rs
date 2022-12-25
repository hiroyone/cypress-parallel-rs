use once_cell::sync::OnceCell;
use std::path::PathBuf;
#[derive(Debug)]
pub struct Settings {
    pub thread_count: u16,
    pub test_suites_path: String,
    pub should_bail: bool,
    pub is_verbose: bool,
    pub weights_json: String,
    pub default_weight: u16,
    pub reporter: String,
    pub reporter_module_path: String,
    pub reporter_options: String,
    pub reporter_options_path: String,
    pub script: String,
    pub strict_mode: bool,
    pub script_arguments: Vec<String>,
    pub results_path: PathBuf,
}

pub static SETTINGS: OnceCell<Settings> = OnceCell::new();

impl Settings {
    /// Add a global default value
    pub fn global() -> &'static Self {
        SETTINGS.get_or_init(|| Self {
            thread_count: 2,
            test_suites_path: "cypress/e2e/**/*.js".into(),
            should_bail: false,
            is_verbose: false,
            weights_json: "cypress/parallel-weights.json".into(),
            default_weight: 1,
            reporter: "cypress-parallel/simple-spec.reporter.js".into(),
            reporter_module_path: "cypress-multi-reporters".into(),
            reporter_options: "".into(),
            reporter_options_path: "multi-reporter-config.json".into(),
            // Todo: script may need be empty string in the end.
            script: "cypress:run".into(),
            strict_mode: true,
            script_arguments: Vec::from(["".into()]),
            results_path: "runner-results".into(),
        })
    }
}
