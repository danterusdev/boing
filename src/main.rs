use std::{error::Error, io::{stdout, Write, Read}, process::exit, fs::File};

use crossterm::{event::{read, Event, KeyCode, KeyEvent}, terminal::{enable_raw_mode, Clear, ClearType}, execute, style::Print, ExecutableCommand, cursor::MoveTo};

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;

    let mut edit_state = EditState {
        buffer: String::new(),
        row: 0,
        column: 0,
    };
    
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    let mut state = State::Editing;

    loop {
        let event = read()?;
        update(Some(event), &mut state, &mut edit_state)?;
    }
}

fn update(event: Option<Event>, state: &mut State, edit_state: &mut EditState) -> Result<(), Box<dyn Error>> {
    
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    match state {
        State::Editing => {
            let lines: Vec<&str> = edit_state.buffer.split("\n").collect();
            if let Some(Event::Key(KeyEvent {code, ..})) = event {
                if let KeyCode::Char(character) = code {
                    let mut total_length = 0;
                    for index in 0..edit_state.row {
                        total_length += lines[index as usize].len() + 1;
                    }
                    total_length += edit_state.column as usize;

                    let before = &edit_state.buffer[0..total_length];
                    let after = &edit_state.buffer[total_length..edit_state.buffer.len()];
                    edit_state.buffer = format!("{}{}{}", before, character, after);
                    edit_state.column += 1;
                } else if code == KeyCode::Backspace {
                    if edit_state.buffer.len() > 0 {
                        let mut total_length = 0;
                        for index in 0..edit_state.row {
                            total_length += lines[index as usize].len() + 1;
                        }
                        total_length += edit_state.column as usize;

                        if edit_state.column != 0 {
                            edit_state.column -= 1;
                        } else {
                            edit_state.row -= 1;
                            edit_state.column = lines[edit_state.row as usize].len() as u16;
                        }

                        let before = &edit_state.buffer[0..total_length - 1];
                        let after = &edit_state.buffer[total_length..edit_state.buffer.len()];
                        edit_state.buffer = format!("{}{}", before, after);
                    }
                } else if code == KeyCode::Enter {
                    edit_state.buffer += &String::from("\n");
                    edit_state.row += 1;
                    edit_state.column = 0;
                } else if code == KeyCode::Right {
                    if edit_state.column != lines[edit_state.row as usize].len() as u16 {
                        edit_state.column += 1;
                    }
                } else if code == KeyCode::Left {
                    if edit_state.column != 0 {
                        edit_state.column -= 1;
                    }
                } else if code == KeyCode::Up {
                    if edit_state.row > 0 {
                        edit_state.row -= 1;

                        if lines[edit_state.row as usize].len() < edit_state.column as usize {
                            edit_state.column = lines[edit_state.row as usize].len() as u16;
                        }
                    }
                } else if code == KeyCode::Down {
                    if edit_state.row < lines.len() as u16 - 1 {
                        edit_state.row += 1;

                        if lines[edit_state.row as usize].len() < edit_state.column as usize {
                            edit_state.column = lines[edit_state.row as usize].len() as u16;
                        }
                    }
                } else if code == KeyCode::Tab {
                    *state = State::Saving("".into());
                    return update(None, state, edit_state);
                } else if code == KeyCode::Insert {
                    *state = State::Loading("".into());
                    return update(None, state, edit_state);
                } else if code == KeyCode::Esc {
                    exit(0);
                }
            }
        
            let mut current_line = 0;
            for character in edit_state.buffer.chars() {
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

            stdout().execute(MoveTo(edit_state.column, edit_state.row))?;
        },
        State::Saving(path) => {
            if let Some(Event::Key(KeyEvent {code, ..})) = event {
                if let KeyCode::Char(character) = code {
                    *path += &String::from(character);
                } else if code == KeyCode::Enter {
                    let mut file = File::create(&path)?;
                    file.write_all(edit_state.buffer.as_bytes())?;
                    *state = State::Editing;
                    return update(None, state, edit_state);
                } else if code == KeyCode::Esc {
                    *state = State::Editing;
                    return update(None, state, edit_state);
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
                    edit_state.buffer.clear();
                    file.read_to_string(&mut edit_state.buffer)?;
                    *state = State::Editing;
                    return update(None, state, edit_state);
                } else if code == KeyCode::Esc {
                    *state = State::Editing;
                    return update(None, state, edit_state);
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

struct EditState {
    buffer: String,
    row: u16,
    column: u16,
}
