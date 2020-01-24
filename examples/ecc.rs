//! Test of secp256k1 library on the board

#![no_main]
#![no_std]

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
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
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use core::slice;

// USE_ECMULT_STATIC_PRECOMPUTATION would help to reduce RAM usage...
static mut SECP256K1_BUF: [u64; 100] = [0u64; 100];
// static mut SECP256K1_BUF: [u64; 70000/8] = [0u64; 70000/8];

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

        // wait for the first character
        uprintln!(tx, "\r\nWelcome stranger! Press any key to continue.");
        let _byte = block!(rx.read()).unwrap();

        /* Here where the magic starts. */

        let bufsize = unsafe { core::mem::size_of_val(&SECP256K1_BUF) };
        uprintln!(tx, "testing {}(sign {} + verify {}) / {}",
                 Secp256k1::preallocate_size(),
                 Secp256k1::preallocate_signing_size(),
                 Secp256k1::preallocate_verification_size(),
                 bufsize);
        assert!(Secp256k1::preallocate_size() < bufsize);

        let buf = unsafe { slice::from_raw_parts_mut(SECP256K1_BUF.as_mut_ptr() as *mut u8, bufsize) };
        uprintln!(tx, "secp initializing");
        let secp = Secp256k1::preallocated_new(buf).unwrap();
        uprintln!(tx, "secp initialized {:?}", secp);

        let input = "The quick brown fox jumps over the lazy dog.";
        uprintln!(tx, "\r\nHere we hash: \"{}\"", input);

        // generate a random-ish 32-byte sequence
        let secret = sha256::Hash::hash(&input.as_bytes());
        uprintln!(tx, "Our pass-based secret: {:x?}", secret);

        // convert to secret key
        let secret_key = SecretKey::from_slice(&secret).expect("32 bytes, within curve order");
        uprintln!(tx, "created secret key {:?}", secret_key);

        // calculate public key
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        uprintln!(tx, "created public key {:?}", public_key);

        // get some message to sign
        let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");
        uprintln!(tx, "created message {:?}", message);

        // sign
        let sig = secp.sign(&message, &secret_key);
        uprintln!(tx, "created signature {:?}", sig);

        // verify
        let result = secp.verify(&message, &sig, &public_key);
        uprintln!(tx, "verified signature {:?}", result);

        // infinite echo loop
        loop {
            let byte = block!(rx.read()).unwrap();
            block!(tx.write(byte)).ok();
        }
    }

    loop {}
}