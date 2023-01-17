use web_parser::{
    process_string::bond,
    static_data::structs::{ReqPair, StyleChild, StyleMain, TermmlMain},
    // webrequest::request::{fetch, get_filename},
};

use renderer::request::webrequest::fetch;

use hard_xml::{XmlRead, XmlWrite};
use std::collections::HashMap;

//tracking memory usage
fn main() {
    start();
}

fn start() {
    //caching
    let mut files: HashMap<String, String> = HashMap::new();
    let server_url = String::from("http://127.0.0.1:5500/");
    let url = format!("{}{}", server_url, "test.termml");
    // let _filename = get_filename(&url);
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
    let mut parsedml = match res {
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
                        header: None,
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

    let binding = parsedml.body.value.clone();
    parsedml.body.value = renderer::entry::ren_entry::MainNavigator::resize_markup(&binding, 10);
    // dbg!(&resized);
    // let termml_vec = construct_termml_vec(parsedml.clone(), hash.clone());
    // dbg!(termml_vec);
    let renderer = renderer::debug::ren_debug::DebugRenderer;

    renderer.debug(parsedml, hash);
    // renderer.entry(vec);
    // dbg!(styles_hash());
}