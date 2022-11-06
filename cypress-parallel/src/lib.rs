use std::{path::{Path, PathBuf}, fs, io};

/// Get a list of file paths under the directory
///
/// # Errors
///
/// This function will return an error if the passed-in directory does not exist.
fn get_file_paths_by_dir_path (dir_path:&Path) -> Result<Vec<PathBuf>, io::Error>{
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    
    entries.sort();
    return Ok(entries)
}
