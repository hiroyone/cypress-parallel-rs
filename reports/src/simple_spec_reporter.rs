use serde_json::{Map, Value};

type CurrentSuite = Map<String, Value>;


pub fn json_stream_custom() {
    todo!()
} 

fn get_suite_titles(current_suite:& CurrentSuite) -> String{
    let mut result = String::new();

    let mut current = current_suite.clone();

    while current.contains_key("parent") {
        result = format!("{} - {}", current_suite["title"], result) ;

        if let Value::Object(map) = &current["parent"] {
            current =  map.clone();
        }
    }
    return result

}
