#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    rcc.ahbenr.write(|w| w.iopeen().set_bit());
    gpioe.moder.write(|w|
      w.moder8().output()
       .moder9().output()
       .moder10().output()
       .moder11().output()
       .moder12().output()
       .moder13().output()
       .moder14().output()
       .moder15().output()
    );

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
