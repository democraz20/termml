use serde_derive::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct TOMLmain {
    doctype: String,
    head: TOMLchild,
    body: Vec<TOMLchild>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLchild {
    tag: String,
    value: String,
    class: Option<String>,
    link: Option<String>
}

pub fn getMarkUp(filename: &str) -> TOMLmain {
    match serial(filename) {
        Ok(s) => return s,
        //this is the error parsing struct
        Err(e) => {
            return TOMLmain {
                doctype: String::from("TERMML"),
                head: TOMLchild {
                    tag: String::from("head"),
                    value: String::from("Error : Failed to parse termML file"),
                    class: None,
                    link: None
                },
                body: vec![
                    TOMLchild {
                        tag: String::from("body"),
                        value: String::from(format!("Info : {}", e)),
                        class: None,
                        link: None
                    }
                ]
            }
        }
    }
}

pub fn serial(filename: &str) -> Result<TOMLmain, Box<dyn Error>> {
    let file = match fs::read_to_string(filename) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e))
    };
    let parse: TOMLmain = toml::from_str(&file)?;
    
    Ok(parse)
}