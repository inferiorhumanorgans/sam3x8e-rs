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
