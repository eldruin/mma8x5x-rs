mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::SystemMode;

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            #[test]
            fn can_get_offset_correction() {
                let mut sensor = $create(&[I2cTrans::write_read(
                    ADDRESS,
                    vec![Register::OFF_X],
                    vec![0x7F, 0x80, 0xFF],
                )]);
                let offsets = sensor.offset_correction().unwrap();
                assert_eq!((127, -128, -1), offsets);
                destroy(sensor);
            }

            get_test!(
                can_get_sysmod_standby,
                new_mma8653,
                SYSMOD,
                0,
                system_mode,
                SystemMode::Standby
            );
            get_test!(
                can_get_sysmod_wake,
                new_mma8653,
                SYSMOD,
                1,
                system_mode,
                SystemMode::Wake
            );
            get_test!(
                can_get_sysmod_sleep,
                new_mma8653,
                SYSMOD,
                2,
                system_mode,
                SystemMode::Sleep
            );
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);

get_test!(
    can_read_device_id,
    new_mma8653,
    WHO_AM_I,
    0x3A,
    device_id,
    0x3A
);
