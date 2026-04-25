#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use uefi::prelude::*;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::{CStr16, Handle};
use uefi_services::println;
use core::fmt::Write;

const BOOT_TIMEOUT: u64 = 30; // 30秒自动启动

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi_services::init(&mut st) {
        return Status::LOAD_ERROR;
    }

    println!("RunST X bootloader starting...");
    
    // 显示引导菜单
    let selected_option = show_boot_menu(&mut st);
    
    match selected_option {
        BootMenuOption::BootSystem => {
            println!("Booting RunST X...");
            // 继续执行原来的启动逻辑
        }
        BootMenuOption::Reboot => {
            println!("Rebooting system...");
            st.runtime_services().reset(
                uefi::table::runtime::ResetType::COLD,
                Status::SUCCESS,
                None,
            );
            // 如果reset失败，继续执行
            println!("Reboot failed, continuing to boot...");
        }
        BootMenuOption::Shutdown => {
            println!("Shutting down...");
            st.runtime_services().reset(
                uefi::table::runtime::ResetType::SHUTDOWN,
                Status::SUCCESS,
                None,
            );
            // 如果reset失败，继续执行
            println!("Shutdown failed, continuing to boot...");
        }
    }

    // 打开当前镜像所在分区（一般是 ESP）
    let loaded_image = st
        .boot_services()
        .open_protocol_exclusive::<LoadedImage>(handle)
        .unwrap();

    let device_handle = loaded_image.device();

    let mut fs = st
        .boot_services()
        .open_protocol_exclusive::<SimpleFileSystem>(device_handle.unwrap())
        .unwrap();

    let mut root = fs.open_volume().unwrap();

    // 这里约定内核路径在 ESP 上的: \RunST_X\kernel\main\kernel.nep
    let kernel_path = cstr16!(r"\RunST_X\kernel\main\kernel.nep");

    // 先释放loaded_image的借用，然后再调用read_all
    let kernel_file = match open_file(&mut root, kernel_path) {
        Ok(f) => f,
        Err(_) => {
            println!("Kernel not found: {}", kernel_path);
            return Status::NOT_FOUND;
        }
    };

    // 释放loaded_image和fs的借用
    drop(loaded_image);
    drop(fs);

    let kernel_buf = match read_all(&mut st, kernel_file) {
        Ok(b) => b,
        Err(_) => {
            println!("Failed to read kernel file");
            return Status::LOAD_ERROR;
        }
    };

    // 为内核分配内存并复制
    let pages = (kernel_buf.len() + 0xFFF) / 0x1000;
    let kernel_addr = st
        .boot_services()
        .allocate_pages(
            AllocateType::AnyPages,
            MemoryType::LOADER_DATA,
            pages,
        )
        .unwrap();

    unsafe {
        core::ptr::copy_nonoverlapping(
            kernel_buf.as_ptr(),
            kernel_addr as *mut u8,
            kernel_buf.len(),
        );
    }

    // 加载并启动内核 EFI 镜像
    let kernel_handle = st
        .boot_services()
        .load_image(
            handle,
            uefi::table::boot::LoadImageSource::FromBuffer {
                buffer: unsafe {
                    core::slice::from_raw_parts(kernel_addr as *const u8, kernel_buf.len())
                },
                file_path: None,
            },
        )
        .unwrap();

    println!("Jumping to kernel...");

    // 关闭 boot services 前不需要传额外参数，本例内核自初始化
    let _ = st.boot_services().start_image(kernel_handle);

    println!("Kernel returned. Halting.");
    Status::SUCCESS
}

fn open_file(root: &mut Directory, path: &CStr16) -> Result<uefi::proto::media::file::FileHandle, Status> {
    match root.open(path, FileMode::Read, FileAttribute::empty()) {
        Ok(file) => Ok(file),
        Err(e) => Err(Status::from(e.status())),
    }
}

fn read_all(_st: &mut SystemTable<Boot>, file: uefi::proto::media::file::FileHandle) -> Result<Vec<u8>, Status> {
    // 将FileHandle转换为RegularFile
    let file_type = file.into_type().unwrap();
    let mut file = match file_type {
        uefi::proto::media::file::FileType::Regular(f) => f,
        _ => return Err(Status::UNSUPPORTED),
    };
    
    let mut info_buf = [0u8; 512];
    let info: &uefi::proto::media::file::FileInfo = match file.get_info::<uefi::proto::media::file::FileInfo>(&mut info_buf) {
        Ok(info) => info,
        Err(e) => return Err(Status::from(e.status())),
    };
    
    let file_size = info.file_size() as usize;
    let mut buf = vec![0u8; file_size];
    let mut read = 0usize;
    
    while read < file_size {
        let chunk = match file.read(&mut buf[read..]) {
            Ok(chunk) => chunk,
            Err(e) => return Err(Status::from(e.status())),
        };
        if chunk == 0 { break; }
        read += chunk;
    }
    Ok(buf)
}

#[derive(Debug, PartialEq)]
enum BootMenuOption {
    BootSystem,
    Reboot,
    Shutdown,
}

fn show_boot_menu(st: &mut SystemTable<Boot>) -> BootMenuOption {
    // 清屏
    st.stdout().reset(false).unwrap();
    st.stdout().clear().unwrap();
    //显示logo
    println!("______            _____ _____ ");
    println!("| ___ \\          /  ___|_   _|");
    println!("| |_/ /   _ _ __ \\ `--.  | |  ");
    println!("|    / | | | '_ \\ `--. \\ | |  ");
    println!("| |\\ \\ |_| | | | /\\__/ / | |  ");
    println!("\\_| \\_\\__,_|_| |_\\____/  \\_/  ");
    println!("                              ");
    println!("                              ");
    println!("__   __                       ");
    println!("\\ \\ / /                       ");
    println!(" \\ V /                        ");
    println!(" /   \\                        ");
    println!("/ /^\\ \\                       ");
    println!("\\/   \\/                       ");
    println!("                              ");
    // 显示引导菜单标题
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                                                                ║");
    println!("║                    RunST X Boot Menu                           ║");
    println!("║                          v1.1.0                                ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!("");
    
    // 显示菜单选项
    println!("Please select an option:");
    println!("");
    println!("1. Boot RunST X System");
    println!("2. Reboot System");
    println!("3. Shutdown System");
    println!("");
    
    let mut input_buffer: Vec<u16> = Vec::new();
    
    loop {
        println!("Enter option number (1-3): ");
        
        // 等待用户输入
        loop {
            // 检查键盘输入
            if let Some(key) = read_key(st) {
                match key {
                    uefi::proto::console::text::Key::Printable(c) => {
                        if c == uefi::Char16::try_from(0x0D).unwrap() { // Enter
                            if !input_buffer.is_empty() {
                                let input_str = String::from_utf16(&input_buffer).unwrap_or_default();
                                if let Some(option) = parse_option(&input_str) {
                                    println!("");
                                    return option;
                                }
                                input_buffer.clear();
                                println!("\nEnter option number (1-3): ");
                            }
                        } else if c == uefi::Char16::try_from(0x08).unwrap() { // Backspace
                            if !input_buffer.is_empty() {
                                input_buffer.pop();
                                let _ = write!(st.stdout(), "\u{8} \u{8}");
                            }
                        } else {
                            input_buffer.push(c.into());
                            let mut buf = [0u16; 2];
                            buf[0] = c.into();
                            let s = CStr16::from_u16_with_nul(&buf).unwrap_or_else(|_| cstr16!("?"));
                            let _ = write!(st.stdout(), "{}", s);
                        }
                    }
                    _ => {
                        // 忽略非可打印字符
                    }
                }
            }
            
            // 短暂延迟以避免CPU占用过高
            let bt = st.boot_services();
            bt.stall(10_000); // 10ms
        }
    }
}

fn read_key(st: &mut SystemTable<Boot>) -> Option<uefi::proto::console::text::Key> {
    // 直接尝试读取键盘输入
    let stdin = st.stdin();
    
    match stdin.read_key() {
        Ok(Some(key)) => Some(key),
        Ok(None) => None,
        Err(_) => None,
    }
}

fn parse_option(input: &str) -> Option<BootMenuOption> {
    match input.trim() {
        "1" => Some(BootMenuOption::BootSystem),
        "2" => Some(BootMenuOption::Reboot),
        "3" => Some(BootMenuOption::Shutdown),
        _ => {
            println!("Invalid option. Please enter 1, 2, or 3.");
            None
        }
    }
}