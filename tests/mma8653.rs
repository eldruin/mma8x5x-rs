mod base;
use crate::base::{destroy, new_mma8653, Register};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::{Measurement, UnscaledMeasurement};

#[test]
fn can_read_10_bit_unscaled() {
    let mut sensor = new_mma8653(&[I2cTrans::write_read(
        0x1D,
        vec![Register::OUT_X_H],
        vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
    )]);
    let expected = UnscaledMeasurement {
        x: 0x4140 >> 6,
        y: 0x4280 >> 6,
        z: 0x43C0 >> 6,
    };
    assert_eq!(expected, sensor.read_unscaled().unwrap());
    destroy(sensor);
}

#[test]
fn can_read_10_bit() {
    let mut sensor = new_mma8653(&[I2cTrans::write_read(
        0x1D,
        vec![Register::OUT_X_H],
        vec![0x41, 0x40, 0x42, 0x80, 0x43, 0xC0],
    )]);
    let expected = Measurement {
        x: f32::from(0x4140_u16 >> 6) / (256.0 / 2.0),
        y: f32::from(0x4280_u16 >> 6) / (256.0 / 2.0),
        z: f32::from(0x43C0_u16 >> 6) / (256.0 / 2.0),
    };
    let m = sensor.read().unwrap();
    assert_near!(m.x, expected.x, 0.01);
    assert_near!(m.y, expected.y, 0.01);
    assert_near!(m.z, expected.z, 0.01);
    destroy(sensor);
}
