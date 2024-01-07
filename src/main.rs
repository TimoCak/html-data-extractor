use html_data_extractor::{website::website_def::Website, create_initial_dir};

/* url is user input */
#[tokio::main]
async fn main() {
    create_initial_dir().unwrap();
    let website = Website::new("https://youtu.be/6Mopunhrtgo").await; 
    println!("{:?}", website.text);

}