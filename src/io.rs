use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};
use crate::models::User;
use crate::errors::{Result, AppError};
use std::path::Path;

pub async fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path).await.map_err(AppError::IoError)
}

pub async fn write_output(data: &[User], output_path: Option<&String>) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    
    if let Some(path) = output_path {
        let mut file = File::create(path).await?;
        file.write_all(json.as_bytes()).await?;
    } else {
        println!("{}", json);
    }
    
    Ok(())
}
