mod base;
use crate::base::{destroy, new_mma8653, Register};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

#[test]
fn can_create_and_destroy() {
    let sensor = new_mma8653(&[]);
    destroy(sensor);
}

#[test]
fn can_read_device_id() {
    let mut sensor = new_mma8653(&[I2cTrans::write_read(
        0x1D,
        vec![Register::WHO_AM_I],
        vec![0x3A],
    )]);
    assert_eq!(0x3A, sensor.device_id().unwrap());
    destroy(sensor);
}
