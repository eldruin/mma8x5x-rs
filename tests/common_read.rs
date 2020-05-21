mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, Register, ADDRESS,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::{Measurement, UnscaledMeasurement};

macro_rules! read_tests {
    ($name:ident, $create:ident, $bit_shift:expr, $max:expr) => {
        mod $name {
            use super::*;

            #[test]
            fn can_read_unscaled() {
                let mut sensor = $create(&[I2cTrans::write_read(
                    ADDRESS,
                    vec![Register::OUT_X_H],
                    vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
                )]);
                let expected = UnscaledMeasurement {
                    x: 0x4140 >> $bit_shift,
                    y: 0x4280 >> $bit_shift,
                    z: 0x43C0 >> $bit_shift,
                };
                assert_eq!(expected, sensor.read_unscaled().unwrap());
                destroy(sensor);
            }

            #[test]
            fn can_read() {
                let mut sensor = $create(&[I2cTrans::write_read(
                    ADDRESS,
                    vec![Register::OUT_X_H],
                    vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
                )]);
                let expected = Measurement {
                    x: f32::from(0x4140_u16 >> $bit_shift) / ($max / 2.0),
                    y: f32::from(0x4280_u16 >> $bit_shift) / ($max / 2.0),
                    z: f32::from(0x43C0_u16 >> $bit_shift) / ($max / 2.0),
                };
                let m = sensor.read().unwrap();
                assert_near!(m.x, expected.x, 0.01);
                assert_near!(m.y, expected.y, 0.01);
                assert_near!(m.z, expected.z, 0.01);
                destroy(sensor);
            }
        }
    };
}

read_tests!(mma8451, new_mma8451, 2, 4096.0);
read_tests!(mma8452, new_mma8452, 4, 1024.0);
read_tests!(mma8453, new_mma8453, 6, 256.0);
read_tests!(mma8652, new_mma8652, 4, 1024.0);
read_tests!(mma8653, new_mma8653, 6, 256.0);
