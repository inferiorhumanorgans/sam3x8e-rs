/*
 *    This file (src/watchdog.rs) is part of sam3x8e-hal.
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

use crate::hal::watchdog;
use crate::pac::WDT;
use crate::time::Hertz;

pub enum WatchdogResetMode {
    ResetAll,
    ResetProcessorOnly,
}

pub struct Config {
    /// 12 bits max
    counter_value: u16,

    /// WDFIEN: Watchdog Fault Interrupt Enable
    /// A Watchdog fault is an underflow or error
    /// false: A Watchdog fault has no effect on interrupt.
    /// true: A Watchdog fault asserts interrupt.
    interrupt_enabled: bool,

    /// WDD: Watchdog Delta Value
    /// Defines the permitted range for reloading the Watchdog Timer
    /// If the Watchdog Timer value is less than or equal to WDD, writing WDT_CR
    /// with WDRSTT = 1 restarts the timer. If the Watchdog Timer value is
    /// greater than WDD, writing WDT_CR with WDRSTT = 1 causes a Watchdog error.

    /// WDRSTEN: Watchdog Reset Enable
    /// false: A Watchdog fault (underflow or error) has no effect on the resets.
    /// true: A Watchdog fault (underflow or error) triggers a Watchdog reset.
    reset_on_fault: bool,

    /// WDRPROC: Watchdog Reset Processor:
    /// false: If WDRSTEN is 0, a Watchdog fault (underflow or error) activates all resets.
    /// true: If WDRSTEN is 1, a Watchdog fault (underflow or error) activates the processor reset.
    reset_mode: WatchdogResetMode,

    /// WDDBGHLT: Watchdog Debug Halt
    /// false: The Watchdog runs when the processor is in debug state.
    /// true: The Watchdog stops when the processor is in debug state.
    run_on_debug: bool,

    /// WDIDLEHLT: Watchdog Idle Halt
    /// false: The Watchdog runs when the system is in idle mode.
    /// true: The Watchdog stops when the system is in idle state.
    run_on_idle: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            counter_value: 0b111111111111,
            interrupt_enabled: false,
            reset_on_fault: true,
            reset_mode: WatchdogResetMode::ResetProcessorOnly,
            run_on_debug: false,
            run_on_idle: false,
        }
    }
}

pub struct Watchdog {
    wdt: WDT,
    config: Config,
}

impl Watchdog {
    pub fn commit_config(&mut self) {
        let Self { config, .. } = self;

        let timeout = config.counter_value;

        self.wdt.mr.write(|w| unsafe {
            let mut w = w
                .wdv().bits(timeout)
                .wddis().clear_bit();

            w = match config.interrupt_enabled {
                true => w.wdfien().set_bit(),
                false => w.wdfien().clear_bit(),
            };

            w = match config.reset_on_fault {
                true => w.wdrsten().set_bit(),
                false => w.wdrsten().clear_bit(),
            };

            w = match config.reset_mode {
                WatchdogResetMode::ResetAll => w.wdrproc().clear_bit(),
                WatchdogResetMode::ResetProcessorOnly => w.wdrproc().set_bit(),
            };

            w = match config.run_on_debug {
                true => w.wddbghlt().clear_bit(),
                false => w.wddbghlt().set_bit(),
            };

            match config.run_on_idle {
                true => w.wdidlehlt().clear_bit(),
                false => w.wdidlehlt().set_bit(),
            }
        } );
    }

}

impl watchdog::Watchdog for Watchdog {
    fn feed(&mut self) {
        self.wdt.cr.write_with_zero(|w|
            w.key().passwd()
            .wdrstt().set_bit()
        );
    }
}

impl watchdog::WatchdogEnable for Watchdog {
    type Time = Hertz;

    /// On the SAM3x/SAM3a this can only be called once so
    /// start will actually commit the configuration.  The processor must be
    /// reset before the watchdog timer can be reconfigured.
    fn start<T>(&mut self, period: T)
    where
        T: Into<Hertz>,
    {
        // § 15.4
        // The Watchdog is built around a 12-bit down counter, which is loaded
        // with the value defined in the field WDV of the Mode Register (WDT_MR).
        // The Watchdog Timer uses the Slow Clock divided by 128 to establish
        // the maximum Watchdog period to be 16 seconds (with a typical Slow
        // Clock of 32,768 Hz).

        const WDT_CLOCK: u32 = 32_768_u32 / 128;

        let freq = period.into().0;
        let timeout = WDT_CLOCK / freq;

        assert!(timeout < (1 << 12));

        self.config.counter_value = timeout as u16;

        self.commit_config();
    }
}

pub trait WatchdogExt {
    fn watchdog(self, config: Config) -> Watchdog;
}

impl WatchdogExt for WDT {
    fn watchdog(self, config: Config) -> Watchdog {
        Watchdog { wdt: self, config }
    }
}
