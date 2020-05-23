pub const MMA845X_BASE_ADDR: u8 = 0x1C;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// All possible errors in this crate
#[derive(Debug)]
pub struct ModeChangeError<E, DEV> {
    /// I²C bus error
    pub error: Error<E>,
    /// Original device without mode changed
    pub dev: DEV,
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

/// Mode markers
pub mod mode {
    /// Standby mode
    pub struct Standby;
    /// Active mode
    pub struct Active;
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

/// Read mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadMode {
    /// Normal read mode (default)
    Normal,
    /// Fast read mode
    Fast,
}

/// Output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputDataRate {
    /// 800 Hz (default)
    Hz800,
    /// 400 Hz
    Hz400,
    /// 200 Hz
    Hz200,
    /// 100 Hz
    Hz100,
    /// 50 Hz
    Hz50,
    /// 12.5 Hz
    Hz12_5,
    /// 6.25 Hz
    Hz6_25,
    /// 1.56 Hz
    Hz1_56,
}

/// Sampling rate used in auto-sleep/wake mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AutoSleepDataRate {
    /// 50 Hz (default)
    Hz50,
    /// 12.5 Hz
    Hz12_5,
    /// 6.25 Hz
    Hz6_25,
    /// 1.56 Hz
    Hz1_56,
}

/// Power mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    /// Normal (default)
    Normal,
    /// Low noise low power
    LowNoiseLowPower,
    /// High resolution
    HighResolution,
    ///Low power
    LowPower,
}

/// Current system mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemMode {
    /// Standby mode
    Standby,
    /// Wake mode
    Wake,
    /// Sleep mode
    Sleep,
}

/// Current data status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DataStatus {
    /// X,Y,Z-axis data overwrite
    pub xyz_overwrite: bool,
    /// X-axis data overwrite
    pub x_overwrite: bool,
    /// Y-axis data overwrite
    pub y_overwrite: bool,
    /// Z-axis data overwrite
    pub z_overwrite: bool,
    /// X,Y,Z-axis new data ready
    pub xyz_new_data: bool,
    /// X-axis data overwrite
    pub x_new_data: bool,
    /// Y-axis data overwrite
    pub y_new_data: bool,
    /// Z-axis data overwrite
    pub z_new_data: bool,
}

/// Portrait/landscape debounce counter mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebounceCounterMode {
    /// Decrements debounce whenever the condition of interest is no longer valid.
    Decrement,
    /// Clears the counter whenever the condition of interest is no longer valid. (default)
    Clear,
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
    use super::MMA845X_BASE_ADDR as BASE_ADDR;
    use super::*;

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
