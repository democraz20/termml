use crossterm::{
    event::Event,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use hard_xml::{XmlRead, XmlWrite};

use web_parser::{static_data::structs::{Div, StyleChild, TermmlMain, ReqPair, StyleMain}, process_string::bond};

use std::{collections::HashMap, io::stdout, time::Duration};

use crate::request::webrequest::fetch;
pub struct MainNavigator;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

impl MainNavigator {
    pub fn getter(&self, server_url: String) {
        let mut files: HashMap<String, String> = HashMap::new();
        // let server_url = String::from("http://127.0.0.1:5500/");
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
        Self::entry(&self, parsedml, hash);
    }
    pub fn entry(&self, termml: TermmlMain, stylemap: HashMap<String, StyleChild>) -> Result<()> {
        let _cleanup = CleanUp;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        loop {
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(event) = event::read()? {
                    match event {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: event::KeyModifiers::CONTROL,
                            ..
                        } => {
                            println!("CTRL-C pressed, doing cleanup");
                            Self::cleanup()?;
                            break;
                        }
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            println!("ESC pressed, doing cleanup");
                            Self::cleanup()?;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        println!("Running cleanup code");
        Ok(())
    }
    pub fn resize_markup<'a>(original: &'a Vec<Div>, width: u16) -> Vec<Div<'a>> {
        let mut new_vec: Vec<Div> = vec![];
        for (_, d) in original.clone().iter_mut().enumerate() {
            let text = d.clone().value;
            if text.len() > width.into() {
                println!("text is longer");
                let splitted = Self::split_by_len(text.to_string(), width.into());
                for i in splitted {
                    new_vec.push(Div {
                        class: d.class.clone(),
                        value: i.into(),
                    });
                }
            } else {
                new_vec.push(d.clone());
            }
        }
        return new_vec;
    }
    fn split_by_len(text: String, len: usize) -> Vec<String> {
        let s = text
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % len as usize == 0 {
                    Some('␡')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .collect::<String>();
        //THIS IS FUCKING GARBAGE
        //SEND HELP
        let c = s.split("␡").map(String::from).collect::<Vec<String>>();
        return c;
    }
    fn cleanup() -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
