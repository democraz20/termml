mod process_string;
mod static_data;
mod webrequest;

use hard_xml::{XmlWrite, XmlRead};
use ureq::{Response, Transport};
use std::sync::mpsc::TryRecvError;
use std::{alloc, collections::HashMap, fs};

//tracking memory usage
use cap::Cap;
#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());
use crate::process_string::bond::{markup_entry, parse_style_sheet, styles_hash};
use crate::static_data::structs::{
    TermmlMain,
    StyleMain,
    StyleChild
};
use crate::webrequest::request::fetch;
fn main() {
    start();
}

fn start() {
    let mut stylesmap: HashMap<String, String> = HashMap::new();
    let url = String::from("http://127.0.0.1:5500/test.termml");
    
    let f = match fetch(&url) {
        Ok(r) => {
            println!("successful");
            r
        },
        Err(e) => {
            match e {
                ureq::Error::Status(code, response) => {
                    //Termml to_string goes here
                    eprintln!("status error code:{code}");
                    TermmlMain::fetch_error(
                        url.as_str(), Some(response.status_text()), Some(code)
                    )
                    .to_string().unwrap()
                },
                ureq::Error::Transport(transport) => {
                    //Termml to_string goes here
                    eprintln!("transport error");
                    transport.to_string();
                    TermmlMain::fetch_error(
                        url.as_str(), Some(transport.kind().to_string()), None
                    )
                    .to_string().unwrap()
                }
            }
        }
    };
    let p = TermmlMain::from_str(f.as_str());
    let parsed = match p {
        Ok(r) => r,
        Err(e) => TermmlMain::parse_error(url.as_str(), e)
    };
    dbg!(&parsed);
    for i in parsed.require {
        // dbg!(i.stylesheet);
        let s = i.stylesheet;
        for i in s {
            println!("Required TERMSS : {}", i.name);
            let t = match fetch(&i.name.into()) {
                Ok(r) => r,
                Err(e) => {
                    toml::to_string(&StyleMain {
                        styles: vec![StyleChild {
                            class: String::from("null"),
                            background: None,
                            foreground: None,
                            underline: None,
                            bold: None,
                            wrap: None,
                            margin: None,
                        }],
                    }).unwrap()
                }
            };
        }
        // println!("{}", i);
    }
    // dbg!(styles_hash());
}

fn _alloced(header: Option<&str>) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    match header {
        Some(h) => {
            println!("{} | Allocated : {} B(ytes)", h, ALLOCATOR.allocated());
        }
        None => {
            println!("  | Allocated : {} B(ytes)", ALLOCATOR.allocated());
        }
    }
}
