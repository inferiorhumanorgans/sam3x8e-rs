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
