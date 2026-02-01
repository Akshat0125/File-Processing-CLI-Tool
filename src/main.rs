mod cli;
mod io;
mod parser;
mod errors;
mod models;

use clap::Parser;
use cli::Cli;
use models::User;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    
    // Using a concurrent vector to collect results is one way, 
    // but simply collecting Futures and awaiting them is cleaner.
    let mut handles = Vec::new();

    for file_path in args.input_files {
        let handle = tokio::spawn(async move {
            process_file(file_path).await
        });
        handles.push(handle);
    }

    let mut all_users = Vec::new();
    let mut errors = Vec::new();

    for handle in handles {
        match handle.await {
            Ok(result) => {
                match result {
                    Ok(mut users) => all_users.append(&mut users),
                    Err(e) => errors.push(e),
                }
            }
            Err(e) => eprintln!("Task execution error: {}", e),
        }
    }

    if !errors.is_empty() {
        eprintln!("Encountered {} errors during processing:", errors.len());
        for err in errors {
            eprintln!("  - {}", err);
        }
    }

    if !all_users.is_empty() {
        if let Err(e) = io::write_output(&all_users, args.output.as_ref()).await {
            eprintln!("Failed to write output: {}", e);
        }
    }
}

async fn process_file(path: String) -> errors::Result<Vec<User>> {
    let content = io::read_file(&path).await?;
    let extension = Path::new(&path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase()) // Simple normalization
        .unwrap_or_else(|| "txt".to_string());
    
    parser::parse(&content, &extension)
}
