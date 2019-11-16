/*
 *    This file (src/timer.rs) is part of sam3x8e-hal.
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

use crate::time::Hertz;

mod syst;

pub use syst::*;

pub trait TimerExt<TIM> {
    fn timer<T>(self, timeout: T) -> Timer<TIM>
    where
        T: Into<Hertz>;
}

/// Hardware timers
pub struct Timer<TIM> {
    tim: TIM,
}
