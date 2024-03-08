mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::{OutputDataRate, PowerMode};

macro_rules! set_odr_test {
    ($name:ident, $create:ident, $variant:ident, $expected:expr) => {
        set_test!(
            $name,
            $create,
            CTRL_REG1,
            $expected,
            set_data_rate,
            OutputDataRate::$variant
        );
    };
}

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;
            #[test]
            fn can_create_and_destroy() {
                let sensor = $create(&[]);
                destroy(sensor);
            }

            #[test]
            fn can_activate_then_standby() {
                let sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 0]),
                ]);
                let sensor = sensor.into_active().ok().unwrap();
                let sensor = sensor.into_standby().ok().unwrap();
                destroy(sensor);
            }

            #[test]
            fn can_set_offset_correction() {
                let mut sensor = $create(&[I2cTrans::write(
                    ADDRESS,
                    vec![Register::OFF_X, 0x7F, 0x80, 0xFF],
                )]);
                sensor.set_offset_correction(127, -128, -1).unwrap();
                destroy(sensor);
            }

            set_odr_test!(set_odr_800, $create, Hz800, 0);
            set_odr_test!(set_odr_400, $create, Hz400, BF::ODR0);
            set_odr_test!(set_odr_200, $create, Hz200, BF::ODR1);
            set_odr_test!(set_odr_100, $create, Hz100, BF::ODR1 | BF::ODR0);
            set_odr_test!(set_odr_50, $create, Hz50, BF::ODR2);
            set_odr_test!(set_odr_12_5, $create, Hz12_5, BF::ODR2 | BF::ODR0);
            set_odr_test!(set_odr_6_25, $create, Hz6_25, BF::ODR2 | BF::ODR1);
            set_odr_test!(
                set_odr_1_56,
                $create,
                Hz1_56,
                BF::ODR2 | BF::ODR1 | BF::ODR0
            );

            set_test!(
                set_wake_pm_normal,
                $create,
                CTRL_REG2,
                0,
                set_wake_power_mode,
                PowerMode::Normal
            );
            set_test!(
                set_wake_pm_low_noise,
                $create,
                CTRL_REG2,
                BF::MODS0,
                set_wake_power_mode,
                PowerMode::LowNoiseLowPower
            );
            set_test!(
                set_wake_pm_high_resolution,
                $create,
                CTRL_REG2,
                BF::MODS1,
                set_wake_power_mode,
                PowerMode::HighResolution
            );
            set_test!(
                set_wake_pm_low_power,
                $create,
                CTRL_REG2,
                BF::MODS1 | BF::MODS0,
                set_wake_power_mode,
                PowerMode::LowPower
            );

            set_test!(can_reset, $create, CTRL_REG2, BF::RST, reset);
            #[test]
            fn can_activate_then_reset() {
                let sensor = $create(&[
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG1, 1]),
                    I2cTrans::write(ADDRESS, vec![Register::CTRL_REG2, BF::RST]),
                ]);
                let sensor = sensor.into_active().ok().unwrap();
                let sensor = sensor.reset().ok().unwrap();
                destroy(sensor);
            }

            set_test!(
                enable_self_test,
                $create,
                CTRL_REG2,
                BF::ST,
                enable_self_test
            );
            set_test!(disable_self_test, $create, CTRL_REG2, 0, disable_self_test);
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);
