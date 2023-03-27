use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use bytes::Bytes;
use regex::Regex;
use reqwest::IntoUrl;
use tokio::spawn;

static THEME_PATH: &str = "theme/";
static CDN_ROOT: &str = "https://cdn.jsdelivr.net/npm/katex@0.12.0/dist/";
static KATEX_CSS_PATH: &str = "katex.min.css";

fn write_file_bytes(filename: &str, bytes: &[u8]) {
    let path = Path::new(filename);
    create_dir_all(path.parent().expect("failed to get parent")).expect("failed to create parent");
    File::create(path)
        .expect("failed to create file")
        .write_all(bytes)
        .expect("failed to write bytes")
}

async fn download_bytes(url: impl IntoUrl) -> Bytes {
    let response = reqwest::get(url).await.expect("failed to get request");
    if !response.status().is_success() {
        panic!("response status not success");
    }
    response.bytes().await.expect("failed to download bytes")
}

async fn download_save(url_base: &str, path: &str, path_base: &str) -> Bytes {
    let url = format!("{url_base}{path}");
    let file_path = format!("{path_base}{path}");
    println!("Downloading from {url} to {file_path}");

    let file_bytes = download_bytes(url).await;
    write_file_bytes(&file_path, &file_bytes);
    file_bytes
}

#[tokio::main]
async fn main() {
    let css_bytes = download_save(CDN_ROOT, KATEX_CSS_PATH, "").await;
    let css_str = String::from_utf8(css_bytes.to_vec()).expect("Failed to decode CSS bytes.");
    let url_pattern = Regex::new(r"url\((fonts/[^()]+)\)").unwrap();
    let mut tasks = Vec::new();
    // Get URLs to fonts `katex.min.css` relies on.
    for capture in url_pattern.captures_iter(&css_str) {
        let path = capture
            .get(1)
            .expect("capture does not have a first element")
            .as_str()
            .to_owned();
        tasks.push(spawn(async move {
            download_save(CDN_ROOT, &path, THEME_PATH).await;
        }));
    }
    // Finish all download tasks.
    for task in tasks {
        task.await.expect("a task failed");
    }
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test() {
        main()
    }
}
