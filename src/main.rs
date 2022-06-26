#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use alloc::alloc::Layout;
use alloc::format;
use core::panic;
use core::panic::PanicInfo;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

use uefi::{prelude::*, CStr16};

extern crate alloc;

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    unsafe {
        uefi::alloc::init(system_table.boot_services());
    };
    system_table = system_table;

    system_table.stdout().clear().unwrap();
    system_table.stdout().enable_cursor(true).unwrap();

    let mut buf = [0; 255];

    let mut fb = match system_table
        .boot_services()
        .locate_protocol::<GraphicsOutput>()
    {
        Err(e) => {
            system_table
                .stdout()
                .output_string(
                    &CStr16::from_str_with_buf(
                        &format!("EfiPtorocol ERROR {:?}", e.status()),
                        &mut buf,
                    )
                    .unwrap(),
                )
                .unwrap();
            panic!("lol");
        }
        Ok(proto) => {
            let gp: &mut GraphicsOutput = unsafe { &mut *proto.get() };
            gp.frame_buffer()
        }
    };

    unsafe {
        fb.write_byte(100, 0xAC);
    };

    loop {}
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
