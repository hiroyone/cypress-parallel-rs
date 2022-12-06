use std::collections::HashMap;

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

