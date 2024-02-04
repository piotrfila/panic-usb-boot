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

type ResetToUsbBootFn = unsafe extern "C" fn(u32, u32) -> !;
type RomTableLookupFn<T> = unsafe extern "C" fn(*const u16, u32) -> T;

/// The following addresses are described at `2.8.2. Bootrom Contents`
/// Pointer to the lookup table function supplied by the rom.
const ROM_TABLE_LOOKUP_PTR: *const u16 = 0x0000_0018 as _;

/// Pointer to helper functions lookup table.
const FUNC_TABLE: *const u16 = 0x0000_0014 as _;

unsafe fn rom_hword_as_ptr(rom_address: *const u16) -> *const u32 {
    let ptr: u16 = *rom_address;
    ptr as *const u32
}

#[inline(never)]
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    let rom_table_lookup_ptr: *const u32 = rom_hword_as_ptr(ROM_TABLE_LOOKUP_PTR);
    let rom_table_lookup_fn: RomTableLookupFn<ResetToUsbBootFn> = core::mem::transmute(rom_table_lookup_ptr);
    let reset_to_usb_boot = rom_table_lookup_fn(
        rom_hword_as_ptr(FUNC_TABLE) as *const u16,
        u16::from_le_bytes(*b"UB") as u32,
    );
    loop {
        reset_to_usb_boot(0_u32, DISABLE_INTEFACE_MASK);
    }
}