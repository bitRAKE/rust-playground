#![cfg(windows)]
#![feature(lang_items, alloc_error_handler)]
#![no_std] #![no_main]
#![windows_subsystem = "console"]

// https://doc.rust-lang.org/rust-by-example/unsafe/asm.html
// https://doc.rust-lang.org/reference/inline-assembly.html
// https://rust-lang.github.io/rfcs/2873-inline-asm.html

//use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::hint::unreachable_unchecked;
use core::mem::MaybeUninit;
use winapi::shared::minwindef::DWORD;
use winapi::um::{
    consoleapi::WriteConsoleA,
//    heapapi::{GetProcessHeap, HeapAlloc, HeapFree},
    processenv::GetStdHandle,
    processthreadsapi::ExitProcess,
    winbase::STD_OUTPUT_HANDLE,
    winnt::VOID,
    winuser::wsprintfA,
};


#[allow(non_snake_case)]
#[inline(always)]
/// Try to showcase all the asm!() features in one routine, lol:
///
/// A000931: Padovan sequence (or Padovan numbers):
///      [1,0,0],1,0,1,1,1,2,2,3,4,5,7,9,12,16,21,28,37,49,65,86,114,151,...
/// `Plastic number, only real solution to x^3 = x + 1`
fn A000931(nth: u64 ) -> u64 {
    let mut result:u64 = unsafe { MaybeUninit::uninit().assume_init() };
    unsafe {asm!(

"           push    1
            pop     {p0}
            xor     {p1:e}, {p1:e}
            xor     eax, eax
        7:  lea     {p0}, [{p0}+{p1}]
            xchg    rax, {p1}
            xchg    rax, {p0}
            loop    7b
",
            p0 = out(reg) _,
            p1 = out(reg) _,
            inlateout("rcx") nth => _,
            inlateout("rax") result,
            options(nomem),
    )};
    result
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn mainCRTStartup() -> ! {
    let buffer: [ MaybeUninit<u8>; 1024 ] = unsafe {
        MaybeUninit::uninit().assume_init()
    };
    let mut rit: DWORD = 0;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        for x in 1..24 {
            let k = wsprintfA(
                buffer.as_ptr() as * mut i8, 
                "%d, ".as_ptr() as * const i8,
                A000931(x)
            );
            WriteConsoleA(
                handle,
                buffer.as_ptr() as *const VOID,
                k as DWORD,
                &mut rit,
                0 as *mut VOID);
        };

        ExitProcess(0);
        unreachable_unchecked()
    }
}

#[alloc_error_handler] #[no_mangle]
fn error_handler(_: core::alloc::Layout) -> ! { unsafe { core::hint::unreachable_unchecked(); } }
#[panic_handler] #[no_mangle]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { unsafe {core::hint::unreachable_unchecked(); } }
#[lang = "eh_personality"] #[no_mangle]
pub extern "C" fn eh_personality() {}
