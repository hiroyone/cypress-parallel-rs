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