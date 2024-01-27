#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let num_leds = leds.len();

    let unit = 50_u16;

    loop {
        for i in 0..num_leds {
            leds[(i+1) % num_leds].on().ok();
            delay.delay_ms(unit);
            leds[i].off().ok();
            delay.delay_ms(unit);
        }
    }
}
