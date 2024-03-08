use html_data_extractor::{website::website_def::Website, create_initial_dir};
use std::env;

/* url is user input */
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    create_initial_dir().unwrap();
    let mut website = Website::new(url).await; 
    //println!("{:?}", website.text);
    website.search_for_files();
    website.request_files().await;
}