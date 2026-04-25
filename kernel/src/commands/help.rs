use crate::console::Shell;

pub fn run(_shell: &mut Shell) {
    uefi_services::println!("Commands:");
    uefi_services::println!("  ver       Show version");
    uefi_services::println!("  help      Show this help");
    uefi_services::println!("  reboot    Reboot machine via UEFI ResetSystem");
    uefi_services::println!("  shutdown  Power off via UEFI ResetSystem");
}