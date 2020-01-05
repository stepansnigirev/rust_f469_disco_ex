//! Serial interface echo server
//!
//! In this example every received byte will be sent back to the sender. You can test this example
//! with serial terminal emulator like `minicom`.

/* TODO:
 * Implement serial communication and make a simple terminal app that reads lines,
 * parses these lines and toggles corresponding Leds
 * Implement uprintln! macro and figure out how to use fmt::stuff
 */

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use nb::block;

use crate::hal::{prelude::*, stm32, serial::Serial, serial::config::Config};

use bitcoin_hashes::{Hash, sha256};

const CHARS: &[u8] = b"0123456789abcdef";

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
        )
        .unwrap();

        let (mut tx, mut rx) = serial.split();

        // wait for the first character
        let _byte = block!(rx.read()).unwrap();
        let input = "The quick brown fox jumps over the lazy dog.\r\n";
        for c in input.as_bytes().iter() {
            block!(tx.write(*c)).ok();
        }
        let hash = sha256::Hash::hash(&input.as_bytes());
        // write!(tx, "{:x?}", hash);
        // let s = format!("{:x?}", hash).unwrap();
        for b in hash.iter() {
            let c = CHARS[(*b >> 4) as usize];
            block!(tx.write(c)).ok();
            let c = CHARS[(*b & 0xf) as usize];
            block!(tx.write(c)).ok();
        }
        block!(tx.write(b'\r')).ok();
        block!(tx.write(b'\n')).ok();

        loop {
            let byte = block!(rx.read()).unwrap();
            block!(tx.write(byte)).ok();
        }
    }

    loop {}
}