#![no_main]
#![no_std]

extern crate panic_msp430; // For now, we only have an infinitely-looping panic handler.

use msp430::asm;
use msp430_rt::entry;

#[allow(unused)]
use msp430f5529;

#[entry]
fn main() -> ! {
    loop {
        // Application begins here.
        asm::nop();
    }
}
