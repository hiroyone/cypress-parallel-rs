use std::{path::{Path, PathBuf}, fs, io::Result, process, time::Duration, collections::HashMap};

/// Remove all files in the directory only if the directory exists. Do nothing if not.
///
/// # Errors
///
/// This function will return an error if the directory operation fails.
fn clean_directory(dir_path:&Path) -> Result<()> {    
    if dir_path.is_dir() {
        println!("The directory {:?} already exists!", dir_path);
        fs::remove_dir_all(&dir_path)?;
        fs::create_dir_all(dir_path)?;
    }
    return Ok(())
}

#[derive(Debug)]
struct SpecWeight {
    time: Duration,
    weight: u64
}

type SpecWeights<'a> = HashMap<&'a str, SpecWeight>;
type TotalWeight = u64;

/// Create weights for parallel computing based on the ratio of an execution time to the total time
#[allow(dead_code)]
fn generate_weights (spec_weights:&mut SpecWeights, total_duration:Duration, total_weight:TotalWeight) {
    for (_, spec_weight) in spec_weights.iter_mut() {
        let ratio = (spec_weight.time.as_millis() / total_duration.as_millis()) as u64 ;
        spec_weight.weight =((ratio * total_weight) as f64).floor() as u64 ;
    }
}

#[test]
fn generate_weights_test() {

    let mut spec_weights:SpecWeights = HashMap::from([
        ("sample", SpecWeight {
            time: Duration::from_millis(500),
            weight:0
        })
    ]) ;

    let total_weight:TotalWeight = 1000;

    generate_weights(&mut spec_weights, Duration::from_millis(100), total_weight);

    assert_eq!(spec_weights["sample"].weight, 5000);
}

type CyRunResults = HashMap<PathBuf, String>;

/// Gather Cypress results from the result directory
///
/// # Errors
///
/// This function will return an error if the directory path does not exist.
fn collect_cy_results (result_path: &Path) -> Result<CyRunResults> {
    let mut results:CyRunResults = HashMap::new();

    for entry in fs::read_dir(result_path)? {
        let path = entry?.path();
        if !path.is_dir() {
            collect_cy_results(&path)?;
        } else {
            let content = fs::read_to_string(&path)?;
            results.insert(path, content);
        }
    }

    return Ok(results)
}


fn main() {
    if let Err(e) = clean_directory(Path::new("distdwamo")) {
        println!("Application Error: {}", e);
        process::exit(1)
    }

    let duration = Duration::from_millis(2569210323242);
    println!("{:?}", duration);

}
