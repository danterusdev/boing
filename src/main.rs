mod application;
mod editor;
mod files;

use std::{error::Error, io::stdout, collections::HashMap};

use application::{Application, Editor};
use crossterm::{event::{read, Event}, terminal::{enable_raw_mode, Clear, ClearType}, execute, cursor::MoveTo};
use files::Files;

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;

    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    let mut state = SystemState {
        current_application: Box::new(Files::new()?),
    };

    update_application(&mut state, None)?;
    loop {
        let event = read()?;
        update_application(&mut state, Some(event))?;
    }
}

fn update_application(state: &mut SystemState, event: Option<Event>) -> Result<(), Box<dyn Error>> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    let mut context = Context::new();
    state.current_application.update(event, &mut context)?;

    for (key, value) in &context.calls {
        if key == "editor" {
            let editor = editor::Editor::open(value.clone())?;
            state.current_application = editor;
            update_application(state, None)?;
        }
    }

    Ok(())
}

pub struct Context {
    calls: HashMap<String, String>,
}

impl Context {
    fn new() -> Self {
        Self {
            calls: HashMap::new(),
        }
    }
}

impl Context {
    pub fn call(&mut self, application_type: String, arguments: String) {
        self.calls.insert(application_type, arguments);
    }
}

struct SystemState {
    current_application: Box<dyn Application>,
}
