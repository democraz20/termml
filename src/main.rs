mod process_string;
mod static_data;
// use crate::process_string::serialize::{self, is_tag};

//tracking memory usage
use cap::Cap;
use std::alloc;

use ansi_term::{Colour};

use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};

use crate::static_data::term_style::{get_color_from_string, construct_styles};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

const INDEX_FILENAME: &str = "index.termml";

#[derive(XmlWrite, XmlRead, PartialEq, Debug, Clone)]
#[xml(tag = "div")]
struct test_xml<'a>{
    #[xml(attr = "attr")]
    attr: Cow<'a, str>,
    // #[xml(text)]
    // text: Cow<'a, str>
}
fn main() {
    let style = construct_styles(
        Colour::White, Colour::Black, true, false);
    println!("{}", style.paint("hello style test\n newline"));
    println!("This is in Red and Green: {}, {}", Colour::Red.paint("Hello"), Colour::Green.paint("World!"));
    let xml_text = r#"<div attr="1"></div"#;
    let parsed = test_xml::from_str(xml_text).unwrap();
    dbg!(&parsed);
    println!("{}", parsed.attr);
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    // dbg!(get_color_from_string(""));
}

fn _printallocd(header: &str) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}
