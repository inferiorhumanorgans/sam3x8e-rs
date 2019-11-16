/*
 *    This file (examples/blink-hal-tc.rs) is part of sam3x8e-hal.
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

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate embedded_hal;
extern crate cortex_m_rt;
use cortex_m_rt::entry;

use sam3x8e_hal::{pac, prelude::*, pmc::Config, pmc::PeripheralClock};

fn flash_init(p: &sam3x8e::Peripherals) {
    // TODO: Set FWS (flash wait state) according to clock configuration
    {
        let efc = &p.EFC0;
        let fmr = &efc.fmr;
        fmr.write(|w| unsafe { w.fws().bits(4) });
    }

    {
        let efc = &p.EFC1;
        let fmr = &efc.fmr;
        fmr.write(|w| unsafe { w.fws().bits(4) });
    }
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    flash_init(&p);

    let mut pmc = p.PMC.freeze(Config::hclk_84mhz());

    // Enable TC Timer 4 clock
    pmc.enable_clock(PeripheralClock::Tc4);

    let mut delay = Delay::<TimerCounter4>::new( TimerCounter4(p.TC1), pmc.clocks );

    let mut pioc = p.PIOC.split(&mut pmc);
    let mut blue = pioc
        .pc25
        .into_peripheral_b(&mut pioc.absr)
        .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);

    blue.set_high().unwrap();

    // Just for fun turn on the green LED so we alternate green/blue

    let mut piod = p.PIOD.split(&mut pmc);
    let mut green = piod
        .pd8
        .into_peripheral_b(&mut piod.absr)
        .into_push_pull_output(&mut piod.mddr, &mut piod.oer);

    green.set_low().unwrap();

    let mut on = true;

    loop {
        if on {
            // Pulling the pins down turns on the LED
            blue.set_low().unwrap();
        } else {
            blue.set_high().unwrap();
        }

        on = !on;
        delay.delay_ms(2500_u32);
    }
}
