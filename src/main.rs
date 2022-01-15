mod application;
mod editor;

use std::{error::Error, io::stdout};

use application::Application;
use crossterm::{event::read, terminal::{enable_raw_mode, Clear, ClearType}, execute, cursor::MoveTo};
use editor::Editor;

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;

    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    let mut current_application: Box<dyn Application> = Box::new(Editor::new());

    loop {
        let event = read()?;
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        current_application.update(Some(event))?;
    }
}
