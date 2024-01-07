use super::file_def::FileDef;
use std::fs::{self, File};
use std::io::{Result, Write};

pub struct Website {
    pub url: String,
    pub dir: String,
    pub files: Vec<FileDef>,
    pub timestamp: String,
    pub text: String,
}

impl Website {
    pub async fn new(url: &str) -> Website {
        let text = request_webpage(&url).await;
        let dir = create_folder(&url).unwrap();
        create_html_file(&dir, &text).unwrap();

        //need to implement
        let timestamp = String::default();
        let files = vec![];

        Website {
            url: url.to_string(),
            dir,
            files,
            timestamp,
            text,
        }
    }


}

async fn request_webpage(url: &str) -> String {
    
    let resp = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    resp
}

fn create_folder(url: &str) -> Result<String> {
    let folder_name = url.replace(".", "%").replace("/", "!");
    fs::create_dir_all(format!("output/{}", folder_name))?;
    Ok(folder_name)
}

fn create_html_file(dir: &str, text: &str) -> Result<()> {
    let mut file = File::create(format!("output/{}/main.html", dir))?;
    file.write(text.as_bytes())?;

    Ok(())
}