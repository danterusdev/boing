use std::{env::current_dir, fs::read_dir, io::stdout, error::Error, process::exit};

use crossterm::{execute, cursor::MoveTo, style::Print, event::{Event, KeyEvent, KeyCode}, ExecutableCommand};

use crate::{application::Application, Context};

pub struct Files {
    row: u16
}

impl Files {
    pub fn new() -> Self {
        Self {
            row: 0
        }
    }
}

impl Application for Files {
    fn update(&mut self, event: Option<Event>, context: &mut Context) -> Result<(), Box<dyn Error>> {
        let directory = current_dir()?;
        let mut files = read_dir(&directory)?;

        if let Some(Event::Key(KeyEvent { code, .. })) = event {
            if code == KeyCode::Down {
                if self.row < files.count() as u16 - 1 {
                    self.row += 1;
                }
            } else if code == KeyCode::Up {
                if self.row > 0 {
                    self.row -= 1;
                }
            } else if code == KeyCode::Enter {
                let file = files.nth(self.row as usize).unwrap()?;
                context.call("editor".into(), format!("{}", file.path().display()));
            } else if code == KeyCode::Esc {
                exit(0);
            }
        }

        let files = read_dir(directory)?;

        for (index, file) in files.enumerate() {
            execute!(stdout(), MoveTo(0, index as u16), Print(file?.file_name().to_str().unwrap()))?;
        }

        stdout().execute(MoveTo(0, self.row))?;

        Ok(())
    }
}
