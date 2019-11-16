/*
 *    This file (examples/blink-hal-syst-pll-84mhz.rs) is part of sam3x8e-hal.
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

/// blink-hal-syst-pll-84mhz is a example program that will toggle PA15 roughly
/// every second using the HAL interfaces. The timer is based off of the
/// standard ARM SysTick clock.
///
/// On a Macchina M2 board PA15 corresponds to one of the yellow LEDs.  The
/// pinout on an Arduino Due will vary.
///
/// This example configures the main processor to run at half the speed of the
/// 'A' PLL.  The PLL is configred to run off the xtal oscillator which is
/// almost always a 12 MHz oscillator.  The end result is a processor clock
/// speed of 84 MHz.
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    flash_init(&p);

    let mut pmc = p.PMC.freeze(Config::hclk_84mhz());

    let mut pioc = p.PIOC.split(&mut pmc);
    let mut blue = pioc
        .pc25
        .into_peripheral_b(&mut pioc.absr)
        .into_push_pull_output(&mut pioc.mddr, &mut pioc.oer);

    blue.set_high().unwrap();

    let mut delay = cp.SYST.delay(pmc.clocks);

    let mut on = true;

    loop {
        if on {
            // Pulling the pins down turns on the LED
            blue.set_low().unwrap();
        } else {
            blue.set_high().unwrap();
        }

        on = !on;
        for _ in 0..5 {
            delay.delay_ms(198_u32);
        }
    }
}
