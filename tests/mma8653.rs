mod base;
use crate::base::{destroy, new_mma8653, Register};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::UnscaledMeasurement;

#[test]
fn can_read_10_bit_acc_unscaled() {
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
