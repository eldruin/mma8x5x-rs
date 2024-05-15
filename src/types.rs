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
    #[derive(Debug)]
    pub struct Standby;
    /// Active mode
    #[derive(Debug)]
    pub struct Active;
}

/// Unscaled acceleration measurement
#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GScale {
    /// Range: +/-2g
    G2,
    /// Range: +/-4g
    G4,
    /// Range: +/-8g
    G8,
}

/// Read mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadMode {
    /// Normal read mode (default)
    Normal,
    /// Fast read mode
    Fast,
}

/// Output data rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemMode {
    /// Standby mode
    Standby,
    /// Wake mode
    Wake,
    /// Sleep mode
    Sleep,
}

/// Current data status
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebounceCounterMode {
    /// Decrements debounce whenever the condition of interest is no longer valid.
    Decrement,
    /// Clears the counter whenever the condition of interest is no longer valid. (default)
    Clear,
}

/// Current portrait/landscape status
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PortraitLandscapeStatus {
    /// True if any of the other fields changed
    pub something_changed: bool,
    /// Z-tilt angle lookout detected
    pub z_tilt_angle_lookout: bool,
    /// Portrait/Landscape orientation
    pub portrait_landscape: PortraitLandscapeOrientation,
    /// Front/Back orientation
    pub front_back: FrontBackOrientation,
}

/// Portrait/Landscape orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PortraitLandscapeOrientation {
    /// Equipment is standing vertically in the normal orientation (default)
    #[default]
    PortraitUp,
    /// Equipment is standing vertically in the inverted orientation
    PortraitDown,
    /// Equipment is standing in landscape mode to the right
    LandscapeRight,
    /// Equipment is standing in landscape mode to the left
    LandscapeLeft,
}

/// Front/Back orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrontBackOrientation {
    /// Equipment is in front-facing orientation (default)
    #[default]
    Front,
    /// Equipment is in back-facing orientation
    Back,
}

/// Freefall/Motion detection mode
#[derive(Default, Debug)]
pub enum FreefallMotionDetectionMode {
    /// detect freefall
    #[default]
    Freefall,
    /// detect motion
    Motion,
}

/// Freefall/Motion debounce mode
#[derive(Default, Debug)]
pub enum FreefallMotionDebounceMode {
    /// Increments or decrements debounce
    #[default]
    IncrementOrDecrement,
    /// Increments or clears counter
    IncrementOrClear,
}

/// Freefall/Motion detection configuration
#[derive(Debug)]
pub struct FreefallMotionConfiguration {
    /// if true, event is latched into FF_MT_SRC and must be read to clear
    pub event_latch: bool,
    /// detection mode
    pub detection_mode: FreefallMotionDetectionMode,
    /// enable detection in x-axis
    pub x_axis: bool,
    /// enable detection in y-axis
    pub y_axis: bool,
    /// enable detection in z-axis
    pub z_axis: bool,
    /// detection threshold, in 1/1000 G
    pub threshold: u16,
    /// debounce counter mode
    pub debounce_mode: FreefallMotionDebounceMode,
    /// debounce sample count
    pub debounce_count: u8,
}

impl Default for FreefallMotionConfiguration {
    fn default() -> Self {
        Self {
            event_latch: false,
            detection_mode: Default::default(),
            x_axis: true,
            y_axis: true,
            z_axis: true,
            threshold: 500,
            debounce_mode: Default::default(),
            debounce_count: 0,
        }
    }
}

/// Freefall/Motion axis activity
#[derive(Debug)]
pub enum FreefallMotionAxisActivity {
    /// motion in the axis not detected
    None,
    /// positive axis movement detected
    Positive,
    /// negative axis movement detected
    Negative,
}

/// Freefall/Motion event source
#[derive(Debug)]
pub struct FreefallMotionSource {
    /// movement activity in x-axis
    pub x_axis: FreefallMotionAxisActivity,
    /// movement activity in y-axis
    pub y_axis: FreefallMotionAxisActivity,
    /// movement activity in z-axis
    pub z_axis: FreefallMotionAxisActivity,
}

/// Current interrupt status
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterruptStatus {
    /// Auto-sleep/wake interrupt occurred
    pub auto_sleep: bool,
    /// FIFO interrupt occurred
    pub fifo: bool,
    /// Acceleration transient value greater than user specified threshold has occurred
    pub transient: bool,
    /// Portrait/landscape orientation interrupt occurred
    pub portrait_landscape: bool,
    /// Single and/or double pulse interrupt occurred
    pub pulse: bool,
    /// Freefall/motion interrupt occurred
    pub freefall_motion: bool,
    /// New data ready and/or data overrun interrupt occurred
    pub data_ready: bool,
}

/// Interrupt source enable/disable
///
/// Unavailable interrupt sources on a device are ignored.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EnabledInterrupts {
    /// Auto-sleep/wake interrupt enabled
    pub auto_sleep: bool,
    /// FIFO interrupt enabled
    pub fifo: bool,
    /// Acceleration transient interrupt enabled
    pub transient: bool,
    /// Portrait/landscape orientation interrupt enabled
    pub portrait_landscape: bool,
    /// Single and/or double pulse detection interrupt enabled
    pub pulse: bool,
    /// Freefall/motion interrupt enabled
    pub freefall_motion: bool,
    /// Data ready interrupt enabled
    pub data_ready: bool,
}

/// Physical interrupt pin polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptPinPolarity {
    /// Low state when active (default)
    ActiveLow,
    /// High state when active
    ActiveHigh,
}

/// Physical interrupt pin configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptPinConfiguration {
    /// Push-pull configuration (default)
    PushPull,
    /// Open drain configuration
    OpenDrain,
}

/// Interrupt source pin route
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InterruptSourcePinRoute {
    /// Source is routed to pin INT1
    Int1,

    /// Source is routed to pin INT2 (default)
    #[default]
    Int2,
}

/// Interrupt source pin route
///
/// Unavailable interrupt sources on a device are ignored.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InterruptPinRoutes {
    /// Auto-sleep/wake interrupt pin route (INT1/INT2)
    pub auto_sleep: InterruptSourcePinRoute,
    /// FIFO interrupt pin route (INT1/INT2)
    pub fifo: InterruptSourcePinRoute,
    /// Acceleration transient interrupt pin route (INT1/INT2)
    pub transient: InterruptSourcePinRoute,
    /// Portrait/landscape orientation interrupt pin route (INT1/INT2)
    pub portrait_landscape: InterruptSourcePinRoute,
    /// Single and/or double pulse detection interrupt pin route (INT1/INT2)
    pub pulse: InterruptSourcePinRoute,
    /// Freefall/motion interrupt pin route (INT1/INT2)
    pub freefall_motion: InterruptSourcePinRoute,
    /// Data ready interrupt pin route (INT1/INT2)
    pub data_ready: InterruptSourcePinRoute,
}

/// Interrupts that wake the device from sleep
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WakeInterrupts {
    /// Acceleration transient interrupt
    pub transient: bool,
    /// Portrait/landscape orientation interrupt
    pub portrait_landscape: bool,
    /// Single and/or double pulse detection interrupt
    pub pulse: bool,
    /// Freefall/motion interrupt
    pub freefall_motion: bool,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SlaveAddr {
    /// Default slave address
    #[default]
    Default,
    /// Alternative slave address providing bit value for A0
    Alternative(bool),
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
