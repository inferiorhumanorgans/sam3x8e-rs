#![feature(proc_macro_span)]
#![feature(proc_macro_diagnostic)]

#[macro_use] extern crate syn;
#[macro_use] extern crate quote;

extern crate proc_macro;
extern crate inflector;


mod pio;

use pio::PioSet;

#[proc_macro]
pub fn gpio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let pio = parse_macro_input!(input as PioSet);

    quote!(
        #pio
    ).into()
}
