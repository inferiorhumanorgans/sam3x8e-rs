/*
 *    This file (src/pio.rs) is part of sam3x8e-hal-codegen.
 *
 *    sam3x8e-hal-codegen is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU Lesser General Public License as published
 *    by the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    sam3x8e-hal-codegen is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU Lesser General Public License for more details.
 *
 *    You should have received a copy of the GNU Lesser General Public License
 *    along with sam3x8e-hal-codegen.  If not, see <https://www.gnu.org/licenses/>.
 */

mod pin;
mod pio;
mod pio_set;

pub use pin::*;
pub use pio::*;
pub use pio_set::*;
