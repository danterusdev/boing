use std::{error::Error, io::{stdout, Write, Read}, process::exit, fs::File};

use crossterm::{event::{read, Event, KeyCode, KeyEvent}, terminal::{enable_raw_mode, Clear, ClearType}, execute, style::Print, ExecutableCommand, cursor::MoveTo};

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;

    let mut buffer = String::new();
    
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    let mut state = State::Editing;

    loop {
        let event = read()?;
        update(Some(event), &mut state, &mut buffer)?;
    }
}

fn update(event: Option<Event>, state: &mut State, buffer: &mut String) -> Result<(), Box<dyn Error>> {
    
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    match state {
        State::Editing => {
            if let Some(Event::Key(KeyEvent {code, ..})) = event {
                if let KeyCode::Char(character) = code {
                    *buffer += &String::from(character);
                } else if code == KeyCode::Backspace {
                    if buffer.len() > 0 {
                        *buffer = (&buffer[0..buffer.len() - 1]).to_string();
                    }
                } else if code == KeyCode::Enter {
                    *buffer += &String::from("\n");
                } else if code == KeyCode::Tab {
                    *state = State::Saving("".into());
                    return update(None, state, buffer);
                } else if code == KeyCode::Insert {
                    *state = State::Loading("".into());
                    return update(None, state, buffer);
                } else if code == KeyCode::Esc {
                    exit(0);
                }
            }
        
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
        },
        State::Saving(path) => {
            if let Some(Event::Key(KeyEvent {code, ..})) = event {
                if let KeyCode::Char(character) = code {
                    *path += &String::from(character);
                } else if code == KeyCode::Enter {
                    let mut file = File::create(&path)?;
                    file.write_all(buffer.as_bytes())?;
                    *state = State::Editing;
                    return update(None, state, buffer);
                } else if code == KeyCode::Esc {
                    *state = State::Editing;
                    return update(None, state, buffer);
                }
            }

            stdout().execute(Print("Save File: "))?;

            for character in path.chars() {
                match character {
                    _ => {
                        stdout().execute(Print(character))?;
                    }
                };
            }
        },
        State::Loading(path) => {
            if let Some(Event::Key(KeyEvent {code, ..})) = event {
                if let KeyCode::Char(character) = code {
                    *path += &String::from(character);
                } else if code == KeyCode::Enter {
                    let mut file = File::open(&path)?;
                    buffer.clear();
                    file.read_to_string(buffer)?;
                    *state = State::Editing;
                    return update(None, state, buffer);
                } else if code == KeyCode::Esc {
                    *state = State::Editing;
                    return update(None, state, buffer);
                }
            }

            stdout().execute(Print("Load File: "))?;

            for character in path.chars() {
                match character {
                    _ => {
                        stdout().execute(Print(character))?;
                    }
                };
            }
        }
    }

    Ok(())
}

enum State {
    Editing,
    Saving(String),
    Loading(String),
}
