use uefi::prelude::*;
use uefi::proto::console::text::Color;
use core::fmt::Write;

pub struct Display;

impl Display {
    pub fn clear_screen(st: &mut SystemTable<Boot>) {
        let _ = st.stdout().clear();
    }

    pub fn set_foreground_color(st: &mut SystemTable<Boot>, color: Color) {
        let _ = st.stdout().set_color(color, Color::Black);
    }

    pub fn set_background_color(st: &mut SystemTable<Boot>, color: Color) {
        let _ = st.stdout().set_color(Color::White, color);
    }

    pub fn reset_color(st: &mut SystemTable<Boot>) {
        let _ = st.stdout().set_color(Color::White, Color::Black);
    }

    pub fn print(st: &mut SystemTable<Boot>, text: &str) {
        let _ = write!(st.stdout(), "{}", text);
    }

    pub fn println(st: &mut SystemTable<Boot>, text: &str) {
        let _ = write!(st.stdout(), "{}\r\n", text);
    }

    pub fn print_colored(st: &mut SystemTable<Boot>, text: &str, foreground: Color, background: Color) {
        let _ = st.stdout().set_color(foreground, background);
        let _ = write!(st.stdout(), "{}", text);
        let _ = st.stdout().set_color(Color::White, Color::Black);
    }

    pub fn println_colored(st: &mut SystemTable<Boot>, text: &str, foreground: Color, background: Color) {
        Self::print_colored(st, text, foreground, background);
        let _ = write!(st.stdout(), "\r\n");
    }

    pub fn print_error(st: &mut SystemTable<Boot>, text: &str) {
        Self::println_colored(st, text, Color::Red, Color::Black);
    }

    pub fn print_success(st: &mut SystemTable<Boot>, text: &str) {
        Self::println_colored(st, text, Color::Green, Color::Black);
    }

    pub fn print_warning(st: &mut SystemTable<Boot>, text: &str) {
        Self::println_colored(st, text, Color::Yellow, Color::Black);
    }

    pub fn print_info(st: &mut SystemTable<Boot>, text: &str) {
        Self::println_colored(st, text, Color::Cyan, Color::Black);
    }

    pub fn print_banner(st: &mut SystemTable<Boot>) {
        Self::clear_screen(st);
        Self::println_colored(st, "╔════════════════════════════════════════════════════════════════╗", Color::Cyan, Color::Black);
        Self::println_colored(st, "║                                                                ║", Color::Cyan, Color::Black);
        Self::println_colored(st, "║                        RunST X v25.12.12B                      ║", Color::Green, Color::Black);
        Self::println_colored(st, "║                     UEFI Operating System                      ║", Color::Yellow, Color::Black);
        Self::println_colored(st, "║                                                                ║", Color::Cyan, Color::Black);
        Self::println_colored(st, "╚════════════════════════════════════════════════════════════════╝", Color::Cyan, Color::Black);
        Self::println(st, "");
        Self::print_success(st, "System initialized successfully!");
        Self::println(st, "");
    }
}