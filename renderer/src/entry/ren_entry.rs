use ansi_term::Style;
use crossterm::{event::Event, event::{self, KeyEvent, KeyCode}, execute, Result, terminal::{EnterAlternateScreen, self, LeaveAlternateScreen}};
use web_parser::static_data::structs::{TermmlMain, StyleChild};

use std::{io::{stdout, Read}, time::Duration, collections::HashMap};
pub struct MainNavigator;

struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

impl MainNavigator {
    pub fn entry(&self, termml: TermmlMain, stylemap: HashMap<String, StyleChild>) -> Result<()>{
        let _cleanup = CleanUp;
        // Self::resize_markup(vec, 10 as u16);
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        loop {
            if event::poll(Duration::from_millis(1000))? {
                if let Event::Key(event) = event::read()? {
                    match event {
                        KeyEvent {
                            code: KeyCode::Char('c'),
                            modifiers: event::KeyModifiers::CONTROL, ..
                        } => {
                            println!("CTRL-C pressed, doing cleanup");
                            Self::cleanup()?;
                            break;
                        },
                        KeyEvent {
                            code: KeyCode::Esc,
                            modifiers: event::KeyModifiers::NONE, ..
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
    fn resize_markup<'a>(original: &'a mut TermmlMain, width: u16) -> TermmlMain<'a>{
        let divs = original.body.value.clone();
        for i in divs {
            let text = i.value;
            if text.len() > width.into() {
                let s = text.chars()
                    .enumerate()
                    .flat_map(|(i, c)| {
                        if i != 0 && i % width as usize == 0 {
                            Some('␡')
                        } else {
                            None
                        }
                        .into_iter()
                        .chain(std::iter::once(c))
                    })
                    .collect::<String>();
                let c = s.split("␡").map(String::from)
                    .collect::<Vec<String>>();
            }
        }
        original.clone()
    }
    fn cleanup() -> Result<()>{
        execute!(stdout(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}