use crate::shell::Shell;
use crate::constants::{SYSTEM_FULL_NAME, BUILD_DATETIME};

pub fn run(shell: &mut Shell) {
    shell.println(SYSTEM_FULL_NAME);
    shell.println(BUILD_DATETIME);
}