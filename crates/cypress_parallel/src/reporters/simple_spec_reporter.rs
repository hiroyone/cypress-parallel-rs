use serde_json::{Map, Value};

type CurrentSuite = Map<String, Value>;

/// Generate a test description from a test suite and runner
fn get_test_description(current_suite: &CurrentSuite, test: &Map<String, Value>) -> String {
    let mut suite_titles = String::new();
    // Concat all titles by recursion
    let mut current = current_suite.clone();
    while current.contains_key("parent") {
        suite_titles = format!("{} - {}", current["title"], suite_titles);

        if let Value::Object(map) = &current["parent"] {
            current = map.clone();
        }
    }

    let test_title = &test["title"];
    // Todo: Get a runner instance
    let runner_suite_file = String::new();

    return format!("{}{} ({})", suite_titles, test_title, runner_suite_file);
}

fn json_stream_custom() {
    let current_suite = CurrentSuite::new();
    let mut test: Map<String, Value> = Map::new();
    test.insert("title".to_owned(), Value::String("Great".to_owned()));
    let _test_description = get_test_description(&current_suite, &test);
    todo!()
}
