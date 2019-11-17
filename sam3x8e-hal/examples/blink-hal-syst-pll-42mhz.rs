/*
 *    This file (examples/blink-hal-syst-pll-42mhz.rs) is part of sam3x8e-hal.
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

#![deny(warnings)]
#![deny(unsafe_code)]

#![no_std]
#![no_main]

extern crate panic_halt;
extern crate embedded_hal;
extern crate cortex_m_rt;

use cortex_m_rt::entry;
use sam3x8e_hal::{pac::self, prelude::*, pmc::{MainOscillator, ProcessorClockPrescaler, DivA, MullA, Config}};

/// blink-hal-syst-pll-42mhz is a example program that will toggle PA15 roughly
/// every second using the HAL interfaces. The delay function uses the standard
/// ARM SysTick clock.
///
/// On a Macchina M2 board PA15 corresponds to one of the yellow LEDs.  The
/// pinout on an Arduino Due will vary.
///
/// This example configures the main processor to run at 1/4 the speed of the
/// 'A' PLL.  The PLL is configured to run off the xtal oscillator which is
/// almost always a 12 MHz oscillator.  The end result is a processor clock
/// speed of 42 MHz.
///
/// The SysTick counter is only 24 bits wide and at a clock speed of 84 MHz
/// the largest counter value systick can hold (2^24) will take 199 ms to
/// underflow.  At 42 MHz you can get twice that hence the loop is 4x 250 ms
/// here instead of 5x 200 ms with the 84 MHz clock.
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Flash needs to be setup before the clocks
    // let's just go with 4 to be safe even at higher clock speeds.
    p.EFC0.freeze(EfcConfig::new());
    p.EFC1.freeze(EfcConfig::new());

    let mut pmc = p.PMC.freeze(
        Config::pll(
            MainOscillator::XtalOscillator,
            DivA::Bypassed,
            MullA::Activated(13)
        )
        .prescaler(ProcessorClockPrescaler::Clk4)
    );

    let mut pioa = p.PIOA.split(&mut pmc);
    let mut yellow = pioa
        .pa15
        .into_peripheral_b(&mut pioa.absr)
        .into_push_pull_output(&mut pioa.mddr, &mut pioa.oer);

    yellow.set_high().unwrap();

    let mut delay = cp.SYST.delay(pmc.clocks);

    let mut on = true;

    loop {
        if on {
            // Pulling the pins down turns on the LED
            yellow.set_low().unwrap();
        } else {
            yellow.set_high().unwrap();
        }

        on = !on;

        for _ in 0..4 {
            delay.delay_ms(250_u32);
        }
    }
}
    