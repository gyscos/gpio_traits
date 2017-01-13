//! This crate define generic traits for common embedded protocols.
//!
//! Those traits are meant to be used by generic device drivers,
//! and implemented by actual hardware controllers.
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod pin;
pub mod spi;
