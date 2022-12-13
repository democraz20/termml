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
    dbg!(&filename);
    let mut termss_vec: Vec<String> = vec![];
    dbg!(&server_url);
    dbg!(&url);
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
    let binding = f.clone();
    let p = TermmlMain::from_str(binding.as_str());
    let binding = url.clone();
    let parsed = match p {
        Ok(r) => r,
        Err(e) => TermmlMain::parse_error(binding.as_str(), e)
    };
    dbg!(&parsed);

    //cache main toml file
    files.insert(
        url.clone(),
        f
    );
    for i in parsed.require {
        // dbg!(i.stylesheet);
        let s = i.stylesheet;
        for i in s {
            println!("Required TERMSS : {}", i.name);
            let req_url = format!("{}{}", server_url, i.name);
            let t = match fetch(&req_url) {
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
            dbg!(&t);

            //cache termss files
            if req_url.ends_with("termss") {
                termss_vec.push(req_url.clone());
            }
            files.insert(
                req_url,
                t
            );

        }
        // println!("{}", i);
    }
    dbg!(&files);
    dbg!(&termss_vec);
    _alloced("End of main");
    // dbg!(styles_hash());
}

fn _alloced<T: std::fmt::Display>(header: T) -> () {
    //only for development : not suppossed to be in actual build AT ALL
    //in the future might add a choice to see if its a release or debug
    //to decide the printing stdout
    println!("{} | Allocated : {} B(ytes)", header, ALLOCATOR.allocated());
}
