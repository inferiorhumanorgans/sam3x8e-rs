//! Delays

use crate::pmc::Clocks;

mod syst;
mod tc;

pub use syst::*;
pub use tc::*;

pub trait DelayExt<S> {
    fn delay(self, clocks: Clocks) -> Delay<S>;
}

/// Generic timer struct.  Possible sources include System timer (SysTick) and the Timer Counter modules (TC0...TC2)
pub struct Delay<S> {
    source: S,
    clocks: Clocks,
}
