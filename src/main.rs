mod process_string;
mod static_data;
// use crate::process_string::serialize::{self, is_tag};

//tracking memory usage
use cap::Cap;
use std::alloc;
use ansi_term::Colour;

use std::borrow::Cow;
use std::fs;
use strong_xml::{XmlRead, XmlWrite};

use crate::static_data::term_style::{construct_styles, get_color_from_string};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

fn main() {
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    // dbg!(get_color_from_string(""));
}

fn _printallocd(header: &str) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());
}
