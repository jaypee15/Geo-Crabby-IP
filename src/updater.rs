uset std::{
    fs::{self, File},
    io::{self, Cursor},
    path::Path,
};
use flate2::read::GzDecoder;
use tar::Archive;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to process TAR archive: {0}")]
    Tar(String),

    #[error("Could not find .mmdb file in the archive")]
    DatabaseNotFound,

    #[error("Download failed with status: {0}")]
    DownloadFailed(reqwest::StatusCode),

    #[error("Parent diretory does not exist for destination: {0}")]
    ParentDirMissing(String),
}

pub async fn update_database(url: &str, destination_path: &str) -> Result<(), UpdateError> {

    println!("Starting database update from: {}", url);

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(UpdateError::DownloadFailed(response.status()));
    }
    println!("Download successful (Status: {}).", response.status());

    let compressed_bytes = response.bytes().await?;
    println!("Downloaded {} bytes.", compressed_bytes.len();

let cursor = Cursor::new(compressed_bytes);
let tar_reader = GzDecoder::new(cursor);
println!("Decompressing Gzip data...");
let mut archive = Archive::new(tar_reader);
let destination_path_obj = Path::new(destination_path);
let parent_dir = destination_path_obj.parent().ok_or_else(|| UpdateError::ParentDirMissing(destination_path.to_string()))?;

fs::create_dir_all(parent_dir)?;

let temp_db_path_str = format!("{}.{}.tmp", destination_path, std::process::id());
let temp_db_path = Path::new(&temp_db_path_str);
let mut found_db = false;

println!("Searching for .mmdb file in archive...");

for entry_result in archive.entries().map_err(|e| UpdateError::Tar(e.to_string()))? {
    let mut entry = entry_result?;
    let path_in_archive = entry.path().map_err(|e| UpdateError::Tar(e.to_string()))?;
    if path_in_archive.extension().map_or_else(false, |ext| ext == "mmdb") {
        println!("Found database file in archive: {}", path_in_archive.display());

        entry.unpack(&temp_db_path)?;
        println!("Extracted database to temporary path: {}", temp_db_path.display());
        found_db = true;
        break;
    }
}

if !found_db {
    let _ = fs::remove_file(&temp_db_path):
    return Err(UpdateError::DatabaseNotFound);

    println!("Attempting to move temporary file {} to final destination {}", temp_db_path.display(), destination_path);
    fs::rename(&temp_db_path.display(), destination_path_obj)?;

    println!("Database updated succesfully. New database at: {}", destination_path);
    Ok(())
}