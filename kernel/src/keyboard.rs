use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::{Char16, CStr16};
use core::fmt::Write;

pub struct Keyboard;

impl Keyboard {
    pub fn read_key(st: &mut SystemTable<Boot>) -> Result<Option<uefi::proto::console::text::Key>, uefi::Error> {
        st.stdin().read_key()
    }

    pub fn read_char(st: &mut SystemTable<Boot>) -> Option<Char16> {
        match Self::read_key(st) {
            Ok(Some(uefi::proto::console::text::Key::Printable(c))) => Some(c),
            _ => None,
        }
    }

    pub fn read_line(st: &mut SystemTable<Boot>) -> Option<String> {
        let mut input: Vec<Char16> = Vec::new();
        
        loop {
            if let Some(key) = Self::read_key(st).ok().flatten() {
                match key {
                    uefi::proto::console::text::Key::Printable(c) => {
                        if c == Char16::try_from(0x0D).unwrap() { // Enter
                            let _ = write!(st.stdout(), "\r\n");
                            break;
                        } else if c == Char16::try_from(0x08).unwrap() { // Backspace
                            if !input.is_empty() {
                                input.pop();
                                let _ = st.stdout().write_str("\u{8} \u{8}");
                            }
                        } else {
                            input.push(c);
                            let mut buf = [0u16; 2];
                            buf[0] = c.into();
                            let s = CStr16::from_u16_with_nul(&buf).unwrap_or_else(|_| cstr16!("?"));
                            let _ = write!(st.stdout(), "{}", s);
                        }
                    }
                    _ => {}
                }
            }
        }

        let utf16: Vec<u16> = input.iter().map(|c| (*c).into()).collect();
        String::from_utf16(&utf16).ok().map(|s| s.trim().to_string())
    }
}