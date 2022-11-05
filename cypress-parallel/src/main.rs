use std::{path::Path, fs, io::Result, process};


/// Remove all files in the directory only if the directory exists. Do nothing if not.
///
/// # Errors
///
/// This function will return an error if the directory operation fails.
fn clean_directory(dir_path:&str) -> Result<()> {    
    if Path::new(&dir_path).is_dir() {
        println!("The directory {} already exists!", dir_path);
        fs::remove_dir_all(&dir_path)?;
        fs::create_dir_all(dir_path)?;
    }
    return Ok(())
}


fn main() {
    if let Err(e) = clean_directory("distdwamo") {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
