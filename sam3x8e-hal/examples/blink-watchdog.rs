/*
 *    This file (examples/blink-watchdog.rs) is part of sam3x8e-hal.
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

#![no_main]
#![no_std]

extern crate panic_halt;
extern crate embedded_hal;
extern crate cortex_m_rt;

use cortex_m::asm;
use cortex_m_rt::entry;
use sam3x8e_hal::{pac, pmc::Config as PmcConfig, watchdog::Config as WdtConfig, prelude::*};

#[entry]
fn main() -> ! {
    const BLINKS : usize = 10;

    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Flash needs to be setup before the clocks
    // let's just go with 4 to be safe even at higher clock speeds.
    p.EFC0.freeze(EfcConfig::new());
    p.EFC1.freeze(EfcConfig::new());

    // Configure the clock.
    let mut pmc = p.PMC.freeze(PmcConfig::hclk_84mhz());

    // Configure a delay for the blink loop
    let mut delay = cp.SYST.delay(pmc.clocks);

    // Configure the watchdog.
    let mut watchdog = p.WDT.watchdog(WdtConfig::default());

    // Start a watchdog with a 1 second timeout.
    watchdog.start(1000.ms());

    let mut pioc = p.PIOC.split(&mut pmc);
    let mut blue = pioc
        .pc25
        .into_peripheral_b(&mut pioc.absr)
        .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);

    blue.set_high().unwrap();

    let mut piod = p.PIOD.split(&mut pmc);
    let mut green = piod
        .pd8
        .into_peripheral_b(&mut piod.absr)
        .into_push_pull_output(&mut piod.mddr, &mut piod.oer);

    green.set_high().unwrap();

    let mut led_state;

    delay.delay_ms(150_u16);

    led_state = false;
    for _ in 0..(BLINKS * 2) {
        match led_state {
            true => blue.set_high().unwrap(),
            false => blue.set_low().unwrap(),
        }
        led_state = !led_state;
        watchdog.feed();
        delay.delay_ms(198_u16);
    }

    blue.set_high().unwrap();

    led_state = false;
    for _ in 0..(BLINKS * 2) {
        match led_state {
            true => green.set_high().unwrap(),
            false => green.set_low().unwrap(),
        }
        led_state = !led_state;
        watchdog.feed();
        delay.delay_ms(198_u16);
    }

    // If the watchdog fails to reset the processor
    // the green LED will stay lit
    green.set_low().unwrap();

    loop {
        asm::nop();
    }
}
