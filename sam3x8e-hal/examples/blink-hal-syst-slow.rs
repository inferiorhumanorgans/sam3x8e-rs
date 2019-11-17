/*
 *    This file (examples/blink-hal-syst-slow.rs) is part of sam3x8e-hal.
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
#![no_main]

extern crate panic_halt;
extern crate embedded_hal;
extern crate cortex_m_rt;

use cortex_m_rt::entry;
use sam3x8e_hal::{pac::self, prelude::*, pmc::Config};

/// blink-hal-syst-slow is a example program that will toggle PA15 roughly every
/// second using the HAL interfaces. The delay function uses the standard ARM
/// SysTick clock as a counter.
///
/// On a Macchina M2 board PA15 corresponds to one of the yellow LEDs.  The
/// pinout on an Arduino Due will vary.
///
/// This example configures the main processor to run at the speed of the slow
/// clock.  The end result is a processor clock speed of 32 KHz.  This is
/// too slow for microsecond precision.
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Flash needs to be setup before the clocks
    // let's just go with 4 to be safe even at higher clock speeds.
    p.EFC0.freeze(EfcConfig::new());
    p.EFC1.freeze(EfcConfig::new());

    let mut pmc = p.PMC.freeze(Config::slow_clock());

    let mut piod = p.PIOD.split(&mut pmc);
    let mut green = piod
        .pd8
        .into_peripheral_b(&mut piod.absr)
        .into_push_pull_output(&mut piod.mddr, &mut piod.oer);

    green.set_high().unwrap();
    
    let mut delay = cp.SYST.delay(pmc.clocks);
    let mut on = true;
    
    loop {
        if on {
            // Pulling the pins down turns on the LED
            green.set_low().unwrap();
        } else {
            green.set_high().unwrap();
        }

        on = !on;

        // As the slow clock runs at 32 KHz we could actually
        // just program a 1 second delay, but for ease of comparison
        // with the PLL clock example, we'll do this.
        for _ in 0..5 {
            delay.delay_ms(198_u32);
        }
    }
}
