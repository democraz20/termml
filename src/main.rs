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
    dbg!(g);
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
