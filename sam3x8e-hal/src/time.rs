//! Time units

use core::fmt;

/// Bits per second
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bps(pub u32);

/// Hertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Hertz(pub u32);

/// KiloHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct KiloHertz(pub u32);

/// MegaHertz
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MegaHertz(pub u32);

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct MicroSeconds(pub u32);

impl fmt::Display for MicroSeconds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} µs", self.0)
    }
}

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `MicroSeconds`
    fn us(self) -> MicroSeconds;

    /// Wrap in `MicroSeconds`
    fn ms(self) -> MicroSeconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn ms(self) -> MicroSeconds {
        MicroSeconds(self * 1_000)
    }

    fn us(self) -> MicroSeconds {
        MicroSeconds(self)
    }
}

impl Into<MicroSeconds> for Hertz {
    fn into(self) -> MicroSeconds {
        let freq = self.0;
        assert!(freq != 0 && freq <= 1_000_000);
        MicroSeconds(1_000_000 / freq)
    }
}

impl Into<Hertz> for MicroSeconds {
    fn into(self) -> Hertz {
        let period = self.0;
        assert!(period != 0 && period <= 1_000_000);
        Hertz(1_000_000 / period)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}
