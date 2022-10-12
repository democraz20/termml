use colored::*;


#[derive(Debug)]
struct HtmlStruct {
    tag: String,
    //attributes
    class: String,
    link: String,
    text: String,
}
impl HtmlStruct {
    fn new(tag: String, class: String, link: String, text: String) -> HtmlStruct{
        HtmlStruct {
            tag, class, link, text
        }
    }
}

fn main() {
    //reserve tag : "body" to wrap the whole thing and maybe "head"
    let html_text = r#"
    <body>
        Hello, World!
        <div>Some div</div>
        Goodbye
    </body>
    "#;
    let data = HtmlStruct::new(
        "div".to_string(),
        "".to_string(),
        "".to_string(),
        "Hello, World!".to_string()
    );
    //which is better?
    // let data = HtmlStruct::new {
    //     tag: "div".to_string(), 
    //     class: "".to_string(),
    //     link: "".to_string(), 
    //     text: "Hello, World!".to_string()
    // };

    println!("{}, {}, {}, {}", data.tag, data.class, data.link, data.text);

    dbg!("{}",data);

    // let pat = Pattern::new(r#"<div class="{{class_name}}">{{mes}}</div>"#).unwrap();
    // println!("class=\"{}\"", ms[0]["class_name"]);
    println!("{}, {}!", "Hello".red(), "World".green().bold());
}
