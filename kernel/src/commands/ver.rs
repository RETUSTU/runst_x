use crate::console::Shell;

pub fn run(_shell: &mut Shell) {
    uefi_services::println!("RunST X v25.12.12A (UEFI DOS-Like)");
}