#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use alloc::vec;
use uefi::prelude::*;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::loaded_image::LoadedImage;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::{CStr16, Handle};
use uefi_services::println;

#[entry]
fn efi_main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi_services::init(&mut st) {
        return Status::LOAD_ERROR;
    }

    println!("RunST X bootloader starting...");

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

fn read_all(st: &mut SystemTable<Boot>, file: uefi::proto::media::file::FileHandle) -> Result<Vec<u8>, Status> {
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