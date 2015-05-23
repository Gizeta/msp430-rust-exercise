#![feature(asm, core, intrinsics, lang_items, no_std, start)]
#![no_std]

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

extern crate core;

macro_rules! port {
    ($name:ident = $value:expr) => {
        const $name: *mut u16 = $value as *mut u16;
    }
}

macro_rules! val {
    ($name:ident = $value:expr) => {
        const $name: u16 = $value as u16;
    }
}

port!(WDTCTL = 0x0120);
val!(WDTPW = 0x5A00);
val!(WDTHOLD = 0x0080);
port!(P1OUT = 0x0021);
port!(P1DIR = 0x0022);
val!(BIT0 = 0x0001);

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        *WDTCTL = WDTPW | WDTHOLD;
        *P1DIR |= BIT0;
        *P1OUT |= BIT0;
    }
    loop {
        unsafe {
            *P1OUT ^= BIT0;
        }

        let mut i = 50000;
        while i > 0 {
            i -= 1;
            unsafe { asm!("nop"); } // won't be optimized
        }
    }
}
