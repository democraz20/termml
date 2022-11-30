mod process_string;
mod static_data;

use std::{
    alloc, fs, collections::HashMap
};
use hard_xml::{XmlWrite};

//tracking memory usage
use cap::Cap;
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
use crate::process_string::bond::{
    parse_style_sheet,
    styles_hash
};
use crate::static_data::structs::{
    StyleMain,
    StyleChild,
    TermmlMain,
    Doctype,
    Head, Body, Div,
    StyleSheet,
    Require
};
fn main() {
    start();
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    
    dbg!(TermmlMain {
        doctype: Doctype {ml: "termml".into()},
        require: Some(Require {
            stylesheet: vec![
                StyleSheet { name: Some("styles.termss".into())}
                ]
            }), //Require
            head: Head {
                value: Div {
                    class: None,
                    value: "Error while parsing Termml file".into()
                },
            },
            body: Body {
                value: vec![
                    Div {
                        class: None,
                        value: format!("Message : ").into()
                    }
                    ]
                }
            }.to_string().unwrap()
        );
    }

fn start() {
    dbg!(styles_hash());
}

fn _alloced(header: Option<&str>) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    match header {
        Some(h) => {
            println!("{} | Allocated : {} B(ytes)", h, ALLOCATOR.allocated());
        },
        None => {
            println!("  | Allocated : {} B(ytes)", ALLOCATOR.allocated());
        }
    }
}
