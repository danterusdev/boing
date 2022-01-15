use std::error::Error;

use crossterm::event::Event;

use crate::Context;

pub trait Application {
    fn update(&mut self, event: Option<Event>, context: &mut Context) -> Result<(), Box<dyn Error>>;
}

pub trait Editor: Application {
    fn open(file: String) -> Result<Box<dyn Application>, Box<dyn Error>>;
}
