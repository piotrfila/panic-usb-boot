//! Set the panicking behavior to reset into the usb boot interface
//!
//! This crate contains an implementation of `panic_fmt` that calls the rp2040's ROM function to reset to usb boot
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_usb_boot;
//!
//! fn main() {
//!     panic!("argument is ignored");
//! }
//! ```
//!
//! # Breakable symbols
//!
//! With the panic handler being `#[inline(never)]` the symbol `rust_begin_unwind` will be
//! available to place a breakpoint on to halt when a panic is happening.

#![no_std]

#[cfg(all(feature="usb_mass_storage", feature="picoboot"))]
const DISABLE_INTEFACE_MASK: u32 = 0;
#[cfg(all(not(feature="usb_mass_storage"), feature="picoboot"))]
const DISABLE_INTEFACE_MASK: u32 = 1;
#[cfg(all(feature="usb_mass_storage", not(feature="picoboot")))]
const DISABLE_INTEFACE_MASK: u32 = 2;
#[cfg(all(not(feature="usb_mass_storage"), not(feature="picoboot")))]
const DISABLE_INTEFACE_MASK: u32 = compile_error!("No interface was selected.\nEnable at least one of the folliwing crate features:\n- picoboot\n- usb_mass_storage\n");

use core::panic::PanicInfo;
use rp2040_hal::rom_data;

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        rom_data::reset_to_usb_boot(0, DISABLE_INTEFACE_MASK);
    }
}