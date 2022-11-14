mod process_string;
mod static_data;
// use crate::process_string::serialize::{self, is_tag};

//tracking memory usage
use cap::Cap;
use std::alloc;

use ansi_term::{Colour};

use crate::static_data::term_style::{get_color_from_string, construct_styles};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

const INDEX_FILENAME: &str = "index.termml";

fn main() {
    let style = construct_styles(
        Colour::White, Colour::Black, true, false);
    println!("{}", style.paint("hello style test\n newline"));
    println!("This is in Red and Green: {}, {}", Colour::Red.paint("Hello"), Colour::Green.paint("World!"));
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    let parsed = process_string::serialize::get_index_mark_up(INDEX_FILENAME);
    
    loop {
        let mut In = String::new();
        std::io::stdin().read_line(&mut In);
        println!("color: {}, printing: {}", In, 
        get_color_from_string(&In).paint("kill the jews"))
    }

    dbg!(get_color_from_string(""));
    dbg!(parsed);
}

fn _printallocd(header: &str) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}
