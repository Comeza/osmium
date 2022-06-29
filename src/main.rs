#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use alloc::alloc::Layout;
use alloc::format;
use core::fmt::Write;
use core::panic;
use core::panic::PanicInfo;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};
use uefi::proto::console::text::ScanCode;

use uefi::{prelude::*, CStr16, Char16};

extern crate alloc;

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    use uefi::proto::console::text::Key;
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    };
    system_table = system_table;

    system_table.stdout().clear().unwrap();
    system_table.stdout().enable_cursor(true).unwrap();

    loop {
        system_table.stdin().wait_for_key_event();
        if let Ok(Some(key)) = system_table.stdin().read_key() {
            match key {
                Key::Printable(key) => system_table
                    .stdout()
                    .write_str(&format!("{}", key))
                    .unwrap(),
                _ => {}
            }
        }
    }

    //Status::SUCCESS
}

#[alloc_error_handler]
fn _alloc_err(_: Layout) -> ! {
    panic!("lol")
}

#[panic_handler]
fn _panic(_: &PanicInfo) -> ! {
    loop {}
}
