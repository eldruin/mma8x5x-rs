pub const MMA845X_BASE_ADDR: u8 = 0x1C;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// IC markers
pub mod ic {
    /// MMA8451 IC marker
    pub struct Mma8451;
    /// MMA8452 IC marker
    pub struct Mma8452;
    /// MMA8453 IC marker
    pub struct Mma8453;
    /// MMA8652 IC marker
    pub struct Mma8652;
    /// MMA8653 IC marker
    pub struct Mma8653;
}

/// Unscaled acceleration measurement
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnscaledMeasurement {
    /// X-axis acceleration.
    pub x: i16,
    /// Y-axis acceleration.
    pub y: i16,
    /// Z-axis acceleration.
    pub z: i16,
}

/// Acceleration measurement scaled to configured G range
#[derive(Debug, Default, Clone)]
pub struct Measurement {
    /// X-axis acceleration.
    pub x: f32,
    /// Y-axis acceleration.
    pub y: f32,
    /// Z-axis acceleration.
    pub z: f32,
}

/// G scale
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GScale {
    /// Range: +/-2g
    G2,
    /// Range: +/-4g
    G4,
    /// Range: +/-8g
    G8,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for A0
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a0) => default | a0 as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MMA845X_BASE_ADDR as BASE_ADDR;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(BASE_ADDR, addr.addr(BASE_ADDR));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(BASE_ADDR, SlaveAddr::Alternative(false).addr(BASE_ADDR));
        assert_eq!(BASE_ADDR | 1, SlaveAddr::Alternative(true).addr(BASE_ADDR));
    }
}
