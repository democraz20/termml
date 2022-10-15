use colored::*;
use smallvec::{SmallVec, smallvec};

//tracking memory usage
use cap::Cap;
use std::alloc;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());


#[derive(Debug)]
struct TextFormat {
    tag: String,
    //attributes
    class: String,
    link: String,
    text: String,
}


fn main() {
    //reserve tag : "body" to wrap the whole thing and maybe "head"
    // ALLOCATOR.set_limit(30 * 1024 * 1024).unwrap();
    let html_text = r#"
    <body>
        Hello, World!
        <div>Some div</div>
        Goodbye
    </body>
    "#;
    let data = TextFormat {
        tag: "div".to_string(), 
        class: "".to_string(),
        link: "".to_string(), 
        text: "Hello, World!".to_string()
    };
    println!("{}, {}, {}, {}", data.tag, data.class, data.link, data.text);

    dbg!("{}",data);

    // let pat = Pattern::new(r#"<div class="{{class_name}}">{{mes}}</div>"#).unwrap();
    // println!("class=\"{}\"", ms[0]["class_name"]);
    println!("{}, {}!", "Hello".red(), "World".green().bold());
    let text = r#"<div>hello</div><a>world</a><b>!</b>"#;
    println!("{:?}", split_tags(text.to_string()));
    // process_text(r#"<div class="test" link="github.com">text<waow></div>"#.to_string());
    println!("main|Currently allocated: {}B", ALLOCATOR.allocated());
}
//input will be ex: <div>text</div>

fn split_tags(text: String) -> Vec<String> {
    let split = text.split("><");
    println!("split_tags|Currently allocated: {}B", ALLOCATOR.allocated());
    split.map(String::from).collect::<Vec<String>>()
}

fn process_text(text: String) -> () {
    let split = text.split("<");
    //at most a line of it would only have 3 items (assuming there's no "<" in the text itself)
    let mut vec = split.clone().map(String::from).collect::<SmallVec<[String; 3]>>();
    //the .clone() allocates memory for the clone but will be dropped after the line since it is not kept
    drop(split); //vec first clones split then splits it, since the original split is no longer used stays idle and we can drop it
    vec.remove(0);
    vec.remove(vec.len()-1);

    let joined = vec.join("<");
    drop(vec);

    let split = joined.split(">");
    let tag = split.clone().map(String::from).collect::<SmallVec<[String; 2]>>();
    drop(split);

    let mut vec_for_text = tag.clone();
    vec_for_text.remove(0);
    

    //some bullshit magic above please ignore
    let final_text = vec_for_text.join(">");
    drop(vec_for_text); //text<waow>
    
    let final_tag = tag[0].clone();
    //div class="test" link="github.com"
    println!("tag: '{}' text: '{}'", final_tag, final_text);
    println!("process_text|Currently allocated: {}B", ALLOCATOR.allocated());
}