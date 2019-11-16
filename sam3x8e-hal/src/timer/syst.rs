/*
 *    This file (src/timer/syst.rs) is part of sam3x8e-hal.
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

use crate::hal::timer::{CountDown, Periodic};
use crate::time::Hertz;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use void::Void;
use crate::prelude::*;

use super::{Timer, TimerExt};

impl Timer<SYST> {
    /// Configures the SYST clock as a periodic count down timer
    pub fn syst<T>(mut syst: SYST, timeout: T) -> Self
    where
        T: Into<Hertz>,
    {
        syst.set_clock_source(SystClkSource::Core);
        let mut timer = Timer { tim: syst };
        timer.start(timeout);
        timer
    }

    /// Starts listening
    pub fn listen(&mut self) {
        self.tim.enable_interrupt()
    }

    /// Stops listening
    pub fn unlisten(&mut self) {
        self.tim.disable_interrupt()
    }
}

impl CountDown for Timer<SYST> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let clock_speed : Hertz = 84.mhz().into();
        let reload_value = (clock_speed.0 / timeout.into().0) - 1;
        assert!(reload_value < (1 << 24));

        self.tim.set_reload(reload_value);
        self.tim.clear_current();
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl TimerExt<SYST> for SYST {
    fn timer<T>(self, timeout: T) -> Timer<SYST>
    where
        T: Into<Hertz>,
    {
        Timer::syst(self, timeout)
    }
}

impl Periodic for Timer<SYST> {}
