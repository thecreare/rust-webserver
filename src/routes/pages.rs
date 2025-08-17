use std::{ffi::OsString, path::{Path, PathBuf}};

use axum::{body::Body, extract, response::{IntoResponse, Response}};
use axum_template::RenderHtml;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::{fs, io::AsyncReadExt};
use toml::value::Datetime;
use tracing::error;

use crate::{util::{md_to_string, open_file}, AppEngine};

#[derive(Debug, Serialize)]
struct MarkdownPage {
    markdown: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TomlMetadata {
    title: String,
    date: Datetime,
    short_description: String,
}

#[derive(Debug, Serialize)]
struct EnumeratedPage {
    meta: TomlMetadata,
    path: String,
}

#[derive(Debug, Serialize)]
struct PageList {
    pages: Vec<EnumeratedPage>,
}

async fn enumerate_pages(
    path: PathBuf,
) -> Result<PageList, Box<dyn std::error::Error + 'static>> {
    let mut paths = fs::read_dir(path).await?;

    let mut post_list = PageList {
        pages: Vec::new(),
    };

    // For each toml file
    while let Some(path) = paths.next_entry().await? {
        if Path::new(&path.file_name()).extension().unwrap_or(&OsString::default()) == "toml" {

            // Read toml
            let toml_str = fs::read_to_string(path.path()).await?;
            let toml_post: TomlMetadata = toml::from_str(&toml_str)?;
            
            // Figure out path
            let path_string = path
                .path()
                .with_extension("")
                .strip_prefix("pages/").ok()
                .and_then(|stem| {
                    stem.to_str()
                }).and_then(|str| {
                    Some(str.to_string())
                });
            
            let path_string = match path_string {
                Some(path_string) => path_string,
                None => {
                    error!("Post at {path:?} has invalid unicode");
                    continue;
                },
            };

            // Push page to list
            post_list.pages.push(EnumeratedPage {
                meta: toml_post,
                path: path_string.to_string(),
            });
        }
    }

    Ok(post_list)   
}

/// Match a query like `projects/evolve-3d` to an actual file or directory
pub async fn find_page(
    path: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error + 'static>> {
    let searching_name = path.file_stem().ok_or("Invalid path")?;
    let mut read_dir = fs::read_dir(path.parent().ok_or("Invalid path")?).await?;
    loop {
        if let Ok(Some(dir_entry)) = read_dir.next_entry().await {
            let dir_path = dir_entry.path();
            
            if dir_path.is_dir() && dir_path.with_extension("").ends_with(searching_name) {
                return Ok(dir_path);
            }
            
            let dir_extension = dir_path.extension().ok_or("Invalid path")?;
            if dir_path.file_stem() == Some(searching_name) && dir_extension != "toml" {
                return Ok(path.with_extension(dir_extension));
            }
        } else {
            return Err("No matching page found".into());
        }
    }
}

pub async fn load_page(
    engine: AppEngine,
    extract::Path(path_name): extract::Path<String>,
) -> impl IntoResponse {
    let mut path = PathBuf::from("pages");
    path.push(&path_name);

    // Find matching file
    path = match find_page(&path).await {
        Ok(path) => path,
        Err(e) => return Err((StatusCode::NOT_FOUND, format!("{:?}", e))),
    };
    
    // Handle directories of pages
    if path.is_dir() {
        let post_list = enumerate_pages(path).await;
        return match post_list {
            Ok(post_list) => Ok(RenderHtml("page_list.html", engine, post_list).into_response()),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e))),
        };
    }

    // Open file
    let (mut file, _content_type, _filename) = match open_file(&path).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // If file is markdown, return a markdown page
    if path.extension().unwrap() == "md" {
        let markdown = md_to_string(&path).await.unwrap();
        let post = MarkdownPage { markdown };
        return Ok(RenderHtml("page.html", engine, post).into_response());
    }

    // Return the file contents
    let mut content = String::new();
    match file.read_to_string(&mut content).await {
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read file: {}", err))),
        Ok(_) => (),
    }
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(content))
        .unwrap())
}

pub async fn load_indexmd(
    engine: AppEngine,
) -> impl IntoResponse {
    let markdown = match md_to_string(std::path::Path::new("pages/index.md")).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let page = MarkdownPage { markdown };

    Ok(RenderHtml("page.html", engine, page))
}