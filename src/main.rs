mod process_string;
use crate::process_string::serialize;

use colored::*;

//tracking memory usage
use cap::Cap;
use std::alloc;

// use crate::{process_string};

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

    println!("{}, {}!", "Hello".red(), "World".green().bold());
    let text = r#"<div>hello</div>
<a>world</a>
<b>!</b>"#;
    let splitted_tags = serialize::split_tags(text.to_string());
    dbg!(&splitted_tags);
    for i in splitted_tags {
        dbg!(serialize::process_text(i));
    }
    // process_text(r#"<div class="test" link="github.com">text<waow></div>"#.to_string());
}
//input will be ex: <div>text</div>

fn _printallocd(header: &str) -> () {
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}