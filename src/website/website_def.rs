use regex::Regex;
use std::fs::{self, File};
use std::io::{Result, Write};
use std::process::Command;

pub struct Website {
    pub url: String,
    pub dir: String,
    pub files: Vec<(String, String)>,
    pub timestamp: String,
    pub text: String,
}

impl Website {
    pub async fn new(url: &str) -> Website {
        let text = request_webpage(url).await;
        let dir = create_folder(url).unwrap();
        create_html_file(&dir, &text).unwrap();

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

    pub fn search_for_files(&mut self) {
        let regex =
            Regex::new(r#"<img\s+[^>]*?src=[\'"]([^\'"\s]+)\.(jpg|jpeg|png)["\']"#).unwrap();

        for (_, [file_name, file_type]) in regex.captures_iter(&self.text).map(|c| c.extract()) {
            self.files
                .push((file_name.to_string(), file_type.to_string()));
        }
    }

    pub async fn request_files(&self) {
        for file in &self.files {
            let build_url = format!("{}{}.{}", self.url, file.0, file.1);
            let path = format!(
                "output/{}/{}.{}",
                self.dir,
                file.0.replace("/", "%"),
                file.1
            );

            println!("{}", &build_url);
            let output = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args([format!(
                        "Invoke-Webrequest {} -OutFile {}",
                        &build_url, &path
                    )])
                    .output()
                    .expect("failed to execute process");
            } else {
                Command::new("bash")
                    .arg(format!("sudo wget -O {} {}", &path, &build_url))
                    .output()
                    .expect("failed to execute process");
            };
            println!("{:?}", &output);
        }
    }
}

async fn request_webpage(url: &str) -> String {
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    resp
}

fn create_folder(url: &str) -> Result<String> {
    let folder_name = url
        .replace(".", "%")
        .replace("/", "%")
        .replace(":", "%")
        .replace("&", "%")
        .replace("?", "%");

    println!("{}", &folder_name);
    fs::create_dir_all(format!("output/{}", folder_name))?;
    Ok(folder_name)
}

fn create_html_file(dir: &str, text: &str) -> Result<()> {
    let mut file = File::create(format!("output/{}/main.html", dir))?;
    file.write(text.as_bytes())?;

    Ok(())
}
