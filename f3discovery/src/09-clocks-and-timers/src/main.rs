#![no_main]
#![no_std]

use aux9::{entry, switch_hal::OutputSwitch, tim6};

#[inline(never)]
fn delay(tim6: &tim6::RegisterBlock, ms: u16) {
    // Processor is 72MHz + most instructions are 1 tick
    // Guess: loop is 2 instructions
    // Thus, computation:
    //     1 instr * 72 MHz = 72×10⁶ instr/sec
    //     72×10⁶ instr/sec / 2 instr/loop = 72×10⁶/2 instr/loop
    // Or, in other words: running at 72MHz means 72M instructions for 1 second,
    // so obviously that means half that many loops if a loop is 2 instructions.
    // Duh.
    
    for _ in 0..103*ms {}

    // Ok, I was very wrong.
    //
    //     X instr/loop = 72×10⁶ instr/sec / 103×10³ loop/sec
    //                  ≈ 699 instr/loop
    //
    // Really‽

    // Of course, if I compile this in release mode, there will be no delay :-)
}

#[entry]
fn main() -> ! {
    let (leds, rcc, tim6) = aux9::init();
    let mut leds = leds.into_array();

    // TODO initialize TIM6

    let ms = 50;
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
