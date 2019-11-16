/*
 *    This file (src/gpio.rs) is part of sam3x8e-hal.
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

#[cfg(feature = "unproven")]
use crate::hal::digital::v2::InputPin;
use crate::hal::digital::v2::OutputPin;

use core::marker::PhantomData;

use crate::pmc::Pmc;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, pmc: &mut Pmc) -> Self::Parts;
}

/// Input mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input
pub struct Floating;
/// Pulled down input
pub struct PullDown;
/// Pulled up input
pub struct PullUp;

/// Output mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output
pub struct PushPull;
/// Open drain output
pub struct OpenDrain;

pub struct PeripheralA;
pub struct PeripheralB;

// SAM3X PIO config
#[cfg(feature = "sam3x")]
gpio! {
    A: {
        // Peripheral magic number Datasheet §9.1
        id: 11,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
    },
    B: {
        id: 12,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    },
    C: {
        id: 13,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
    },
    D: {
        id: 14,
        // The datasheet appears to indicate that PIOD only has 10 pins on the SAM3x models, but
        // the Macchina M2 definitely has at least 11 pins.
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    },
}

// SAM3A PIO config
#[cfg(feature = "sam3a")]
gpio! {
    A: {
        id: 11,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
    },
    B: {
        id: 12,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    },
    C: {
        id: 13,
        pins: [0],
    },
}

// SAM3X8H PIO config
#[cfg(feature = "sam3x8h")]
gpio! {
    A: {
        id: 11,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    },
    B: {
        id: 12,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    },
    C: {
        id: 13,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
    },
    D: {
        id: 14,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
    },
    E: {
        id: 15,
        pins: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
    },
    F: {
        id: 16,
        pins: [0, 1, 2, 3, 4, 5, 6],
    },
}
