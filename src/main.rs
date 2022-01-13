use std::{error::Error, io::stdout, process::exit};

use crossterm::{event::{read, Event, KeyCode, KeyEvent}, terminal::{enable_raw_mode, Clear, ClearType}, execute, style::Print, ExecutableCommand, cursor::MoveTo};

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;

    let mut buffer = String::new();
    
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    loop {
        let event = read()?;
        if event == Event::Key(KeyCode::Esc.into()) {
            exit(0);
        }

        if let Event::Key(KeyEvent {code, ..}) = event {
            if let KeyCode::Char(character) = code {
                buffer += &String::from(character);
            } else if code == KeyCode::Backspace {
                if buffer.len() > 0 {
                    buffer = (&buffer[0..buffer.len() - 1]).to_string();
                }
            } else if code == KeyCode::Enter {
                buffer += &String::from("\n");
            }
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

        let mut current_line = 0;
        for character in buffer.chars() {
            match character {
                '\n' => {
                    current_line += 1;
                    execute!(stdout(), MoveTo(0, current_line))?;
                }
                _ => {
                    stdout().execute(Print(character))?;
                }
            };
        }
    }
}
