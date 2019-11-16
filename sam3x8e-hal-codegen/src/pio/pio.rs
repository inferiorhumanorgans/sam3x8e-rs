/*
 *    This file (src/pio/pio.rs) is part of sam3x8e-hal-codegen.
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

use syn::{LitInt, Ident, parse::{Parse, ParseStream, Result}, punctuated::Punctuated, token::Comma};
use quote::ToTokens;
use proc_macro2::TokenStream;

use inflector::Inflector;

use crate::pio::Pin;

mod kw {
    syn::custom_keyword!(pins);
    syn::custom_keyword!(id);
}

pub struct Pio {
    pub name: Ident,
    pub id: u8,
    pub pins: Vec<u8>,
}

impl Parse for Pio {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;

        let value;
        braced!(value in input);

        let mut id = None;
        let mut pins = None;

        loop {
            if value.is_empty() {
                break
            }

            let lookahead = value.lookahead1();

            if lookahead.peek(kw::pins) {
                value.parse::<kw::pins>()?;
                value.parse::<Token![:]>()?;

                let inner;
                bracketed!(inner in value);

                let punctuated : Punctuated<LitInt, Comma> = inner.parse_terminated(LitInt::parse)?;

                pins = Some(
                    punctuated
                        .into_iter()
                        .map(|elem|
                            elem.base10_parse::<u8>().unwrap()
                        )
                        .collect()
                );
            } else if lookahead.peek(kw::id) {
                value.parse::<kw::id>()?;
                value.parse::<Token![:]>()?;
                let literal = value.parse::<LitInt>()?;
                id = Some(literal.base10_parse()?);
            } else {
                return Err(lookahead.error())
            }

            if value.is_empty() {
                break
            }

            value.parse::<Token![,]>()?;
        }

        let id = id.unwrap();
        let pins = pins.unwrap();

        Ok(Pio {
            name,
            id,
            pins,
        })
    }
}

impl<'a> ToTokens for Pio {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let index = &self.name;
        let upper_ident = index.to_string().to_screaming_snake_case();
        let lower_ident = index.to_string().to_snake_case();

        let lower_name = format_ident!("pio{}", lower_ident);
        let upper_name = format_ident!("PIO{}", upper_ident);
        let pio_partial_erase = format_ident!("P{}x", upper_ident);

        let pins = self.pins.iter().map(|pin| Pin { pio: &index, index: *pin});
        let upper_pin_idents : Vec<Ident> = self.pins.iter().map(|pin| format_ident!("P{}{}", upper_ident, pin)).collect();
        let lower_pin_idents : Vec<Ident> = self.pins.iter().map(|pin| format_ident!("p{}{}", lower_ident, pin)).collect();

        // Peripheral magic number Datasheet §9.1
        let pio_clock = format_ident!("pid{}", self.id);

        tokens.extend(quote!(
            pub mod #lower_name {
                use super::{
                    Floating, Gpio, GpioExt, Input, OpenDrain, Output, PXx, PullDown, PullUp, PushPull, PeripheralA,
                    PeripheralB,
                };

                #[cfg(feature = "unproven")]
                use crate::hal::digital::v2::InputPin;
                use crate::hal::digital::v2::OutputPin;

                use crate::pac::{#lower_name, #upper_name, PMC};
                use crate::pmc::Pmc;
                use core::marker::PhantomData;

                pub struct Parts {
                    pub absr: ABSR,
                    pub mddr: MDDR,
                    pub mder: MDER,
                    pub oer: OER,
                    #(pub #lower_pin_idents: #upper_pin_idents<Input<Floating>>),*
                }

                impl GpioExt for #upper_name {
                    type Parts = Parts;
                    fn split(self, pmc: &mut Pmc) -> Parts {
                        // Unlock everything
                        self.wpmr.write(|w| unsafe { w.wpen().clear_bit().wpkey().bits(0x50494F) });

                        // PER = PIO Enable Register - enable all pins
                        self.per.write_with_zero(|w| unsafe { w.bits(0xFFFFFFFF) });

                        pmc.pmc.pmc_wpmr.write(|w|
                            w
                            .wpen().clear_bit()
                            .wpkey().passwd()
                        );

                        pmc.pmc.pmc_pcer0.write_with_zero(|w| w.#pio_clock().set_bit());

                        Parts {
                            absr: ABSR { _ownership: () },
                            mddr: MDDR { _ownership: () },
                            mder: MDER { _ownership: () },
                            oer: OER { _ownership: () },
                            #(#lower_pin_idents: #upper_pin_idents { _mode: PhantomData }),*
                        }
                    }
                }

                /// Opaque ABSR register. Datasheet §31.5.3
                /// 
                /// The PIO Controller provides multiplexing of up to two
                /// peripheral functions on a single pin. The selection is
                /// performed by writing PIO_ABSR (AB Select Register). For each
                /// pin, the corresponding bit at level 0 means peripheral A is
                /// selected whereas the corresponding bit at level 1 indicates
                /// that peripheral B is selected.
                /// 
                /// Note that multiplexing of peripheral lines A and B only
                /// affects the output line. The peripheral input lines are
                /// always connected to the pin input.
                /// 
                /// After reset, PIO_ABSR is 0, thus indicating that all the PIO
                /// lines are configured on peripheral A. However, peripheral A
                /// generally does not drive the pin as the PIO Controller resets
                /// in I/O line mode.
                /// 
                /// Writing in PIO_ABSR manages the multiplexing regardless of
                /// the configuration of the pin. However, assignment of a pin
                /// to a peripheral function requires a write in the peripheral
                /// selection register (PIO_ABSR) in addition to a write in
                /// PIO_PDR.
                pub struct ABSR {
                    _ownership: (),
                }

                impl ABSR {
                    pub(crate) fn absr(&mut self) -> &#lower_name::ABSR {
                        unsafe { &(*#upper_name::ptr()).absr }
                    }
                }

                /// Opaque MDDR register.  Datasheet §31.5.6
                /// 
                /// Each I/O can be independently programmed in Open Drain by
                /// using the Multi Drive feature. This feature permits several
                /// drivers to be connected on the I/O line which is driven low
                /// only by each device. An external pull-up resistor (or
                /// enabling of the internal one) is generally required to
                /// guarantee a high level on the line.
                /// 
                /// The Multi Drive feature is controlled by PIO_MDER (Multi-driver
                /// Enable Register) and PIO_MDDR (Multi-driver Disable Register).
                /// The Multi Drive can be selected whether the I/O line is
                /// controlled by the PIO controller or assigned to a peripheral
                /// function. PIO_MDSR (Multi-driver Status Register) indicates
                /// the pins that are configured to support external drivers.
                pub struct MDDR {
                    _ownership: (),
                }

                impl MDDR {
                    pub(crate) fn mddr(&mut self) -> &#lower_name::MDDR {
                        unsafe { &(*#upper_name::ptr()).mddr }
                    }
                }

                /// Opaque MDER register.  Datasheet §31.5.6
                /// 
                /// Each I/O can be independently programmed in Open Drain by
                /// using the Multi Drive feature. This feature permits several
                /// drivers to be connected on the I/O line which is driven low
                /// only by each device. An external pull-up resistor (or
                /// enabling of the internal one) is generally required to
                /// guarantee a high level on the line.
                /// 
                /// The Multi Drive feature is controlled by PIO_MDER (Multi-driver
                /// Enable Register) and PIO_MDDR (Multi-driver Disable Register).
                /// The Multi Drive can be selected whether the I/O line is
                /// controlled by the PIO controller or assigned to a peripheral
                /// function. PIO_MDSR (Multi-driver Status Register) indicates
                /// the pins that are configured to support external drivers.
                pub struct MDER {
                    _ownership: (),
                }

                impl MDER {
                    pub(crate) fn mder(&mut self) -> &#lower_name::MDER {
                        unsafe { &(*#upper_name::ptr()).mder }
                    }
                }

                /// Opaque OER register.  Datasheet §31.5.4
                /// 
                /// When the I/O line is controlled by the PIO controller, the
                /// pin can be configured to be driven. This is done by writing
                /// PIO_OER (Output Enable Register) and PIO_ODR (Output Disable
                /// Register). The results of these write operations are detected
                /// in PIO_OSR (Output Status Register). When a bit in this
                /// register is at 0, the corresponding I/O line is used as an
                /// input only. When the bit is at 1, the corresponding I/O line
                /// is driven by the PIO controller.
                pub struct OER {
                    _ownership: (),
                }

                impl OER {
                    pub(crate) fn oer(&mut self) -> &#lower_name::OER {
                        unsafe { &(*#upper_name::ptr()).oer }
                    }
                }

                /// Partially erased pin
                pub struct #pio_partial_erase<MODE> {
                    i: u8,
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> #pio_partial_erase<MODE> {
                    /// Erases the port letter from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> PXx<MODE> {
                        PXx {
                            i: self.i,
                            gpio: Gpio::#upper_name,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for #pio_partial_erase<Output<MODE>> {
                    type Error = ();
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*#upper_name::ptr()).sodr.write_with_zero(|w| w.bits(1 << self.i)) };
                        Ok(())
                    }
                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        unsafe { (*#upper_name::ptr()).codr.write_with_zero(|w| w.bits(1 << self.i)) };
                        Ok(())
                    }
                }

                #[cfg(feature = "unproven")]
                impl<MODE> InputPin for #pio_partial_erase<Input<MODE>> {
                    type Error = ();
                    fn is_high(&self) -> Result<bool, Self::Error> {
                        unimplemented!()
                    }
                    fn is_low(&self) -> Result<bool, Self::Error> {
                        unimplemented!()
                    }
                }

                #(#pins)*
            }

        ));
    }
}
