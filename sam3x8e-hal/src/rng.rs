/*
 *    This file (src/rng.rs) is part of sam3x8e-hal.
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

 extern crate core;

#[cfg(feature = "unproven")]
use core::cmp;

pub use crate::pac::{trng, PMC, TRNG};

// TODO: Edit the SVD to include this constant
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
