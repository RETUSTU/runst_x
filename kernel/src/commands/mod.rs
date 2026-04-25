use alloc::string::String;
use alloc::format;

use crate::shell::Shell;

mod ver;
mod help;
mod reboot;
mod shutdown;
mod echo;
mod clear;

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
        "echo" => echo::run(shell, line),
        "clear" => clear::run(shell),
        _ => {
            shell.print_error(&format!("Unknown command: {}", cmd));
            shell.println("");
            shell.println("Type 'help' for list.");
        }
    }
}