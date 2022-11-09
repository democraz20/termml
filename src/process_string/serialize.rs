use serde::Deserialize;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use std::ops::Index;

#[derive(Debug, Deserialize)]
pub struct IndexMain {
    pub doctype: String,
    pub stylesheet: Option<String>,
    pub head: IndexChild,
    pub body: Vec<IndexChild>,
}

#[derive(Debug, Deserialize)]
pub struct IndexChild {
    pub tag: String,
    pub value: String,
    pub class: Option<String>,
    pub link: Option<String>
}

pub fn get_index_mark_up(filename: &str) -> IndexMain {
    let file = match fs::read_to_string(filename){
        Ok(x) => x,
        Err(e) => {
            return IndexMain {
                doctype: String::from("TERMML"),
                head: IndexChild {
                    tag: String::from("head"),
                    value: String::from("Error : Failed to parse termML file"),
                    class: None,
                    link: None
                },
                stylesheet: None,
                body: vec![
                    IndexChild {
                        tag: String::from("body"),
                        value: String::from(format!("Filename: {}\nInfo : {}", filename, e)),
                        class: None,
                        link: None
                    }
                ]
            }
        }
    };
    match serial(&file) {
        Ok(s) => return s,
        //this is the error parsing struct
        Err(e) => {
            return IndexMain {
                doctype: String::from("TERMML"),
                head: IndexChild {
                    tag: String::from("head"),
                    value: String::from("Error : Failed to parse termML file"),
                    class: None,
                    link: None
                },
                stylesheet: None,
                body: vec![
                    IndexChild {
                        tag: String::from("body"),
                        value: String::from(format!("Filename: {}\nInfo : {}", filename, e)),
                        class: None,
                        link: None
                    }
                ]
            }
        }
    }
}

//should parse ANYTHING now
fn serial<'a, T: Deserialize<'a>>(file: &'a str) 
-> Result<T, Box<dyn Error>> {
    // let file = match fs::read_to_string(filename) {
    //     Ok(file) => &file,
    //     Err(e) => return Err(Box::new(e))
    // };
    // {
    //     let borrow = file;
    //     let parse: T = toml::from_str(&borrow)?;
    
    //     Ok(parse)
    // }
    let parse: T = toml::from_str(&file)?;
    Ok(parse)
}

#[derive(Debug, Deserialize)]
pub struct StyleMain {
    pub styles: Vec<StyleChild>
}

#[derive(Debug, Deserialize)]   
pub struct StyleChild {
    pub tag: String,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub wrap: Option<u16>,
    pub margin: Option<u8>,
}

pub fn get_styles_mark_up(filename: &str) -> Result<StyleMain, Box<dyn Error>> {
    let mut file = match fs::read_to_string(filename){
        Ok(x) => x,
        Err(e) => {return Err(Box::new(e))}
    };
    file = file.replace("\n", "");
    file = file.replace("\r", "");
    match serial(&file) {
        Ok(s) => return Ok(s),
        //this is the error parsing struct
        Err(e) => {
            return Err(e)
        }
    }
}