use std::time::{Duration, Instant};

use serde_json::{Map, Value, Number};


/// Create a clean statistics from the base statistics 
fn clean_statistics() -> Map<String, Value> {

    // Todo: configure stats
    let mut stats = Map::new();

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

pub fn json_stream_custom() {
    clean_statistics();
    todo!();
}