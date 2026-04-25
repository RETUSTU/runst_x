#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;
use uefi_services::println;

mod console;
mod commands;

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi_services::init(&mut st) {
        return Status::LOAD_ERROR;
    }

    println!("RunST X kernel started.");
    println!("Type 'help' to list commands.");

    let mut shell = console::Shell::new(&mut st);
    loop {
        shell.print_prompt();
        if let Some(line) = shell.read_line() {
            commands::dispatch(&mut shell, line);
        }
    }
}