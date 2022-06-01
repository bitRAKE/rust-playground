#![feature(lang_items)]
#![no_std]
#![no_main]
#![windows_subsystem = "windows"]
use core::ffi::*;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"] //#[no_mangle]
pub extern "C" fn eh_personality() {}


#[repr(transparent)]
pub struct HANDLE(pub isize);

#[link(name = "User32")]
extern "system" { fn GetDesktopWindow() -> HANDLE; }
#[link(name = "User32")]
extern "system" { fn MessageBoxA(hWnd: HANDLE, lpText: *mut c_void, lpCaption: *mut c_void, uType: u32) -> i32; }

// Rust always forces this name - even for GUI applications:
// see: https://github.com/rust-lang/rust/blob/b5a2d27f8f59df6f2162e61461b41d6116d4815e/compiler/rustc_codegen_ssa/src/back/linker.rs#L1003
#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup() -> isize {
    const CAPTION: &'static str = "From Rust:\0";
    const TEXT: &'static str = "Minimal example.\0";
    unsafe {
        let desktop = GetDesktopWindow();
        MessageBoxA(
            desktop as HANDLE,
            TEXT.as_ptr() as *mut c_void,
            CAPTION.as_ptr() as *mut c_void,
            0 as u32) as isize
    }
}

#[cfg(not(windows))]
fn main() {}
