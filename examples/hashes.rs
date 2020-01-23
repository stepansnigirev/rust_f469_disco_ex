//! Test of bitcoin_hashes on the board.
//! Prints sha256 of a fixed sentence to serial port

#![no_main]
#![no_std]

#![allow(unused_extern_crates)]
extern crate panic_halt;

// hardware specific
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use nb::block;
use core::fmt::Write;
use crate::hal::{prelude::*, stm32, serial::Serial, serial::config::Config};

// bitcoin stuff
use bitcoin_hashes::{Hash, sha256};

// print macros
macro_rules! uprint {
    ($tx:expr, $($arg:tt)*) => {
        $tx.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($tx:expr, $fmt:expr) => {
        uprint!($tx, concat!($fmt, "\r\n"))
    };
    ($tx:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($tx, concat!($fmt, "\r\n"), $($arg)*)
    };
}

#[entry]
fn main() -> ! {

    if let Some(dp) = stm32::Peripherals::take(){

        let gpiob = dp.GPIOB.split(); // UART3

        // Set up the system clock. We want to run at 180MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        let tx = gpiob.pb10.into_alternate_af7();
        let rx = gpiob.pb11.into_alternate_af7();
    
        let serial = Serial::usart3(
            dp.USART3, 
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            clocks,
        ).unwrap();

        let (mut tx, mut rx) = serial.split();

        /* Here where the magic starts. */

        // wait for the first character
        uprintln!(tx, "\r\nWelcome stranger! Press any key to continue.");
        let _byte = block!(rx.read()).unwrap();

        let input = "The quick brown fox jumps over the lazy dog.";
        uprintln!(tx, "\r\nHere we hash: \"{}\"", input);

        let hash = sha256::Hash::hash(&input.as_bytes());
        uprintln!(tx, "Hash: {:x?}", hash);

        // infinite echo loop
        loop {
            let byte = block!(rx.read()).unwrap();
            block!(tx.write(byte)).ok();
        }
    }

    loop {}
}