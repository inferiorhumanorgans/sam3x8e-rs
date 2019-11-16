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
