#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use heapless::Vec;

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\r\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\r\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl SerialPort {
    fn read_byte(&mut self) -> u8 {
        while self.usart1.isr.read().rxne().bit_is_clear() {}
        self.usart1.rdr.read().rdr().bits() as u8
    }

    fn write_bytes(&mut self, bytes: impl Iterator<Item = u8>) -> () {
        bytes.for_each(|byte| {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(byte as u16));
        })
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.bytes());
        Ok(())
    }
}

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();

    let mut serial = SerialPort { usart1 };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        match serial.read_byte() {
            b'\r' => {
                serial.write_bytes(buffer.iter().rev().chain(b"\r\n").cloned());
                buffer.clear();
            },
            byte => match buffer.push(byte) {
                Ok(()) => {},
                Err(lost) => {
                    uprintln!(serial, "LOST DATA: {}", lost as char);
                }
            }
        }
    }
}
