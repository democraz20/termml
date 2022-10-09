use colored::*;
use easy_scraper::Pattern;

fn main() {
    //reserve tag : "body" to wrap the whole thing and maybe "head"
    let html_text = r#"
    <body>
        Hello, World!
        <div>Some div</div>
        Goodbye
    </body>
    "#;
    // let pat = Pattern::new(r#"<div class="{{class_name}}">{{mes}}</div>"#).unwrap();
    let pat1 = Pattern::new(r#"<body>{{body_text:*}}</body>"#).unwrap();
    let first_match = pat1.matches(html_text);
    dbg!("{}", first_match);
    // println!("{}", ms[0]["mes"]);
    // println!("class=\"{}\"", ms[0]["class_name"]);
    println!("{}, {}!", "Hello".red(), "World".green().bold());
}
