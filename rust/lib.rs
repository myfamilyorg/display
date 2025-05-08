#![no_std]

extern crate error;
extern crate result;

use error::*;
use result::Result;

pub trait Fmt {
    fn append(&mut self, s: &str) -> Result<()>;
    fn to_str(&self) -> &str;
}

pub trait Display {
    fn format(&self, f: &mut dyn Fmt) -> Result<()>;
}

impl Display for Error {
    fn format(&self, f: &mut dyn Fmt) -> Result<()> {
        Ok(())
    }
}

impl Display for Backtrace {
    fn format(&self, f: &mut dyn Fmt) -> Result<()> {
        Ok(())
    }
}
