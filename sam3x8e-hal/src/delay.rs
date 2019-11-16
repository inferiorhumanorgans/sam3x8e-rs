/*
 *    This file (src/delay.rs) is part of sam3x8e-hal.
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


//! Delays

use crate::pmc::Clocks;

mod syst;
mod tc;

pub use syst::*;
pub use tc::*;

pub trait DelayExt<S> {
    fn delay(self, clocks: Clocks) -> Delay<S>;
}

/// Generic timer struct.  Possible sources include System timer (SysTick) and the Timer Counter modules (TC0...TC2)
pub struct Delay<S> {
    source: S,
    clocks: Clocks,
}
