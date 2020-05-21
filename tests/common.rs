mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, Register,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;
            #[test]
            fn can_create_and_destroy() {
                let sensor = $create(&[]);
                destroy(sensor);
            }
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);

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
