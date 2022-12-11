use std::{path::Path, error::Error};
use crate::utility::clean_directory;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new("sample_dir");
    clean_directory(dir_path)?;

    Ok(())
}