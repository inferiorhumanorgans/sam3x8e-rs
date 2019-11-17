/*
 *    This file (src/efc.rs) is part of sam3x8e-hal.
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

// TODO: Use a macro to generate this

pub use crate::pac::{efc0, EFC0, efc1, EFC1};

pub enum FlashAccessMode {
    AccessMode128,
    AccessMode64,
}

pub struct Config {
    wait_state: u8,
    interrupt_enable: bool,
    access_mode: FlashAccessMode,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            wait_state: 4,
            interrupt_enable: false,
            access_mode: FlashAccessMode::AccessMode128,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config {..Self::default()}
    }

    pub fn wait_state(mut self, ws: u8) -> Self {
        self.wait_state = ws;
        self
    }

    pub fn interrupt_enable(mut self, enabled: bool) -> Self {
        self.interrupt_enable = enabled;
        self
    }

    pub fn access_mode(mut self, mode: FlashAccessMode) -> Self {
        self.access_mode = mode;
        self
    }
}

pub struct Efc0 {
    #[allow(dead_code)]
    efc: EFC0,
}

pub trait Efc0Ext {
    /// There's no technical reason we need to freeze the config
    /// and at slower clock speeds we could probably get more aggressive
    /// with the timing.  But the flash controller needs to be initialized
    /// before the clocks
    fn freeze(self, config: Config) -> Efc0;
}

impl Efc0Ext for EFC0 {
    fn freeze(self, config: Config) -> Efc0 {
        self.fmr.write(|w| unsafe {
            let w = w.fws().bits(config.wait_state);

            let w = match config.interrupt_enable {
                true => w.frdy().set_bit(),
                false => w.frdy().clear_bit(),
            };

            match config.access_mode {
                FlashAccessMode::AccessMode64 => w.fam().set_bit(),
                FlashAccessMode::AccessMode128 => w.fam().clear_bit(),
            }
        });

        Efc0 {
            efc: self
        }
    }
}

pub struct Efc1 {
    #[allow(dead_code)]
    efc: EFC1,
}

pub trait Efc1Ext {
    fn freeze(self, config: Config) -> Efc1;
}

impl Efc1Ext for EFC1 {
    fn freeze(self, config: Config) -> Efc1 {
        self.fmr.write(|w| unsafe {
            let w = w.fws().bits(config.wait_state);

            let w = match config.interrupt_enable {
                true => w.frdy().set_bit(),
                false => w.frdy().clear_bit(),
            };

            match config.access_mode {
                FlashAccessMode::AccessMode64 => w.fam().set_bit(),
                FlashAccessMode::AccessMode128 => w.fam().clear_bit(),
            }
        });

        Efc1 {
            efc: self
        }
    }
}
