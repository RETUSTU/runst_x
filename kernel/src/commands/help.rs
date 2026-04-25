use alloc::format;
use crate::shell::Shell;
use crate::constants::{commands, command_descriptions};

pub fn run(shell: &mut Shell) {
    shell.println("Available commands:");
    shell.println(&format!("  {:<8} - {}", commands::VER, command_descriptions::VER));
    shell.println(&format!("  {:<8} - {}", commands::HELP, command_descriptions::HELP));
    shell.println(&format!("  {:<8} - {}", commands::REBOOT, command_descriptions::REBOOT));
    shell.println(&format!("  {:<8} - {}", commands::SHUTDOWN, command_descriptions::SHUTDOWN));
    shell.println(&format!("  {:<8} - {}", commands::ECHO, command_descriptions::ECHO));
    shell.println(&format!("  {:<8} - {}", commands::CLEAR, command_descriptions::CLEAR));
}