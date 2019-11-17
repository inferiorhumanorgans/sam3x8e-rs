/*
 *    This file (examples/timer.rs) is part of sam3x8e-hal.
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

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::interrupt::Mutex;
use cortex_m::{asm, peripheral::SYST};
use cortex_m_rt::{entry, exception};
use sam3x8e_hal::{pac, gpio::*, prelude::*, timer::Timer, pmc::Config};

static LED: Mutex<RefCell<Option<pioa::PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<Timer<SYST>>>> = Mutex::new(RefCell::new(None));

/// timer is a example program that will toggle PA5 roughly every second using
/// the HAL interfaces. The timer in this case uses the standard ARM SysTick clock.
///
/// On a Macchina M2 board PA5 corresponds to one of the yellow LEDs.  The
/// pinout on an Arduino Due will vary.
///
/// This example configures the main processor to run at half the speed of the
/// 'A' PLL.  The PLL is configured to run off the xtal oscillator which is
/// almost always a 12 MHz oscillator.  The end result is a processor clock
/// speed of 84 MHz.
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Flash needs to be setup before the clocks
    // let's just go with 4 to be safe even at higher clock speeds.
    p.EFC0.freeze(EfcConfig::new());
    p.EFC1.freeze(EfcConfig::new());

    let mut pmc = p.PMC.freeze(Config::hclk_84mhz());

    let mut pioa = p.PIOA.split(&mut pmc);
    let mut yellow = pioa
        .pa5
        .into_peripheral_b(&mut pioa.absr)
        .into_push_pull_output(&mut pioa.mddr, &mut pioa.oer);

    yellow.set_high().unwrap();

    // Configure the timer.
    let mut timer = cp.SYST.timer(6.hz());
    timer.listen();

    // Store the LED and timer in mutex refcells to make them available from the
    // timer interrupt.
    cortex_m::interrupt::free(|cs| {
        *LED.borrow(cs).borrow_mut() = Some(yellow);
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });

    loop {
        asm::nop();
    }
}

#[exception]
fn SysTick() {
    // Keep a state to blink the LED.
    static mut STATE: bool = false;

    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut _timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
            // Change the LED state on each exception.
            if let Some(ref mut led) = LED.borrow(cs).borrow_mut().deref_mut() {
                if *STATE {
                    led.set_high().unwrap();
                    *STATE = false;
                } else {
                    led.set_low().unwrap();
                    *STATE = true;
                }
            }
        }
    });
}
