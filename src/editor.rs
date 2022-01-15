use std::{error::Error, process::exit, io::{stdout, Write, Read}, fs::File};

use crossterm::{event::{Event, KeyEvent, KeyCode}, execute, cursor::MoveTo, style::Print, ExecutableCommand};

use crate::application::Application;

pub struct Editor {
    state: State,
    edit_state: EditState
}

impl Editor {
    pub fn new() -> Self {
        Self {
            state: State::Editing,
            edit_state: EditState {
                buffer: String::new(), row: 0, column: 0
            }
        }
    }
}

impl Application for Editor {
    fn update(&mut self, event: Option<Event>) -> Result<(), Box<dyn Error>> {
        match self.state.clone() {
            State::Editing => {
                let lines: Vec<&str> = self.edit_state.buffer.split("\n").collect();
                if let Some(Event::Key(KeyEvent {code, ..})) = event {
                    if let KeyCode::Char(character) = code {
                        let mut total_length = 0;
                        for index in 0..self.edit_state.row {
                            total_length += lines[index as usize].len() + 1;
                        }
                        total_length += self.edit_state.column as usize;
    
                        let before = &self.edit_state.buffer[0..total_length];
                        let after = &self.edit_state.buffer[total_length..self.edit_state.buffer.len()];
                        self.edit_state.buffer = format!("{}{}{}", before, character, after);
                        self.edit_state.column += 1;
                    } else if code == KeyCode::Backspace {
                        if self.edit_state.buffer.len() > 0 {
                            let mut total_length = 0;
                            for index in 0..self.edit_state.row {
                                total_length += lines[index as usize].len() + 1;
                            }
                            total_length += self.edit_state.column as usize;
    
                            if self.edit_state.column != 0 {
                                self.edit_state.column -= 1;
                            } else {
                                self.edit_state.row -= 1;
                                self.edit_state.column = lines[self.edit_state.row as usize].len() as u16;
                            }
    
                            let before = &self.edit_state.buffer[0..total_length - 1];
                            let after = &self.edit_state.buffer[total_length..self.edit_state.buffer.len()];
                            self.edit_state.buffer = format!("{}{}", before, after);
                        }
                    } else if code == KeyCode::Enter {
                        self.edit_state.buffer += &String::from("\n");
                        self.edit_state.row += 1;
                        self.edit_state.column = 0;
                    } else if code == KeyCode::Right {
                        if self.edit_state.column != lines[self.edit_state.row as usize].len() as u16 {
                            self.edit_state.column += 1;
                        }
                    } else if code == KeyCode::Left {
                        if self.edit_state.column != 0 {
                            self.edit_state.column -= 1;
                        }
                    } else if code == KeyCode::Up {
                        if self.edit_state.row > 0 {
                            self.edit_state.row -= 1;
    
                            if lines[self.edit_state.row as usize].len() < self.edit_state.column as usize {
                                self.edit_state.column = lines[self.edit_state.row as usize].len() as u16;
                            }
                        }
                    } else if code == KeyCode::Down {
                        if self.edit_state.row < lines.len() as u16 - 1 {
                            self.edit_state.row += 1;
    
                            if lines[self.edit_state.row as usize].len() < self.edit_state.column as usize {
                                self.edit_state.column = lines[self.edit_state.row as usize].len() as u16;
                            }
                        }
                    } else if code == KeyCode::Tab {
                        self.state = State::Saving("".into());
                        return self.update(None);
                    } else if code == KeyCode::Insert {
                        self.state = State::Loading("".into());
                        return self.update(None);
                    } else if code == KeyCode::Esc {
                        exit(0);
                    }
                }
            
                let mut current_line = 0;
                for character in self.edit_state.buffer.chars() {
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
    
                stdout().execute(MoveTo(self.edit_state.column, self.edit_state.row))?;
            },
            State::Saving(mut path) => {
                if let Some(Event::Key(KeyEvent {code, ..})) = event {
                    if let KeyCode::Char(character) = code {
                        path += &String::from(character);
                    } else if code == KeyCode::Enter {
                        let mut file = File::create(&path)?;
                        file.write_all(self.edit_state.buffer.as_bytes())?;
                        self.state = State::Editing;
                        return self.update(None);
                    } else if code == KeyCode::Esc {
                        self.state = State::Editing;
                        return self.update(None);
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
            State::Loading(mut path) => {
                if let Some(Event::Key(KeyEvent {code, ..})) = event {
                    if let KeyCode::Char(character) = code {
                        path += &String::from(character);
                    } else if code == KeyCode::Enter {
                        let mut file = File::open(&path)?;
                        self.edit_state.buffer.clear();
                        file.read_to_string(&mut self.edit_state.buffer)?;
                        self.state = State::Editing;
                        return self.update(None);
                    } else if code == KeyCode::Esc {
                        self.state = State::Editing;
                        return self.update(None);
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
}

#[derive(Clone)]
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
