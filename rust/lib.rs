#![no_std]

extern crate error;
extern crate result;

use error::*;
use result::Result;

pub trait Formatter {
    fn write_str(&mut self, s: &str, len: usize) -> Result<()>;
    fn as_str(&self) -> &str;
}

pub trait Display {
    fn format(&self, f: &mut dyn Formatter) -> Result<()>;
}

impl Display for Error {
    fn format(&self, f: &mut dyn Formatter) -> Result<()> {
        Ok(())
    }
}

impl Display for Backtrace {
    fn format(&self, f: &mut dyn Formatter) -> Result<()> {
        Ok(())
    }
}
