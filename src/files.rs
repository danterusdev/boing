use std::{env::current_dir, fs::{read_dir, File}, io::stdout, error::Error, process::exit, path::PathBuf};

use crossterm::{execute, cursor::MoveTo, style::Print, event::{Event, KeyEvent, KeyCode}, ExecutableCommand};

use crate::{application::Application, Context};

pub struct Files {
    row: u16,
    directory: PathBuf,
}

impl Files {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            row: 0,
            directory: current_dir()?,
        })
    }
}

impl Application for Files {
    fn update(&mut self, event: Option<Event>, context: &mut Context) -> Result<(), Box<dyn Error>> {
        let mut files = read_dir(&self.directory)?;

        if let Some(Event::Key(KeyEvent { code, .. })) = event {
            if code == KeyCode::Down {
                if self.row < files.count() as u16 - 1 + 1 {
                    self.row += 1;
                }
            } else if code == KeyCode::Up {
                if self.row > 0 {
                    self.row -= 1;
                }
            } else if code == KeyCode::Enter {
                match files.nth(self.row as usize) {
                    Some(file) => {
                        let file = file?;
                        let metadata = File::open(file.path())?.metadata()?;
                        if metadata.is_file() {
                            context.call("editor".into(), format!("{}", file.path().display()));
                        } else if metadata.is_dir() {
                            self.directory = file.path();
                            self.row = 0;
                            return self.update(None, context);
                        }
                    },
                    None => {
                        let prev_directory = &self.directory;
                        let prev_directory = prev_directory.clone();
                        self.directory = self.directory.clone().parent().unwrap().into();
                        let position = read_dir(&self.directory)?.position(|file| file.unwrap().file_name() == prev_directory.file_name().unwrap()).unwrap();
                        self.row = position as u16;
                        return self.update(None, context);
                    }
                };
            } else if code == KeyCode::Esc {
                exit(0);
            }
        }

        let files = read_dir(&self.directory)?;

        for (index, file) in files.enumerate() {
            execute!(stdout(), MoveTo(0, index as u16), Print(file?.file_name().to_str().unwrap()))?;
        }

        execute!(stdout(), MoveTo(0, read_dir(&self.directory)?.count() as u16), Print("../"))?;

        stdout().execute(MoveTo(0, self.row))?;

        Ok(())
    }
}
