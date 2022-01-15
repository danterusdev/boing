use std::error::Error;

use crossterm::event::Event;

pub trait Application {
    fn update(&mut self, event: Option<Event>) -> Result<(), Box<dyn Error>>;
}
