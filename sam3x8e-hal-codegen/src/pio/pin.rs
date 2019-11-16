use syn::Ident;
use quote::ToTokens;
use proc_macro2::TokenStream;

use crate::inflector::Inflector;

pub struct Pin<'a> {
    pub pio: &'a Ident,
    pub index: u8,
}

impl<'a> ToTokens for Pin<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let upper_ident = self.pio.to_string().to_screaming_snake_case();
        let pin_ident = format_ident!("P{}{}", upper_ident, self.index);
        let upper_name = format_ident!("PIO{}", upper_ident);
        let accessor = format_ident!("p{}", self.index);
        let pio_remove_pin = format_ident!("P{}x", upper_ident);

        tokens.extend(quote!(
            /// Parallel I/O Pin.  Datasheet §31
            pub struct #pin_ident<MODE> {
                _mode: PhantomData<MODE>
            }

            impl<MODE> #pin_ident<MODE> {
                /// Configures the pin to use peripheral A
                /// AB Select -- select A.  Datasheet §31.5.3
                pub fn into_peripheral_a(self, absr: &mut ABSR) -> #pin_ident<PeripheralA> {
                    absr.absr().write(|w| w.#accessor().clear_bit());
                    #pin_ident { _mode: PhantomData }
                }

                /// Configures the pin to use peripheral B
                /// AB Select -- select B.  Datasheet §31.5.3
                pub fn into_peripheral_b(self, absr: &mut ABSR) -> #pin_ident<PeripheralB> {
                    absr.absr().write(|w| w.#accessor().set_bit());
                    #pin_ident { _mode: PhantomData }
                }

                /// Configures the pin to operate as a floating input pin
                pub fn into_floating_input(
                    self,
                ) -> #pin_ident<Input<Floating>> {
                    unimplemented!()
                }

                /// Configures the pin to operate as a pulled down input pin
                pub fn into_pull_down_input(
                    self,
                ) -> #pin_ident<Input<PullDown>> {
                    unimplemented!()
                }

                /// Configures the pin to operate as a pulled up input pin
                pub fn into_pull_up_input(
                    self,
                ) -> #pin_ident<Input<PullUp>> {
                    unimplemented!()
                }

                /// Configures the pin to operate as an open drain output pin
                /// Datasheet §31.5.6
                pub fn into_open_drain_output(
                    self,
                    mder: &mut MDER,
                    oer: &mut OER,
                ) -> #pin_ident<Output<OpenDrain>> {
                    // OER = Output Enable Register
                    oer.oer().write_with_zero(|w| w.#accessor().set_bit());

                    // Enable multi-mode / open drain
                    mder.mder().write_with_zero(|w| w.#accessor().set_bit());

                    #pin_ident { _mode: PhantomData }
                }

                /// Configures the pin to operate as an push pull output pin
                pub fn into_push_pull_output(
                    self,
                    mddr: &mut MDDR,
                    oer: &mut OER,
                ) -> #pin_ident<Output<PushPull>> {
                    // OER = Output Enable Register
                    oer.oer().write_with_zero(|w| w.#accessor().set_bit());

                    // Disable multi-mode / open drain
                    mddr.mddr().write_with_zero(|w| w.#accessor().set_bit());

                    #pin_ident { _mode: PhantomData }
                }
            }

            impl<MODE> #pin_ident<Output<MODE>> {
                /// Erases the pin number from the type
                ///
                /// This is useful when you want to collect the pins into an array where you
                /// need all the elements to have the same type
                pub fn downgrade(self) -> #pio_remove_pin<Output<MODE>> {
                    #pio_remove_pin {
                        i: 0,
                        _mode: self._mode,
                    }
                }
            }

            impl<MODE> #pin_ident<Input<MODE>> {
                /// Erases the pin number from the type
                ///
                /// This is useful when you want to collect the pins into an array where you
                /// need all the elements to have the same type
                pub fn downgrade(self) -> #pio_remove_pin<Input<MODE>> {
                    #pio_remove_pin {
                        i: 0,
                        _mode: self._mode,
                    }
                }
            }

            impl<MODE> OutputPin for #pin_ident<Output<MODE>> {
                type Error = ();
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*#upper_name::ptr()).sodr.write_with_zero(|w| w.#accessor().set_bit()) };
                    Ok(())
                }
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    unsafe { (*#upper_name::ptr()).codr.write_with_zero(|w| w.#accessor().set_bit()) };
                    Ok(())
                }
            }

            #[cfg(feature = "unproven")]
            impl<MODE> InputPin for #pin_ident<Input<MODE>> {
                type Error = ();
                fn is_high(&self) -> Result<bool, Self::Error> {
                    Ok(unsafe { (*#upper_name::ptr()).pdsr.read().#accessor().bits() })
                }
                fn is_low(&self) -> Result<bool, Self::Error> {
                    Ok(unsafe { !(*#upper_name::ptr()).pdsr.read().#accessor().bits() })
                }
            }
        ))
    }
}
