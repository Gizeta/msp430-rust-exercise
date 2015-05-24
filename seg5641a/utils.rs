pub fn delay(n: u32) {
    let mut i = n;
    while i > 0 {
        i -= 1;
        unsafe { asm!("nop"); } // won't be optimized
    }
}
