#![cfg(windows)] #![feature(lang_items)] #![no_std] #![no_main]
#![windows_subsystem = "console"]

use core::hint::unreachable_unchecked;
use winapi::shared::minwindef::DWORD;
use winapi::um::{
    consoleapi::WriteConsoleA,
    processenv::GetStdHandle,
    processthreadsapi::ExitProcess,
    winbase::STD_OUTPUT_HANDLE,
    winnt::VOID,
};

#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup() -> ! {
    const HELLO: &'static str = "Hello, world!\n\0";
    let mut rit: DWORD = 0;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteConsoleA(handle,
            HELLO.as_ptr() as *const VOID,
            HELLO.len() as DWORD,
            &mut rit, 0 as *mut VOID);
            ExitProcess(0);
            unreachable_unchecked()
        }
}

#[panic_handler] #[no_mangle]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { unsafe {core::hint::unreachable_unchecked(); } }
#[lang = "eh_personality"] #[no_mangle]
pub extern "C" fn eh_personality() {}
