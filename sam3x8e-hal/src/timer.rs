//! Delays

use crate::time::Hertz;

mod syst;

pub use syst::*;

pub trait TimerExt<TIM> {
    fn timer<T>(self, timeout: T) -> Timer<TIM>
    where
        T: Into<Hertz>;
}

/// Hardware timers
pub struct Timer<TIM> {
    tim: TIM,
}
