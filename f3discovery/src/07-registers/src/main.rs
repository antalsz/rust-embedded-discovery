#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln, ITM};

fn _fun_blink_steps(itm: &mut ITM) -> () {
    fn off_on(off_on: u8) -> u32 {
        ((!off_on) as u32) << (8+16) | (off_on as u32) << 8
    }

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x48001018;

        let mut set_leds = |pattern: u8| {
            ptr::write_volatile(GPIOE_BSRR as *mut u32, off_on(pattern));
            iprintln!(&mut itm.stim[0], "{:#010b}", pattern);
        };

        for i in 0..8 {
            set_leds(1 << i);
        }

        for pattern in 0x0_u8..=0xff_u8 {
            set_leds(pattern);
        }
    }
}

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turn on the "North" LED (red)
    gpioe.bsrr.write(|w| w.bs9().set_bit());

    // Turn on the "East" LED (green)
    gpioe.bsrr.write(|w| w.bs11().set_bit());

    // Turn off the "North" LED
    gpioe.bsrr.write(|w| w.br9().set_bit());

    // Turn off the "East" LED
    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
