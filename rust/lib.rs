#![no_std]

extern crate ffi;
use core::str::from_utf8_unchecked;

pub fn u128_as_str(mut n: u128, offset: usize, buf: &mut [u8], base: u8) -> usize {
    let buf_len = buf.len();
    let mut i = buf_len - 1;

    while n > 0 {
        if i == 0 {
            break;
        }
        if i < buf_len && base != 0 {
            let digit = (n % base as u128) as u8;
            buf[i] = if digit < 10 {
                b'0' + digit
            } else {
                b'a' + (digit - 10)
            };
        }
        if base != 0 {
            n /= base as u128;
        }
        i -= 1;
    }

    let mut len = buf_len - i - 1;

    if len == 0 && buf_len > 0 && offset < buf_len {
        buf[offset] = b'0';
        len = 1;
    } else {
        let mut k = 0;
        for j in i + 1..buf_len {
            if k + offset < buf_len {
                buf[k + offset] = buf[j];
            }
            k += 1;
        }
    }

    len
}

pub fn i128_as_str(mut n: i128, buf: &mut [u8], base: u8) -> usize {
    if n < 0 {
        n *= -1;
        if buf.len() < 2 {
            0
        } else {
            buf[0] = b'-';
            u128_as_str(n as u128, 1, buf, base) + 1
        }
    } else {
        u128_as_str(n as u128, 0, buf, base)
    }
}

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

impl Display for &str {
    fn format(&self, f: &mut dyn Fmt) -> Result<()> {
        f.append(self)
    }
}

macro_rules! impl_display_unsigned {
        ($($t:ty),*) => {
                $(
                        impl Display for $t {
                                fn format(&self, f: &mut dyn Fmt) -> Result<()> {
                                        let mut buf = [0u8; 64];
                                        let len = u128_as_str((*self) as u128, 0, &mut buf, 10);
                                        unsafe { f.append(from_utf8_unchecked(&buf[0..len]) ) }
                                }
                        }
                )*
        };
}

impl_display_unsigned!(u8, u16, u32, u64, u128);

impl Display for usize {
    fn format(&self, f: &mut dyn Fmt) -> Result<()> {
        let mut buf = [0u8; 64];
        let len = u128_as_str(*self as u128, 0, &mut buf, 10);
        unsafe { f.append(from_utf8_unchecked(&buf[0..len])) }
    }
}

macro_rules! impl_display_signed {
        ($($t:ty),*) => {
                $(
                        impl Display for $t {
                                fn format(&self, f: &mut dyn Fmt) -> Result<()> {
                                        let mut buf = [0u8; 64];
                                        let len = i128_as_str((*self) as i128, &mut buf, 10);
                                        unsafe { f.append(from_utf8_unchecked(&buf[0..len]) ) }
                                }
                        }
                )*
        };
}

impl_display_signed!(i8, i16, i32, i64, i128);

impl Display for isize {
    fn format(&self, f: &mut dyn Fmt) -> Result<()> {
        let mut buf = [0u8; 64];
        let len = i128_as_str((*self) as i128, &mut buf, 10);
        unsafe { f.append(from_utf8_unchecked(&buf[0..len])) }
    }
}
