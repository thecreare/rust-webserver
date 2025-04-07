use hyper::StatusCode;
use tokio::io::AsyncReadExt;

use crate::util;

// TODO: Should be cached, maybe generate these at compile time
pub async fn md_to_string(path: &std::path::Path) -> Result<String, (StatusCode, String)> {
    // Open markdown file
    let (mut file, _content_type, _filename) = match util::open_file(&path).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // Read file
    let mut markdown = String::new();
    match file.read_to_string(&mut markdown).await {
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file: {}", err))),
        Ok(_) => (),
    }

    // Parse markdown and return html
    Ok(markdown::to_html(&markdown))
}