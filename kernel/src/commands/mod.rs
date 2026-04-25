use alloc::string::String;
use alloc::format;

use crate::shell::Shell;
use crate::constants::{commands, UNKNOWN_COMMAND_HELP};

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
        commands::VER => ver::run(shell),
        commands::HELP => help::run(shell),
        commands::REBOOT => reboot::run(shell),
        commands::SHUTDOWN => shutdown::run(shell),
        commands::ECHO => echo::run(shell, line),
        commands::CLEAR => clear::run(shell),
        _ => {
            shell.print_error(&format!("Unknown command: {}", cmd));
            shell.println("");
            shell.println(UNKNOWN_COMMAND_HELP);
        }
    }
}