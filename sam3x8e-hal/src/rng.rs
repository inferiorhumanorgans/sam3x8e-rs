extern crate core;

#[cfg(feature = "unproven")]
use core::cmp;

pub use crate::pac::{trng, PMC, TRNG};

const TRNG_PASSWORD: u32 = 0x524e47;

pub struct Rng {
    rng: TRNG,
}

impl Rng {
    pub fn new(rng: TRNG, pmc: &PMC) -> Rng {
        // Enable clock 41 (TRNG)
        // Peripheral magic number Datasheet §9.1
        pmc.pmc_pcer1.write_with_zero(|w| w.pid41().set_bit());

        let mut ret = Self { rng };

        ret.enable();

        ret
    }

    pub fn enable(&mut self) {
        self.rng
            .cr
            .write_with_zero(|w| unsafe { w.enable().set_bit().key().bits(TRNG_PASSWORD) });
    }

    pub fn disable(&mut self) {
        self.rng
            .cr
            .write_with_zero(|w| unsafe { w.enable().clear_bit().key().bits(TRNG_PASSWORD) });
    }

    pub fn wait(&mut self) {
        while self.rng.isr.read().datrdy().bit_is_clear() {}
    }

    pub fn take_result(&mut self) -> u32 {
        self.rng.odata.read().bits()
    }
}

#[derive(Debug)]
pub enum Error {}

#[cfg(feature = "unproven")]
impl crate::hal::blocking::rng::Read for Rng {
    type Error = Error;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let mut pos = 0;

        while pos < buffer.len() {
            self.wait();
            let random_word: u32 = self.take_result();
            let bytes: [u8; 4] = random_word.to_ne_bytes();
            let n = cmp::min(4, buffer.len() - pos);
            buffer[pos..pos + n].copy_from_slice(&bytes[..n]);
            pos += n;
        }

        Ok(())
    }
}
