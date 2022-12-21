use ansi_term::Style;
use crossterm::{event::{Event}, event::{self, KeyEvent, KeyCode}, execute, Result, terminal::{EnterAlternateScreen, self, LeaveAlternateScreen}};

use std::{io::stdout, time::Duration};
pub struct MainNavigator;

impl MainNavigator {
    pub fn entry(&self, vec: Vec<(String, Style)>) -> Result<()>{
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        loop {
            Self::event_poll()?;
        }
        // for i in vec {
        //     //temp
        //     println!("{}", i.0);
        // }
        Ok(())
    }
    fn event_poll() -> Result<()> {
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(event) = event::read()? {
                Self::match_key_event(event)?;
            }
        }
        Ok(())
    }
    fn match_key_event(event: KeyEvent) -> Result<()> {
        match event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL, ..
            } => {
                println!("CTRL-C pressed, doing cleanup");
                execute!(stdout(), LeaveAlternateScreen)?;
                terminal::disable_raw_mode()?;
            }
            _ => {}
        }
        Ok(())
    }
}