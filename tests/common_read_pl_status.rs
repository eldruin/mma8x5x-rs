mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::{FrontBackOrientation, PortraitLandscapeOrientation, PortraitLandscapeStatus};

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            get_test!(
                nothing,
                $create,
                PL_STATUS,
                0,
                portrait_landscape_status,
                PortraitLandscapeStatus::default()
            );
            get_test!(
                all,
                $create,
                PL_STATUS,
                BF::NEWLP | BF::LO | BF::LAPO1 | BF::LAPO0 | BF::BAFRO,
                portrait_landscape_status,
                PortraitLandscapeStatus {
                    something_changed: true,
                    z_tilt_angle_lookout: true,
                    portrait_landscape: PortraitLandscapeOrientation::LandscapeLeft,
                    front_back: FrontBackOrientation::Back
                }
            );
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);
