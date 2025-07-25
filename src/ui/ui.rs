use crate::core::State;
use std::error::Error;

pub enum UIType {
    Terminal,
}

pub trait UI
where
    Self: Sized,
{
    fn new() -> Result<Self, Box<dyn Error>>;
    fn render(&self, state: &State);
    fn finalize(self) -> Result<(), Box<dyn Error>>;
}
