use std::ffi::OsStr;

use hyper::StatusCode;
use tokio::fs::File;

/// Opens a file
/// Returns (file, content type, file name) or (status code, error message)
pub async fn open_file(path: &std::path::Path) -> Result<(File, &str, &OsStr), (StatusCode, String)> {
    // Validate file name
    let filename = match path.file_name() {
        Some(name) => name,
        None => return Err((StatusCode::BAD_REQUEST, "File name couldn't be determined".to_string()))
    };

    // Open file
    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err)))
    };

    // Determine content type
    let content_type = match mime_guess::from_path(&path).first_raw() {
        Some(mime) => mime,
        None => return Err((StatusCode::BAD_REQUEST, "MIME Type couldn't be determined".to_string()))
    };

    Ok((file, content_type, filename))
}