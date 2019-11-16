use syn::{Ident, parse::{Parse, ParseStream, Result}, punctuated::Punctuated, token::Comma};
use quote::ToTokens;
use proc_macro2::TokenStream;

use inflector::Inflector;

use crate::pio::Pio;

pub struct PioSet {
    pub pio: Vec<Pio>,
}

impl Parse for PioSet {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated : Punctuated<Pio, Comma> = input.parse_terminated(Pio::parse)?;

        let pio = punctuated.into_iter().collect();

        Ok( PioSet {
            pio
        })
    }    
}


impl ToTokens for PioSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let upper_names : Vec<Ident> = self.pio.iter().map(|p| format_ident!("PIO{}", p.name.to_string().to_screaming_snake_case())).collect();

        // Generate use statement
        tokens.extend(quote!(use crate::pac::{#(#upper_names),*};));

        // Generate Gpio enum
        tokens.extend(quote!(pub enum Gpio {#(#upper_names),*}));

        tokens.extend(quote!(
            pub struct PXx<MODE> {
                i: u8,
                gpio: Gpio,
                _mode: PhantomData<MODE>,
            }
        ));

        tokens.extend(quote!(
            impl<MODE> OutputPin for PXx<Output<MODE>> {
                type Error = ();
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe {
                        match self.gpio {
                            #(Gpio::#upper_names => (*#upper_names::ptr()).sodr.write_with_zero(|w| w.bits(1 << self.i))),*
                        }
                    }

                    Ok(())
                }
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe {
                        match self.gpio {
                            #(Gpio::#upper_names => (*#upper_names::ptr()).codr.write_with_zero(|w| w.bits(1 << self.i))),*
                        }
                    }

                    Ok(())
                }
            }
        ));

        tokens.extend(quote!(
            impl<MODE> InputPin for PXx<Input<MODE>> {
                type Error = ();
                fn is_high(&self) -> Result<bool, Self::Error> {
                    unimplemented!()
                }
                fn is_low(&self) -> Result<bool, Self::Error> {
                    unimplemented!()
                }
            }
        ));

        let pio = &self.pio;
        tokens.extend(quote!(
            #(#pio)*
        ));
    }
}
