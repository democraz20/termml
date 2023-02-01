use std::collections::HashMap;

use crate::entry::ren_entry::MainNavigator;

use web_parser::process_string::bond::remove_tabs;
use web_parser::static_data::{
    structs::{StyleChild, TermmlMain},
    term_style::get_color_from_string,
};

pub struct DebugRenderer;

impl DebugRenderer {
    pub fn debug(&self, markup: TermmlMain, stylesmap: HashMap<String, StyleChild>) {
        println!("=====[start debug renderer]=====");
        let body_divs = markup.body.value;
        let head_divs = markup.head.value;
        println!("[INFO] Head tag");
        match head_divs.class {
            Some(class) => {
                let k: String = class.into();
                let c = stylesmap.get(&k);
                let style = c.cloned();
                drop(c);
                match style {
                    Some(style) => Self::print_style(head_divs.value.to_string(), style),
                    None => Self::print_plain(head_divs.value),
                }
            }
            None => Self::print_plain(head_divs.value),
        }
        println!("[INFO] body tag");
        for mut i in body_divs {
            i.value = remove_tabs(i.value.clone().to_string()).into();
            match i.class {
                Some(class) => {
                    let k: String = class.into();
                    let c = stylesmap.get(&k);
                    let style = c.cloned();
                    drop(c);
                    match style {
                        Some(style) => Self::print_style(i.value.to_string(), style),
                        None => Self::print_plain(i.value),
                    }
                }
                None => Self::print_plain(i.value),
            }
        }
        println!("======[end debug renderer]======");
    }
    pub fn temp(&self, markup: TermmlMain, stylesmap: HashMap<String, StyleChild>) {
        println!("=====[start debug renderer]=====");
        let body_divs = markup.body.value;
        let head_div  = markup.head.value;
        let (mut column, mut rows) = crossterm::terminal::size().unwrap();
        let mut count = 0;
        loop {
            let (c, r) = crossterm::terminal::size().unwrap();
            if c != column || r != rows { //term size changed
                count += 1;
                column = c;
                rows = r;
                println!("===[cycle : {count}]===");
                let body = MainNavigator::resize_markup(body_divs.clone(), c);
                let head = MainNavigator::resize_markup(vec![head_div.clone()], c)[0].clone();
                match head.class {
                    Some(c) => {
                        let k: String = c.into();
                        let cl = stylesmap.get(&k);
                        let style = cl.cloned();
                        drop(cl);
                        match style {
                            Some(style) => Self::print_style(head.value.to_string(), style),
                            None => Self::print_plain(head.value)
                        }
                    }
                    None => Self::print_plain(head.value)
                }
                for i in body {
                    match i.class {
                        Some(c) => {
                            let k: String = c.into();
                            let cl = stylesmap.get(&k);
                            let style = cl.cloned();
                            drop(cl);
                            match style {
                                Some(style) => Self::print_style(i.value.to_string(), style),
                                None => Self::print_plain(i.value)
                            }
                        }
                        None => Self::print_plain(i.value)
                    }
                }
            }
        }
    }

    fn print_plain<T: std::fmt::Display>(text: T) {
        println!("{}", text)
    }

    fn print_style(text: String, style: StyleChild) {
        let mut s = ansi_term::Style::new();
        match style.background {
            Some(b) => s = s.on(get_color_from_string(b)),
            _ => {}
        }
        match style.foreground {
            Some(b) => s = s.fg(get_color_from_string(b)),
            _ => {}
        }
        match style.underline {
            Some(b) => {
                if b {
                    s = s.underline()
                }
            }
            _ => {}
        }
        match style.bold {
            Some(b) => {
                if b {
                    s = s.bold()
                }
            }
            _ => {}
        }
        println!("{}", s.paint(text));
    }
}
