#![no_std]
#![no_main]
#![feature(core_intrinsics)]

extern crate rlibc;
extern crate bootloader;

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {

    unsafe { intrinsics::abort() }
}

static HELLOWORLD: &[u8] = b"Hey Keshiba";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLOWORLD.iter().enumerate() {
        
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop { }
}
