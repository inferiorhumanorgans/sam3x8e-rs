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

#![no_std]
#![no_main]

extern crate panic_halt;
extern crate embedded_hal;

extern crate cortex_m_rt;

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::interrupt::Mutex;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use sam3x8e_hal::{pac::{self, SYST}, gpio::*, prelude::*, timer::Timer, pmc::{Config, PeripheralClock}};

static LED: Mutex<RefCell<Option<pioa::PA5<Output<PushPull>>>>> = Mutex::new(RefCell::new(None));
static TIMER: Mutex<RefCell<Option<Timer<SYST>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

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
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
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
