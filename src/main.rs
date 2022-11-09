mod process_string;
mod static_data;
// use crate::process_string::serialize::{self, is_tag};

//tracking memory usage
use cap::Cap;
use std::alloc;

use ansi_term::Colour::{
    Red,
    Green,
};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

const INDEX_FILENAME: &str = "index.termml";

fn main() {
    //enable ansi mode
    // let enabled = ansi_term::enable_ansi_support();
    // drop(enabled); //returns a result, no longer used, no error propagation, can be dropped
    //might not even need to enable ansi at all?
    println!("This is in Red and Green: {}, {}", Red.paint("Hello"), Green.paint("World!"));
    //reserve tag : "body" to wrap the whole thing and maybe "head"
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    // let html_text = r#"
    // <body>
    //     Hello, World!
    //     <div>Some div</div>
    //     Goodbye
    // </body>
    // "#;
    let parsed = process_string::serialize::get_index_mark_up(INDEX_FILENAME);

    // let styles = match process_string::serialize::get_styles_mark_up(&parsed.stylesheet.as_ref().unwrap()) {
    //     Ok(styles) => styles,
    //     Err(e) => {
    //         panic!("failed to serialize styles : {}", e) //return an empty stylesheet here ad log for error
    //     }
    // };
    println!("{}", parsed.doctype);
    dbg!(parsed);
    // dbg!(styles);
    // dbg!(styles);
    // process_text(r#"<div class="test" link="github.com">text<waow></div>"#.to_string());
}
//input will be ex: <div>text</div>

fn _printallocd(header: &str) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}
