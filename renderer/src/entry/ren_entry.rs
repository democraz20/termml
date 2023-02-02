use crossterm::{
    cursor::MoveTo,
    event::Event,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use debug::logger::Logger;
use hard_xml::{XmlRead, XmlWrite};

use web_parser::{
    process_string::bond,
    static_data::structs::{Div, ReqPair, StyleChild, StyleMain, TermmlMain},
};

use std::{collections::HashMap, io::stdout, time::Duration};

use crate::{debug::ren_debug, request::webrequest::fetch};
pub struct MainNavigator;

pub mod split_chunk;
use split_chunk::CharChunksTrait;
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

impl MainNavigator {
    pub fn getter(&self, server_url: String, dbg: bool) {
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
                        TermmlMain::fetch_error(
                            url.as_str(),
                            Some(response.status_text()),
                            Some(code),
                        )
                        .to_string()
                        .unwrap()
                    }
                    ureq::Error::Transport(transport) => {
                        //Termml to_string goes here
                        transport.to_string();
                        TermmlMain::fetch_error(
                            url.as_str(),
                            Some(transport.kind().to_string()),
                            None,
                        )
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
        // let resizedml = Self::resize_markup(parsedml, width)

        //just for ease of debug
        match dbg {
            false => {
                let _ = Self::entry(&self, parsedml, hash);
            }
            true => {
                let d = crate::debug::ren_debug::DebugRenderer;
                d.temp(parsedml, hash)
            }
        }
    }
    pub fn entry(
        &self,
        mut termml: TermmlMain,
        stylemap: HashMap<String, StyleChild>,
    ) -> Result<()> {
        let _cleanup = CleanUp;
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        execute!(stdout(), MoveTo(0, 0))?;
        let mut line_index: u32 = 0; //shouldnt go below 0
        let (mut column, mut rows) = crossterm::terminal::size().unwrap();
        let mut logger = Logger::new("bufferlog", "buffer.log", true);
        //init
        loop {
            let (c, r) = crossterm::terminal::size().unwrap();
            if c != column || r != rows {
                //terminal resized
                println!("terminal resized c:{},r:{}", c, r);
                column = c;
                rows = r;

                //to fix this later, this would not allow it to be resized back to original
                let head = termml.head.value.clone();
                termml.head.value = Self::resize_markup(vec![head], column)[0].clone();
                let divs = termml.body.value.clone();
                termml.body.value = Self::resize_markup(divs, column);

                let mut testlog = Logger::new("test", "test.log", true);
                println!("===\n\n{:?}", termml.body.value.clone());
                for i in termml.body.value.clone() {
                    testlog.add(&format!("{}", i.value));
                }
                testlog.save()?;

                let mut buf: Vec<Div> = vec![]; //because the terminal resized
                for i in 0..r {
                    if (i as usize) < termml.body.value.len() {
                        buf.push(termml.body.value[i as usize].clone());
                        logger.add(&format!("{}", termml.body.value[i as usize].value.clone()));
                        // println!("pushed: {}", termml.body.value[i as usize].value.clone());
                    }
                    //for how many rows there are on the screen
                    //making sure the indexes dont go beyond buffer len
                    //including when it is iterating

                    // println!("{:?},{:?}",
                    // line_index < buf.len() as u32,
                    // (line_index+1) < buf.len() as u32);
                    // println!("index: {}, buflen: {}", line_index, buf.len());
                    // println!("c    : {}, r     : {}", c, r);
                    // if line_index < buf.len() as u32
                    // {
                    //     buf.push(termml.body.value[i as usize].clone());
                    //     logger.add(&format!("{}", termml.body.value[i as usize].value.clone()));
                    //     println!("pushed: {}", termml.body.value[i as usize].value.clone());
                    // }
                }
                logger.save()?;
                println!("saved log");
                if line_index > buf.len() as u32 {
                    line_index = buf.len() as u32
                }
            }
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
                        KeyEvent {
                            code: KeyCode::Up, ..
                        } => {
                            if line_index >= 1 {
                                line_index -= 1;
                                println!("line_index: {}", line_index);
                            } else if line_index == 0 {
                                println!("line_index: {}", line_index);
                            }
                            //navigation code, call re write buffer
                        }
                        KeyEvent {
                            code: KeyCode::Down,
                            ..
                        } => {
                            line_index += 1;
                            println!("line_index: {}", line_index);
                            //navigation code, call re write buffer
                        }
                        //idle
                        _ => {}
                    }
                }
            }
        }
        println!("Running cleanup code");
        Ok(())
    }
    pub fn resize_markup(original: Vec<Div>, width: u16) -> Vec<Div> {
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
        return text.char_chunks(len).map(String::from).collect::<Vec<_>>();
    }
    fn cleanup() -> Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
