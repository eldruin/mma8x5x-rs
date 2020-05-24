mod base;
use crate::base::{destroy, new_mma8451, new_mma8652, Register, ADDRESS};
use embedded_hal_mock::i2c::Transaction as I2cTrans;

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            #[test]
            fn set_debounce_counter() {
                let sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                    I2cTrans::write(ADDRESS, vec![Register::PL_COUNT, 0xAB]),
                ]);
                let mut sensor = sensor.into_active().ok().unwrap();
                sensor.set_debounce_counter(0xAB).unwrap();
                destroy(sensor);
            }
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8652, new_mma8652);
