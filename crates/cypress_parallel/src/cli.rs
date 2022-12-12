use tokio::task::JoinHandle;
use tokio::time::Instant;

use crate::test_suites;
use crate::threads;
use crate::threads::Thread;
use crate::utility::clean_directory;
use std::error::Error;
use std::path::Path;
use std::process::ExitStatus;
use tokio::io;

async fn parallel_execute(test_weight_threads: Vec<Thread>) -> Result<(), Box<dyn Error>> {
    let handles: Vec<JoinHandle<Result<ExitStatus, io::Error>>> = test_weight_threads
        .into_iter()
        .enumerate()
        .map(|(index, thread)| {
            tokio::spawn(async move {
                threads::execute_thread(&thread, index.try_into().unwrap()).await
            })
        })
        .collect();

    // Todo: Set a proper start and end
    let start = Instant::now();

    for handle in handles {
        // Todo: remove double questions
        handle.await??;
    }
    println!(
        "It took {} seconds to execute tests",
        start.elapsed().as_secs()
    );

    Ok(())
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    // Todo: configure the proper path
    let dir_path = Path::new("sample_dir");
    clean_directory(dir_path)?;

    let test_weight_threads = test_suites::get_test_weight_threads()?;
    parallel_execute(test_weight_threads).await?;

    Ok(())
}
