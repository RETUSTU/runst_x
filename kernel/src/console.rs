use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::prelude::*;
use uefi::table::boot::BootServices;
use uefi::{print, Char16, CStr16};
use uefi_services::println;

pub struct Shell<'a> {
    st: &'a mut SystemTable<Boot>,
    input: Vec<Char16>,
}

impl<'a> Shell<'a> {
    pub fn new(st: &'a mut SystemTable<Boot>) -> Self {
        Self { st, input: Vec::new() }
    }

    pub fn boot_services(&mut self) -> &BootServices {
        self.st.boot_services()
    }

    pub fn system_table(&mut self) -> &mut SystemTable<Boot> {
        self.st
    }

    pub fn print_prompt(&self) {
        print!("RunST X> ");
    }

    pub fn read_line(&mut self) -> Option<String> {
        self.input.clear();
        
        loop {
            let key = self.st.stdin().read_key().ok()?;
            if let Some(k) = key {
                match k {
                    uefi::proto::console::text::Key::Printable(c) => {
                        if c == Char16::try_from(0x0D).unwrap() { // Enter
                            println!();
                            break;
                        } else if c == Char16::try_from(0x08).unwrap() { // Backspace
                            if !self.input.is_empty() {
                                self.input.pop();
                                let _ = self.st.stdout().write_str("\u{8} \u{8}");
                            }
                        } else {
                            self.input.push(c);
                            let mut buf = [0u16; 2];
                            buf[0] = c.into();
                            let s = CStr16::from_u16_with_nul(&buf).unwrap_or_else(|_| cstr16!("?"));
                            let _ = write!(self.st.stdout(), "{}", s);
                        }
                    }
                    _ => {}
                }
            }
        }

        let utf16: Vec<u16> = self.input.iter().map(|c| (*c).into()).collect();
        String::from_utf16(&utf16).ok().map(|s| s.trim().to_string())
    }
}