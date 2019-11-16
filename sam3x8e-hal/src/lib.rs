/*
 *    This file (src/lib.rs) is part of sam3x8e-hal.
 *
 *    sam3x8e-hal is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU Lesser General Public License as published
 *    by the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    sam3x8e-hal is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU Lesser General Public License for more details.
 *
 *    You should have received a copy of the GNU Lesser General Public License
 *    along with sam3x8e-hal.  If not, see <https://www.gnu.org/licenses/>.
 */

#![no_std]

//! HAL for the SAM3X / SAM3A family of microcontrollers
//!
//! This is an implementation of the [`embedded-hal`] traits for the SAM3X / SAM3A
//! family of microcontrollers.
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

#[macro_use]
extern crate sam3x8e_hal_codegen;

extern crate cortex_m;
extern crate embedded_hal as hal;
extern crate nb;
extern crate void;

#[cfg(feature = "sam3x")]
pub use sam3x8e as pac;

pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod rng;
pub mod time;
pub mod timer;
pub mod pmc;
