use serde::Deserialize;
use std::error::Error;
use std::fs;
use crate::static_data::structs::{
    IndexMain,
    StyleMain,
    // StyleChild,
};


pub fn get_index_mark_up(filename: &str) -> IndexMain {
    let file = match fs::read_to_string(filename){
        Ok(x) => x,
        Err(e) => {
            return IndexMain::new_error(filename, e)
        }
    };
    match serialize(&file) {
        Ok(s) => return s,
        //this is the error parsing struct
        Err(e) => {
            return IndexMain::new_error(filename, e)
        }
    }
}

//should parse ANYTHING now
fn serialize<'a, T: Deserialize<'a>>(file: &'a str) 
-> Result<T, Box<dyn Error>> {
    let parse: T = toml::from_str(&file)?;
    Ok(parse)
}

pub fn get_styles_mark_up(filename: &str) -> Result<StyleMain, Box<dyn Error>> {
    let mut file = match fs::read_to_string(filename){
        Ok(x) => x,
        Err(e) => {return Err(Box::new(e))}
    };
    file = file.replace("\n", "");
    file = file.replace("\r", "");
    match serialize(&file) {
        Ok(s) => return Ok(s),
        //this is the error parsing struct
        Err(e) => {
            return Err(e)
        }
    }
}