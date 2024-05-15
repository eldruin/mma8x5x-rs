use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{
    mode,
    register_access::{BitFlags, Register},
    types::{FreefallMotionAxisActivity, FreefallMotionSource},
    Error, FreefallMotionConfiguration, FreefallMotionDebounceMode, FreefallMotionDetectionMode,
    Mma8x5x,
};

/// Freefall/motion detection configuration
impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// enable freefall or motion detection
    pub fn enable_freefall_motion_detection(
        &mut self,
        conf: FreefallMotionConfiguration,
    ) -> Result<(), Error<E>> {
        let mut value = match conf.detection_mode {
            FreefallMotionDetectionMode::Freefall => 0,
            FreefallMotionDetectionMode::Motion => BitFlags::OAE,
        };

        if conf.event_latch {
            value |= BitFlags::ELE;
        }

        if conf.x_axis {
            value |= BitFlags::XEFE;
        }

        if conf.y_axis {
            value |= BitFlags::YEFE;
        }

        if conf.z_axis {
            value |= BitFlags::ZEFE;
        }

        self.write_reg(Register::FF_MT_CFG, value)?;

        let mut value = match conf.debounce_mode {
            FreefallMotionDebounceMode::IncrementOrDecrement => 0,
            FreefallMotionDebounceMode::IncrementOrClear => BitFlags::DBCNTM,
        };
        let thres: u8 = (conf.threshold / 63).clamp(0, 127).try_into().unwrap();
        value |= thres;
        self.write_reg(Register::FF_MT_THS, value)?;

        self.write_reg(Register::FF_MT_COUNT, value)?;

        Ok(())
    }
}

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Active>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// get source of the freefall/motion event
    pub fn freefall_motion_source(&mut self) -> Result<Option<FreefallMotionSource>, Error<E>> {
        let act = |he, hp| match (he, hp) {
            (false, _) => FreefallMotionAxisActivity::None,
            (true, false) => FreefallMotionAxisActivity::Positive,
            (true, true) => FreefallMotionAxisActivity::Negative,
        };

        self.read_reg(Register::FF_MT_SRC).map(|src| {
            if (src & BitFlags::EA) != 0 {
                Some(FreefallMotionSource {
                    x_axis: act((src & BitFlags::XHE) != 0, (src & BitFlags::XHP) != 0),
                    y_axis: act((src & BitFlags::YHE) != 0, (src & BitFlags::YHP) != 0),
                    z_axis: act((src & BitFlags::ZHE) != 0, (src & BitFlags::ZHP) != 0),
                })
            } else {
                None
            }
        })
    }
}
