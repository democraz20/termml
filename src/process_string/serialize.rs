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

pub fn serial() -> Result<TOMLmain, Box<dyn Error>> {
    let file = match fs::read_to_string("example.toml") {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e))
    };
    let parse: TOMLmain = toml::from_str(&file)?;
    
    Ok(parse)
}