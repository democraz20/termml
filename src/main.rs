use ansi_term::Style;
use crossterm::style;
use serde::__private::de::FlatInternallyTaggedAccess;
use web_parser::{
    process_string::bond,
    process_string::bond::{markup_entry, parse_style_sheet, styles_hash, remove_tabs},
    static_data::structs::{ReqPair, StyleChild, StyleMain, TermmlMain},
    webrequest::request::{fetch, get_filename},
};

use renderer::{debug::ren_debug::DebugRenderer, entry::ren_entry::MainNavigator};

use hard_xml::{XmlRead, XmlWrite};
use std::{alloc, collections::HashMap, process::Child};
use ureq::{Response, Transport};

//tracking memory usage
use cap::Cap;
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
fn main() {
    start();
}

fn start() {
    //caching
    let mut files: HashMap<String, String> = HashMap::new();
    let server_url = String::from("http://127.0.0.1:5500/");
    let url = format!("{}{}", server_url, "test.termml");
    let filename = get_filename(&url);
    let mut termss_vec: Vec<String> = vec![];
    let fetched = match fetch(&url) {
        Ok(r) => r,
        Err(e) => {
            match e {
                ureq::Error::Status(code, response) => {
                    //Termml to_string goes here
                    TermmlMain::fetch_error(url.as_str(), Some(response.status_text()), Some(code))
                        .to_string()
                        .unwrap()
                }
                ureq::Error::Transport(transport) => {
                    //Termml to_string goes here
                    transport.to_string();
                    TermmlMain::fetch_error(url.as_str(), Some(transport.kind().to_string()), None)
                        .to_string()
                        .unwrap()
                }
            }
        }
    };
    let binding = fetched.clone();
    let res = TermmlMain::from_str(binding.as_str());
    let binding = url.clone();
    let parsedml = match res {
        Ok(r) => r,
        Err(e) => TermmlMain::parse_error(binding.as_str(), e),
    };

    //cache main toml file
    files.insert(url.clone(), fetched);
    let mut read_style: Vec<ReqPair> = vec![];
    for i in parsedml.require.clone() {
        // dbg!(i.stylesheet);
        let stlyesheet = i.stylesheet;
        for styleiter in stlyesheet {
            println!("Required TERMSS : {}", styleiter.name);
            let req_url = format!("{}{}", server_url, styleiter.name);
            let fetched = match fetch(&req_url) {
                Ok(r) => r,
                Err(_) => toml::to_string(&StyleMain {
                    styles: vec![StyleChild {
                        class: String::from("null"),
                        background: None,
                        foreground: None,
                        underline: None,
                        bold: None,
                        header: None
                    }],
                })
                .unwrap(),
            };
            read_style.push(ReqPair {
                name: styleiter.name.to_string(),
                value: fetched.clone(),
            });
            //cache termss files
            if req_url.ends_with("termss") {
                termss_vec.push(req_url.clone());
            }
            files.insert(req_url, fetched);
        }
        // println!("{}", i);
    }
    let hash = bond::styles_hash(read_style);
    dbg!(&hash);

    // let debug_renderer = DebugRenderer;
    // debug_renderer.debug(parsedml, hash);
    let renderer = MainNavigator;
    let termml_vec = construct_termml_vec(parsedml, hash);
    dbg!(termml_vec);
    // renderer.entry(vec);
    _alloced("End of main");
    // dbg!(styles_hash());
}

//TEMP
fn construct_termml_vec(
    markup: TermmlMain, stylesmap: HashMap<String, StyleChild>
    ) -> Vec<(String, StyleChild)>{
    let mut vec: Vec<(String, StyleChild)> = vec![];
    let head_divs = markup.head.value;
    let body_divs = markup.body.value;
    match head_divs.class {
        Some(class) => {
            let k: String = class.into();
            let c = stylesmap.get(&k);
            let style = c.cloned();
            drop(c);
            match style {
                Some(child) => {
                    println!("pushing vec");
                    vec.push((head_divs.value.into(), child));
                },
                None => {
                    println!("no pushing vec");
                    vec.push((head_divs.value.into(), 
                        StyleChild {
                            class: String::from("null"),
                            background: None,
                            foreground: None,
                            underline: None,
                            bold: None,
                            header: Some(true)
                        }
                    ));
                }
            }
        },
        None => {
            vec.push((head_divs.value.into(),
                StyleChild {
                    class: String::from("null"),
                    background: None,
                    foreground: None,
                    underline: None,
                    bold: None,
                    header: Some(true)
                }
            ))
        }
    }
    for mut i in body_divs {
        println!("in loop");
        i.value = remove_tabs(i.value.clone().to_string()).into();
        match i.class {
            Some(class) => {
                let k: String = class.into();
                let c = stylesmap.get(&k);
                let style = c.cloned();
                drop(c);
                match style {
                    Some(child) => {
                        vec.push((i.value.into(), child));
                    },
                    None => {
                        vec.push((i.value.into(), 
                            StyleChild {
                                class: String::from("null"),
                                background: None,
                                foreground: None,
                                underline: None,
                                bold: None,
                                header: Some(false)
                            }
                        ));
                    }
                }
            },
            None => {
                vec.push((i.value.into(),
                StyleChild {
                    class: String::from("null"),
                    background: None,
                    foreground: None,
                    underline: None,
                    bold: None,
                    header: Some(false)
                }
            ))
            }
        }
    }
    return vec;
}

fn _alloced<T: std::fmt::Display>(header: T) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());
}
