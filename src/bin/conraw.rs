#![cfg(windows)] #![feature(lang_items)] #![no_std] #![no_main]
#![windows_subsystem = "console"]
use core::ffi::*;

pub struct BOOL(pub i32);
impl BOOL {
    #[inline]
    pub fn as_bool(self) -> bool { self.0 != 0 }
}

#[repr(C)] // better than #[repr(transparent)]?
pub struct HANDLE(pub isize);
//const STD_INPUT_HANDLE:     isize = -10isize;
const STD_OUTPUT_HANDLE:    isize = -11isize;
//const STD_ERROR_HANDLE:     isize = -12isize;

#[link(name = "Kernel32")]
extern "system" {
    fn GetStdHandle(nStdHandle: isize) -> HANDLE;
}

#[link(name = "Kernel32")]
extern "system" {
    fn WriteConsoleA(hConsoleOutput: HANDLE,
        lpBuffer: *mut c_void,
        nNumberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *mut c_void) -> bool;
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup() -> isize {
    const HELLO: &'static str = "Hello, world!\n\0";
    let mut rit: u32 = 0;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteConsoleA(
            handle,
            HELLO.as_ptr() as *mut c_void,
            HELLO.len() as u32,
            &mut rit,
            0 as *mut c_void);
    };
    0 // return ERROR_SUCCESS
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { loop {} }
#[lang = "eh_personality"] //#[no_mangle]
pub extern "C" fn eh_personality() {}
