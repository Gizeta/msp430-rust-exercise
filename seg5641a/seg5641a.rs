#![allow(dead_code)]

use msp430g2553::{ P1OUT, P1DIR, P2OUT, P2DIR };
use msp430g2553::{ BIT0, BIT1, BIT2, BIT3, BIT6, BIT7 };
use utils;

const SEG_DIGITAL0: u8 = 0x3F;
const SEG_DIGITAL1: u8 = 0x06;
const SEG_DIGITAL2: u8 = 0x5B;
const SEG_DIGITAL3: u8 = 0x4F;
const SEG_DIGITAL4: u8 = 0x66;
const SEG_DIGITAL5: u8 = 0x6D;
const SEG_DIGITAL6: u8 = 0x7D;
const SEG_DIGITAL7: u8 = 0x07;
const SEG_DIGITAL8: u8 = 0x7F;
const SEG_DIGITAL9: u8 = 0x6F;
const SEG_DIGITALA: u8 = 0x77;
const SEG_DIGITALB: u8 = 0x7C;
const SEG_DIGITALC: u8 = 0x39;
const SEG_DIGITALD: u8 = 0x5E;
const SEG_DIGITALE: u8 = 0x79;
const SEG_DIGITALF: u8 = 0x71;
const SEG_POINT: u8 = BIT7 as u8;
const SEG_MINUS: u8 = BIT6 as u8;

pub fn init() {
    unsafe {
        *P1DIR = 0xFF;
        *P2DIR = 0x0F;
        *P1OUT = 0x00;
        *P2OUT = 0x0F;
    }
}

pub fn test() {
    unsafe {
        *P1OUT = 0xFF;
        *P2OUT = 0x00;
    }
}

pub fn show(num: i16, point: i16) {
    if num < -999 || num > 9999 { return; }

    if num >= 0 {
        let mut n: u16 = num as u16;

        for i in 1..5 {
            let mut dig = get_digital_num(n % 10);
            if i == point {
                dig |= SEG_POINT;
            }
            show_digital(dig, i);
            utils::delay(100);
            n = div10(n);
        }
    }
    else {
        let mut n: u16 = (-num) as u16;

        show_digital(SEG_MINUS, 4);
        utils::delay(100);

        for i in 1..4 {
            let mut dig = get_digital_num(n % 10);
            if i == point {
                dig |= SEG_POINT;
            }
            show_digital(dig, i);
            utils::delay(100);
            n = div10(n);

        }
    }
}

fn show_digital(num: u8, dig: i16) {
    match dig {
        1 => unsafe { *P1OUT = num; *P2OUT = !(BIT3 as u8); },
        2 => unsafe { *P1OUT = num; *P2OUT = !(BIT2 as u8); },
        3 => unsafe { *P1OUT = num; *P2OUT = !(BIT1 as u8); },
        4 => unsafe { *P1OUT = num; *P2OUT = !(BIT0 as u8); },
        _ => {},
    };
}

fn get_digital_num(num: u16) -> u8 {
    match num {
        0  => SEG_DIGITAL0,
        1  => SEG_DIGITAL1,
        2  => SEG_DIGITAL2,
        3  => SEG_DIGITAL3,
        4  => SEG_DIGITAL4,
        5  => SEG_DIGITAL5,
        6  => SEG_DIGITAL6,
        7  => SEG_DIGITAL7,
        8  => SEG_DIGITAL8,
        9  => SEG_DIGITAL9,
        10 => SEG_DIGITALA,
        11 => SEG_DIGITALB,
        12 => SEG_DIGITALC,
        13 => SEG_DIGITALD,
        14 => SEG_DIGITALE,
        15 => SEG_DIGITALF,
        _  => 0x00,
    }
}

fn div10(n: u16) -> u16 {
    let mut q = (n >> 1) + (n >> 2);
    q = q + (q >> 4);
    q = q + (q >> 8);
    q = q >> 3;
    let r = n - q * 10;
    return q + ((r + 6) >> 4);
}
