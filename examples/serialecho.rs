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

        let _byte = block!(rx.read()).unwrap();
        for c in b"The quick brown fox jumps over the lazy dog.\r\n".iter() {
            block!(tx.write(*c)).ok();
        }

        loop {
            let byte = block!(rx.read()).unwrap();
            block!(tx.write(byte)).ok();
        }
    }

    loop {}
}