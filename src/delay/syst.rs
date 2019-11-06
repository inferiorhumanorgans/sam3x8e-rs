use super::Delay;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;

use hal::blocking::delay::{DelayMs, DelayUs};

impl Delay<SYST> {
    /// Configures the system timer (SysTick) as a delay provider
    pub fn new(mut source: SYST, clock_source: SystClkSource) -> Self {
        source.set_clock_source(clock_source);

        Delay { source }
    }

    /// Releases the system timer (SysTick) resource
    pub fn free(self) -> SYST {
        self.source
    }
}

impl DelayMs<u32> for Delay<SYST> {
    /// This is limited to 2^24 / (8 * 1000) or 2097 ms.
    /// If a longer delay is needed consider a different clock
    /// source.
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1_000);
    }
}

impl DelayMs<u16> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u8> for Delay<SYST> {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32);
    }
}

impl DelayUs<u32> for Delay<SYST> {
    fn delay_us(&mut self, us: u32) {
        let reload_value = (us * 8) - 1;

        // The register is only 24 bits wide
        assert!(reload_value < (1 << 24));

        self.source.set_reload(reload_value);
        self.source.clear_current();
        self.source.enable_counter();
        self.source.disable_interrupt();

        while !self.source.has_wrapped() {}

        self.source.disable_counter();
    }
}

impl DelayUs<u16> for Delay<SYST> {
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUs<u8> for Delay<SYST> {
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}
