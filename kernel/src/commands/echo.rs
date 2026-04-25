use alloc::string::String;
use alloc::vec::Vec;
use crate::shell::Shell;

pub fn run(shell: &mut Shell, line: String) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() > 1 {
        shell.println(parts[1]);
    } else {
        shell.println("");
    }
}