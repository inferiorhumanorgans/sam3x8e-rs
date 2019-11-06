//! Delays

mod tc0;
mod syst;

pub use tc0::*;
pub use syst::*;

/// System timer (SysTick) as a delay provider
pub struct Delay<S> {
    source: S,
}

