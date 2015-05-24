#![feature(asm, core, intrinsics, lang_items, no_std, start)]
#![no_std]

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

extern crate core;

mod msp430g2553;
mod utils;
mod seg5641a;

use msp430g2553::{ WDTPW, WDTCTL, WDTHOLD };

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe { *WDTCTL = WDTPW | WDTHOLD; }

    seg5641a::init();

    loop {
        seg5641a::show(-123, 2);
    }
}
