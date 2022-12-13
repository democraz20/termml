use std::{collections::HashMap, fs, hash};

use crate::static_data::structs::{Require, StyleChild, StyleMain, StyleSheet, TermmlMain};
use ansi_term::Style;
use hard_xml::XmlRead;

pub fn markup_entry(text: String) -> () {
    let markup = match TermmlMain::from_str(text.as_str()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error while parsing: {}", e);
            TermmlMain::parse_error("filename.termml", e)
        }
    };
    dbg!(&markup);
    let required_entry = markup.require;
    let required = match required_entry {
        Some(s) => s,
        None => {
            //needs error value for this
            Require {
                stylesheet: vec![StyleSheet {
                    // name: Some("styles.termss".into()),
                    name: "styles.termss".into()
                }],
            }
        }
    };
    let requiredvec = required.stylesheet;
    let hashmap = styles_hash(requiredvec);
    dbg!(hashmap);
}

pub fn styles_hash(required: Vec<StyleSheet>) -> HashMap<String, StyleChild> {
    //imagine a getter from Require termml page
    let mut stylesmap: HashMap<String, StyleChild> = HashMap::new();

    for stylesheet in required {
        let i: String = stylesheet.name.into();
        let file = fs::read_to_string(i.as_str()).unwrap();

        let s = i.split(".");
        let mut t = s.clone().map(String::from).collect::<Vec<String>>();
        t.pop();
        let styles_namespace = t.join("."); //splitted ".", rejoin "."
        drop(s);
        drop(t);

        let styles = parse_style_sheet(file);
        for i in styles.styles {
            stylesmap.insert(
                format!("{}::{}", styles_namespace, i.class.clone()),
                i,
            );
        }
    }

    return stylesmap;
}

pub fn parse_style_sheet(file: String) -> StyleMain {
    let mut file = file.replace("\n", "");
    file = file.replace("\r", "");

    match toml::from_str(&file) {
        Ok(p) => p,
        Err(e) => {
            println!("error while parsing: {}", e);
            StyleMain {
                styles: vec![StyleChild {
                    class: String::from("null"),
                    background: None,
                    foreground: None,
                    underline: None,
                    bold: None,
                    wrap: None,
                }],
            }
        }
    }
}