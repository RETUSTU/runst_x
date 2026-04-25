#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;

mod keyboard;
mod display;
mod shell;
mod commands;

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi::helpers::init(&mut st) {
        return Status::LOAD_ERROR;
    }

    let mut shell = shell::Shell::new(&mut st);
    
    shell.show_banner();
    shell.println("Type 'help' to list commands.");
    shell.println("");

    loop {
        shell.print_prompt();
        if let Some(line) = shell.read_line() {
            commands::dispatch(&mut shell, line);
        }
    }
}