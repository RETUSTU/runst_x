use alloc::string::String;

use crate::console::Shell;

mod ver;
mod help;
mod reboot;
mod shutdown;

pub fn dispatch(shell: &mut Shell, line: String) {
    if line.is_empty() {
        return;
    }

    let cmd = line.split_whitespace().next().unwrap_or("");

    match cmd {
        "ver" => ver::run(shell),
        "help" => help::run(shell),
        "reboot" => reboot::run(shell),
        "shutdown" => shutdown::run(shell),
        _ => {
            uefi_services::println!("Unknown command: {}", cmd);
            uefi_services::println!("Type 'help' for list.");
        }
    }
}