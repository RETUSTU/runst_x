use crate::shell::Shell;

pub fn run(shell: &mut Shell) {
    shell.println("Available commands:");
    shell.println("  ver      - Show system version");
    shell.println("  help     - Show this help");
    shell.println("  reboot   - Reboot the system");
    shell.println("  shutdown - Shutdown the system");
    shell.println("  echo     - Echo text to screen");
    shell.println("  clear    - Clear the screen");
}