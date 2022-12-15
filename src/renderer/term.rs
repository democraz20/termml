use std::collections::HashMap;

use crate::static_data::{structs::{TermmlMain, StyleChild}, term_style::get_color_from_string};

pub fn debug(markup: TermmlMain, 
    stylesmap: HashMap<String, StyleChild>) {
    let divs = markup.body.value;
    for i in divs {
        match i.class {
            Some(class) => {
                let k: String = class.into();
                let c = stylesmap.get(&k);
                let style = c.cloned();
                drop(c);
                match style {
                    Some(style) => {
                        print_style(i.value.to_string(), style)
                    },
                    None => {
                        print_plain(i.value.to_string())
                    }
                }
            },
            None => {
                // println!("{}", i.value)
                print_plain(i.value.to_string())
            }
        }
    }
}

fn print_plain(text: String) {
    println!("{}", text)
}

fn print_style(text: String, style: StyleChild) {
    let mut s = ansi_term::Style::new();
    match style.background {
        Some(b) => { s = s.on(
            get_color_from_string(b)
        )},
        _ => {}
    }
    match style.foreground {
        Some(b) => { s = s.fg(
            get_color_from_string(b)
        )},
        _ => {}
    }
    match style.underline {
        Some(b) => {
            if b { s = s.underline() }
        }
        _ => {}
    }
    match style.bold {
        Some(b) => {
            if b { s = s.bold() }
        }
        _ => {}
    }
}