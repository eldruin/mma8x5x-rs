mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, Register, ADDRESS,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::{GScale, Measurement, ReadMode, UnscaledMeasurement};

macro_rules! set_scale_read_test {
    ($name:ident, $create:ident, $gscale:ident, $gscale_bits:expr, $bit_shift:expr, $max:expr, $g:expr) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[
                I2cTrans::write(ADDRESS, vec![Register::XYZ_DATA_CFG, $gscale_bits]),
                I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                I2cTrans::write_read(
                    ADDRESS,
                    vec![Register::OUT_X_H],
                    vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
                ),
            ]);
            let expected = Measurement {
                x: f32::from(0x4140_u16 >> $bit_shift) / ($max / $g),
                y: f32::from(0x4280_u16 >> $bit_shift) / ($max / $g),
                z: f32::from(0x43C0_u16 >> $bit_shift) / ($max / $g),
            };
            sensor.set_scale(GScale::$gscale).unwrap();
            let mut sensor = sensor.active().ok().unwrap();
            let m = sensor.read().unwrap();
            assert_near!(m.x, expected.x, 0.01);
            assert_near!(m.y, expected.y, 0.01);
            assert_near!(m.z, expected.z, 0.01);
            destroy(sensor);
        }
    };
}

macro_rules! read_tests {
    ($name:ident, $create:ident, $bit_shift:expr, $max:expr) => {
        mod $name {
            use super::*;

            #[test]
            fn can_read_unscaled() {
                let sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                    I2cTrans::write_read(
                        ADDRESS,
                        vec![Register::OUT_X_H],
                        vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
                    ),
                ]);
                let expected = UnscaledMeasurement {
                    x: 0x4140 >> $bit_shift,
                    y: 0x4280 >> $bit_shift,
                    z: 0x43C0 >> $bit_shift,
                };
                let mut sensor = sensor.active().ok().unwrap();
                assert_eq!(expected, sensor.read_unscaled().unwrap());
                destroy(sensor);
            }

            #[test]
            fn can_read() {
                let sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                    I2cTrans::write_read(
                        ADDRESS,
                        vec![Register::OUT_X_H],
                        vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
                    ),
                ]);
                let expected = Measurement {
                    x: f32::from(0x4140_u16 >> $bit_shift) / ($max / 2.0),
                    y: f32::from(0x4280_u16 >> $bit_shift) / ($max / 2.0),
                    z: f32::from(0x43C0_u16 >> $bit_shift) / ($max / 2.0),
                };
                let mut sensor = sensor.active().ok().unwrap();
                let m = sensor.read().unwrap();
                assert_near!(m.x, expected.x, 0.01);
                assert_near!(m.y, expected.y, 0.01);
                assert_near!(m.z, expected.z, 0.01);
                destroy(sensor);
            }

            #[test]
            fn can_read_fast_unscaled() {
                let mut sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 2]),
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 3]),
                    I2cTrans::write_read(ADDRESS, vec![Register::OUT_X_H], vec![0x41, 0x42, 0x43]),
                ]);
                let expected = UnscaledMeasurement {
                    x: 0x4100 >> $bit_shift,
                    y: 0x4200 >> $bit_shift,
                    z: 0x4300 >> $bit_shift,
                };
                sensor.set_read_mode(ReadMode::Fast).unwrap();
                let mut sensor = sensor.active().ok().unwrap();
                assert_eq!(expected, sensor.read_unscaled().unwrap());
                destroy(sensor);
            }

            set_scale_read_test!(set_2g_read, $create, G2, 0, $bit_shift, $max, 2.0);
            set_scale_read_test!(set_4g_read, $create, G4, 1, $bit_shift, $max, 4.0);
            set_scale_read_test!(set_8g_read, $create, G8, 2, $bit_shift, $max, 8.0);
        }
    };
}

read_tests!(mma8451, new_mma8451, 2, 4096.0);
read_tests!(mma8452, new_mma8452, 4, 1024.0);
read_tests!(mma8453, new_mma8453, 6, 256.0);
read_tests!(mma8652, new_mma8652, 4, 1024.0);
read_tests!(mma8653, new_mma8653, 6, 256.0);
