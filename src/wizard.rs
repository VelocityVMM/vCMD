//! Provides configuration wizards to ease input-heavy configurations

mod corevm;
mod disks;
mod displays;
mod nics;

pub use corevm::*;
pub use disks::*;
pub use displays::*;
pub use nics::*;

use rustyline::{error::ReadlineError, history::History, Editor, Helper};
use std::str::FromStr;

/// A simple yes or no answer to a prompt that parses
/// `y` to Self::YES and `n` to Self::NO
pub enum YesNo {
    /// true
    YES,
    /// false
    NO,
}

impl FromStr for YesNo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "y" => Self::YES,
            "n" => Self::NO,
            _ => return Err("Expected 'y' or 'n'".to_owned()),
        })
    }
}

impl Into<bool> for YesNo {
    fn into(self) -> bool {
        match self {
            YesNo::YES => true,
            YesNo::NO => false,
        }
    }
}

/// Extended readline functions
trait ReadlineExt {
    /// Read in a line and try to parse the line to the supplied type,
    /// repeating the prompt on failure
    /// # Arguments
    /// * `promp` - The prompt to display
    fn readline_t<T: FromStr>(&mut self, prompt: &str) -> Result<T, ReadlineError>
    where
        <T as FromStr>::Err: std::fmt::Display;
}

impl<H: Helper, I: History> ReadlineExt for Editor<H, I> {
    fn readline_t<T: FromStr>(&mut self, prompt: &str) -> Result<T, ReadlineError>
    where
        <T as FromStr>::Err: std::fmt::Display,
    {
        let mut last = String::new();
        loop {
            last = self.readline_with_initial(prompt, (&last, ""))?;
            match last.parse::<T>() {
                Ok(v) => {
                    self.add_history_entry(&last)?;
                    return Ok(v);
                }
                Err(e) => {
                    println!("{}, try again", e);
                }
            }
        }
    }
}
