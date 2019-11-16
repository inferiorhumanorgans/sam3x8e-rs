/*
 *    This file (src/lib.rs) is part of sam3x8e-hal-codegen.
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

#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

extern crate inflector;
extern crate proc_macro;

mod pio;

use pio::PioSet;

#[proc_macro]
pub fn gpio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pio = parse_macro_input!(input as PioSet);

    quote!(
        #pio
    )
    .into()
}
