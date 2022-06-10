#![cfg(windows)] #![feature(lang_items)] #![no_std] #![no_main]
#![windows_subsystem = "windows"]

use core::hint::unreachable_unchecked;
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winuser::{
    GetDesktopWindow, MessageBoxA, MB_OK,
};

#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup() -> ! {
    unsafe {
        let desktop = GetDesktopWindow();
        _ = MessageBoxA(desktop,
            "Minimal example.".as_ptr() as *const i8,
            "From Rust:".as_ptr() as *const i8,
            MB_OK);
            ExitProcess(0);
            unreachable_unchecked();
        }
}

#[panic_handler] #[no_mangle]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { unsafe {core::hint::unreachable_unchecked(); } }
#[lang = "eh_personality"] #[no_mangle]
pub extern "C" fn eh_personality() {}
