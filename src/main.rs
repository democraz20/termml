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
    // let html_text = r#"
    // <body>
    //     Hello, World!
    //     <div>Some div</div>
    //     Goodbye
    // </body>
    // "#;
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
    let splitted_tags = split_tags(text.to_string());
    dbg!(splitted_tags);
    // process_text(r#"<div class="test" link="github.com">text<waow></div>"#.to_string());
    printallocd("main");
}
//input will be ex: <div>text</div>

fn split_tags(text: String) -> Vec<String> {
    let split = text.split("><");
    printallocd("split_tags");
    let mut untreated_list = split.map(String::from).collect::<Vec<String>>();
    if untreated_list.len() >= 3 {
        //reserve
        let len: usize = untreated_list.len();
        let first = untreated_list[0].clone()+">";
        let last = "<".to_string()+&untreated_list[len-1].clone();
        // println!("first:'{}' second:'{}'", first, last);
        //remove the first and last items so things can iterate flawlessly
        for i in 0..len { 
            //loop with the number of items instead
            untreated_list[i] = "<".to_string()+&untreated_list[i]+&">".to_string();
        } 
        untreated_list[0] = first.clone();
        printallocd("68");
        drop(first);
        printallocd("70");
        untreated_list[len-1] = last.clone();
        drop(last);
        drop(len);
        return untreated_list;
    } else if untreated_list.len() == 2 {

    } else {
        //reserve for no element
    }
    //list will never have 1 item in it
    //either more or less (none or more)
    vec![]
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
    printallocd("process_text");
}

fn printallocd(header: &str) -> () {
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());  
}