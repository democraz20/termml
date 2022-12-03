mod process_string;
mod static_data;
mod webrequest;

use hard_xml::XmlWrite;
use std::{alloc, collections::HashMap, fs};

//tracking memory usage
use cap::Cap;
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
use crate::process_string::bond::{markup_entry, parse_style_sheet, styles_hash};
use crate::static_data::structs::{
    Body, Div, Doctype, Head, Require, StyleChild, StyleMain, StyleSheet, TermmlMain,
};
use crate::webrequest::request::get;
fn main() {
    let url = String::from("http://127.0.0.1:5500/test.termml");
    let g = get(url.clone());
    // println!("{}", get.unwrap());
    //what a fuckin mess
    let res = match g {
        Ok(r) => r,
        Err(e) => {
            //retry
            match get(url.clone()) {
                Ok(r) => r,
                Err(e) => {
                    match get(url.clone()) {
                        Ok(r) => r,
                        Err(ureq::Error::Status(code, response)) => {
                            TermmlMain::fetch_error(url.as_str(), e, code)
                            .to_string().unwrap()
                        }
                        Err(_)=>{
                            TermmlMain::fetch_error(url.as_str(), e, 0)
                            .to_string().unwrap()
                        }
                    }
                }
            }
        }
    };
    dbg!(res);
    

    // start();
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();

    // dbg!(TermmlMain {
    //     doctype: Doctype {
    //         ml: "termml".into()
    //     },
    //     require: Some(Require {
    //         stylesheet: vec![StyleSheet {
    //             name: Some("styles.termss".into())
    //         }]
    //     }), //Require
    //     head: Head {
    //         value: Div {
    //             class: None,
    //             value: "Error while parsing Termml file".into()
    //         },
    //     },
    //     body: Body {
    //         value: vec![Div {
    //             class: None,
    //             value: format!("Message : ").into()
    //         }]
    //     }
    // }
    // .to_string()
    // .unwrap());
}

fn start() {
    let markup = fs::read_to_string("index.termml").unwrap();
    markup_entry(markup);
    // dbg!(styles_hash());
}

fn _alloced(header: Option<&str>) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    match header {
        Some(h) => {
            println!("{} | Allocated : {} B(ytes)", h, ALLOCATOR.allocated());
        }
        None => {
            println!("  | Allocated : {} B(ytes)", ALLOCATOR.allocated());
        }
    }
}
