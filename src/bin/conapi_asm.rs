#![cfg(windows)]
#![feature(lang_items, alloc_error_handler)]
#![no_std] #![no_main]
#![windows_subsystem = "console"]
#![allow(non_snake_case)]

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


//#[repr(align(16))]
// str is utf8, but these values are all in ASCII range
const DIGIT_TABLE:&str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[inline(always)]
fn BaseForm( buffer: *mut u8, number: u64, base: u8 ) {
    assert!(0 < base && base < DIGIT_TABLE.len() as u8);
    unsafe { asm!(r#"
       	    push    0
        2:  xor     edx, edx
            div     {base}
            push    [{tab}+rdx]
            test    rax, rax
            jnz     2b
        3:  pop     rax
            stosb
            test    al, al
            jnz     3b
"#,         base = in(reg) u64::from(base),     // unchanged
            tab = in(reg) DIGIT_TABLE.as_ptr(), // unchanged
            inout("rdi") buffer => _,          // unchanged
            inout("rax") number => _,
            // RDX used internally and not availible for other uses
            out("rdx") _,
    )};
}


#[inline(never)]
/// `This WILL cause an exception if (n,k) produce too large a result.`
/// https://en.wikipedia.org/wiki/Binomial_coefficient
///
fn binomial_coefficient( n:u64, k:u64 ) -> u64 {
    let mut result:u64;
    unsafe { asm!(
// this hack because we don't know what the epilogue/prologue is doing
// maybe there is a way to make a naked leaf fn?
"           jmp     9f
        2:  lea     edx,[rcx*2]
            jrcxz   4f
            cmp     eax, edx
            jnc     3f
            sub     ecx, eax
            neg     ecx
        3:  push    rcx
            push    rax
            sub     ecx, 1
            sub     eax, 1
            call    2b
            pop     rdx
            pop     rcx
            mul     rdx
            div     rcx
            retn
        4:  lea     eax,[rcx+1]
            retn
        9:  call    2b
",          out("rdx") _,
            inout("rcx") k => _,
            inout("rax") n => result,
            options(pure, nomem),
    )};
    result
}


#[inline(always)]
/// Try to showcase all the asm!() features in one routine, lol:
///
/// A000931: Padovan sequence (or Padovan numbers):
///      [1,0,0],1,0,1,1,1,2,2,3,4,5,7,9,12,16,21,28,37,49,65,86,114,151,...
/// `Plastic number, only real solution to x^3 = x + 1`
fn A000931( nth: u64 ) -> u64 {
    let mut result:u64;
    unsafe { asm!(
"           push    1
            pop     {p0}
            xor     {p1:e}, {p1:e}
            xor     eax, eax
        7:  lea     {p0}, [{p0}+{p1}]
            xchg    rax, {p1}
            xchg    rax, {p0}
            loop    7b
",          p0 = out(reg) _,
            p1 = out(reg) _,
            inlateout("rcx") nth => _,
            lateout("rax") result,
            options(pure, nomem, nostack),
    )};
    result
}




#[no_mangle]
pub fn mainCRTStartup() -> ! {
    // some uninitialized slices
    let nbuff: [ MaybeUninit<u8>; 128 ] = unsafe { MaybeUninit::uninit().assume_init() };
    let buffer: [ MaybeUninit<u8>; 1024 ] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut rit: DWORD = 0;
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let tbuff = nbuff.as_ptr() as *mut u8;
        for x in 1..24 {
            BaseForm(tbuff, A000931(x), 10);
            let k = wsprintfA(buffer.as_ptr() as *mut i8, "%hs, \0".as_ptr() as *const i8, tbuff);
            WriteConsoleA(handle, buffer.as_ptr() as *const VOID,
            k as DWORD, &mut rit, 0 as *mut VOID);
        };

        BaseForm(tbuff, binomial_coefficient(40,20), 10);
        let k = wsprintfA(buffer.as_ptr() as * mut i8, "%hs, \0".as_ptr() as * const i8, tbuff );
        WriteConsoleA(handle, buffer.as_ptr() as *const VOID,
        k as DWORD, &mut rit, 0 as *mut VOID);

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
