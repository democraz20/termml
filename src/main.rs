mod process_string;
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

#[derive(Debug)]
//attributes
pub struct TextFormat {
    tag: String,
    text: String,
    class: String,
    link: String,
}

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
    let data = TextFormat {
        tag: "div".to_string(), 
        text: "Hello, World!".to_string(),
        class: "".to_string(),
        link: "".to_string(), 
    };

    dbg!("{}",data);

    
    let text = r"<div>hello</div>
<a>world</a>
<b>!</b>";
    // process_text(r#"<div class="test" link="github.com">text<waow></div>"#.to_string());
}
//input will be ex: <div>text</div>

fn _printallocd(header: &str) -> () {
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}