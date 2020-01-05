//! Blinks with LEDs on f469-discovery board

/* STEP1:
 * Implement Led struct and Leds struct that returns all LEDs and allows iteration
 * Implement .on, .off, .toggle, .value (?) methods for Led.
 * -> use hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin}; look interesting
 *
 * STEP2:
 * Implement serial communication and make a simple terminal app that reads lines,
 * parses these lines and toggles corresponding Leds
 *
 * STEP3:
 * Try making hashlib, secp256k1 and bitcoin crates working
 *
 * FUTURE WORK:
 * display, QSPI, SDRAM, bootloader
 */
#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    // prints hello using semihosting
    hprintln!("Hello, world!").unwrap();

    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ){
        // Set up the LEDs. 
        // On the F469-Disco they are connected to pins:
        // LED1 (green):         PG6, 
        // LED2 (orange):        PD4, 
        // LED3 (red):           PD5,
        // LED4 (blue):          PK3,
        // LED7 (back, green):   PD3
        let gpiog = dp.GPIOG.split(); // LED 1
        let gpiod = dp.GPIOD.split(); // LED 2,3,7
        let gpiok = dp.GPIOK.split(); // LED 4
        let mut led1 = gpiog.pg6.into_push_pull_output();
        let mut led2 = gpiod.pd4.into_push_pull_output();
        let mut led3 = gpiod.pd5.into_push_pull_output();
        let mut led4 = gpiok.pk3.into_push_pull_output();
        let mut led7 = gpiod.pd3.into_push_pull_output();
        let mut _leds = []

        // Set up the system clock. We want to run at 180MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            // FIXME: too much of copy-paste...
            //        implement a struct with iterator
            led1.set_high().unwrap();
            led2.set_low().unwrap();
            led3.set_high().unwrap();
            led4.set_low().unwrap();
            led7.set_high().unwrap();
            delay.delay_ms(1000_u32);
            led1.set_low().unwrap();
            led2.set_high().unwrap();
            led3.set_low().unwrap();
            led4.set_high().unwrap();
            led7.set_low().unwrap();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}