pub enum FileType {
    HTML,
    CSS,
    JS,
    JPG,
    PNG,
    WEBP,
    SVG,
}

pub struct FileDef {
    pub file_type: FileType,
    pub path: String,
    pub text: String,
}

impl FileDef {

    //creates a new file definition
    pub fn new(file_type: FileType, path: String, text: String) -> FileDef {
        //set timestamp
        FileDef {
            file_type,
            path,
            text,
        }
    }

    pub fn match_file_type(file_type: FileType) -> String {
        match file_type {
            FileType::HTML => "html".to_string(),
            FileType::CSS => "css".to_string(),
            FileType::JS => "js".to_string(),
            FileType::JPG => "jpg".to_string(),
            FileType::PNG => "png".to_string(),
            FileType::WEBP => "webp".to_string(),
            FileType::SVG => "svg".to_string(),
        }
    }
       
}