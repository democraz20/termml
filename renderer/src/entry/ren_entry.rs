use crossterm::{
    event::Event,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
use web_parser::static_data::structs::{Div, StyleChild, TermmlMain};

use std::{collections::HashMap, io::stdout, time::Duration};
pub struct MainNavigator;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

impl MainNavigator {
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
