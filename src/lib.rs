#![no_std]

//! HAL for the SAM3x8e microcontroller
//!
//! This is an implementation of the [`embedded-hal`] traits for the SAM3x8e
//! microcontrollers.
//!
//! [`embedded-hal`]: https://github.com/japaric/embedded-hal
//!
//! # Requirements
//!
//! This crate requires `arm-none-eabi-gcc` to be installed and available in `$PATH` to build.
//!
//! # Usage
//!
//! To build applications (binary crates) using this crate follow the [cortex-m-quickstart]
//! instructions and add this crate as a dependency in step number 5 and make sure you enable the
//! "rt" Cargo feature of this crate.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.3
//!

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;

pub mod delay;
pub mod time;
