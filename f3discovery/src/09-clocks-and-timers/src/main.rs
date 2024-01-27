#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Clock frequency is 8 MHz / (psc + 1)
    // Goal: Clock frequency is 1/ms = 1/(10⁻³ Hz) = 1 kHz
    // 1 kHz = 8 MHz / (psc + 1)
    // (psc + 1) = 8 MHz / 1 kHz = 8×10⁶ Hz / 1×10³ Hz = 8×10³ = 8,000
    // psc = 7,999

    // The one thing I missed is that you should disable the counter first in
    // case somebody else left it running.
    tim6.cr1.write(|w| w.opm().set_bit().cen().clear_bit());
    tim6.psc.write(|w| w.psc().bits(7_999));
    tim6.arr.write(|w| w.arr().bits(ms));
    tim6.cr1.write(|w| w.cen().set_bit());
    while tim6.sr.read().uif().is_clear() { }
    tim6.sr.write(|w| w.uif().clear_bit());
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    rcc.apb1enr.write(|w| w.tim6en().set_bit());

    let ms = 1000;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on().unwrap();
            delay(tim6, ms);
            leds[curr].off().unwrap();
            delay(tim6, ms);
        }
    }
}
