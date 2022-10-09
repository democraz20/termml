use colored::*;
use scraper::{Html, Selector};

fn main() {
    let html_text = r#"<h1>Hello, World!<div>waow</div></h1> <h1>waow</h1>"#;
    let fragment = Html::parse_fragment(html_text);
    let h1_selector = Selector::parse("h1").unwrap();
    let h1 = fragment.select(&h1_selector).next().unwrap();
    let h1_text = h1.text().collect::<Vec<_>>();

    println!("{:?}", h1_text);

    // let element = fragment.select(&h1Selector);
    // let text: String = r#"<a color.red title.true>hello world<\a>"#.to_string();
    // println!("{}", text);

    println!("{}, {}!", "Hello".red(), "World".green().bold());
}
