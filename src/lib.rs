//! This crate define generic traits for common embedded protocols.
//!
//! Those traits are meant to be used by generic device drivers,
//! and implemented by actual hardware controllers.
#![no_std]

#[cfg(feature = "debug")]
#[macro_use]
extern crate std;

pub mod bits;
pub mod pin;

pub mod spi;

// pub mod i2c;
// pub mod w1;
