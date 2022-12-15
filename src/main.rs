mod process_string;
mod static_data;
mod webrequest;
mod renderer;

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
use crate::webrequest::request::{fetch, get_filename};
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
                    TermmlMain::fetch_error(
                        url.as_str(), Some(response.status_text()), Some(code)
                    )
                    .to_string().unwrap()
                },
                ureq::Error::Transport(transport) => {
                    //Termml to_string goes here
                    transport.to_string();
                    TermmlMain::fetch_error(
                        url.as_str(), Some(transport.kind().to_string()), None
                    )
                    .to_string().unwrap()
                }
            }
        }
    };
    let binding = fetched.clone();
    let res = TermmlMain::from_str(binding.as_str());
    let binding = url.clone();
    let parsed = match res {
        Ok(r) => r,
        Err(e) => TermmlMain::parse_error(binding.as_str(), e)
    };

    //cache main toml file
    files.insert(
        url.clone(),
        fetched
    );
    for i in parsed.require {
        // dbg!(i.stylesheet);
        let stlyesheet = i.stylesheet;
        for styleiter in stlyesheet {
            println!("Required TERMSS : {}", styleiter.name);
            let req_url = format!("{}{}", server_url, styleiter.name);
            let fetched = match fetch(&req_url) {
                Ok(r) => r,
                Err(_) => {
                    toml::to_string(&StyleMain {
                        styles: vec![StyleChild {
                            class: String::from("null"),
                            background: None,
                            foreground: None,
                            underline: None,
                            bold: None,
                            wrap: None,
                        }],
                    }).unwrap()
                }
            };

            //cache termss files
            if req_url.ends_with("termss") {
                termss_vec.push(req_url.clone());
            }
            files.insert(
                req_url,
                fetched
            );

        }
        // println!("{}", i);
    }
    _alloced("End of main");
    // dbg!(styles_hash());
}

fn _alloced<T: std::fmt::Display>(header: T) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());
}
