use alloc::string::String;
use uefi::prelude::*;
use uefi::proto::console::text::Color;

use crate::keyboard::Keyboard;
use crate::display::Display;
use crate::constants::{SYSTEM_PROMPT, WELCOME_MESSAGE, HELP_PROMPT};

pub struct Shell<'a> {
    st: &'a mut SystemTable<Boot>,
    prompt: &'static str,
}

impl<'a> Shell<'a> {
    pub fn new(st: &'a mut SystemTable<Boot>) -> Self {
        Self {
            st,
            prompt: SYSTEM_PROMPT,
        }
    }

    pub fn print_prompt(&mut self) {
        Display::set_foreground_color(self.st, Color::Green);
        Display::print(self.st, self.prompt);
        Display::reset_color(self.st);
    }

    pub fn read_line(&mut self) -> Option<String> {
        Keyboard::read_line(self.st)
    }

    pub fn print(&mut self, text: &str) {
        Display::print(self.st, text);
    }

    pub fn println(&mut self, text: &str) {
        Display::println(self.st, text);
    }

    pub fn print_error(&mut self, text: &str) {
        Display::print_error(self.st, text);
    }

    pub fn print_success(&mut self, text: &str) {
        Display::print_success(self.st, text);
    }

    pub fn print_warning(&mut self, text: &str) {
        Display::print_warning(self.st, text);
    }

    pub fn print_info(&mut self, text: &str) {
        Display::print_info(self.st, text);
    }

    pub fn clear_screen(&mut self) {
        Display::clear_screen(self.st);
    }

    pub fn show_banner(&mut self) {
        Display::print_banner(self.st);
    }

    pub fn system_table(&mut self) -> &mut SystemTable<Boot> {
        self.st
    }
}