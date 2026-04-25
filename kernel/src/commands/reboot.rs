use crate::console::Shell;
use uefi::table::runtime::ResetType;

pub fn run(shell: &mut Shell) {
    uefi_services::println!("Rebooting...");
    let rt = shell.system_table().runtime_services();
    rt.reset(ResetType::COLD, uefi::Status::SUCCESS, None);
}