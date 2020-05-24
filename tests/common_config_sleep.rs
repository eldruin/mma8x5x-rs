mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::{AutoSleepDataRate, PowerMode};

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            set_test!(
                set_sleep_pm_normal,
                $create,
                CTRL_REG2,
                0,
                set_sleep_power_mode,
                PowerMode::Normal
            );
            set_test!(
                set_sleep_pm_low_noise,
                $create,
                CTRL_REG2,
                BF::SMODS0,
                set_sleep_power_mode,
                PowerMode::LowNoiseLowPower
            );
            set_test!(
                set_sleep_pm_high_resolution,
                $create,
                CTRL_REG2,
                BF::SMODS1,
                set_sleep_power_mode,
                PowerMode::HighResolution
            );
            set_test!(
                set_sleep_pm_low_power,
                $create,
                CTRL_REG2,
                BF::SMODS1 | BF::SMODS0,
                set_sleep_power_mode,
                PowerMode::LowPower
            );

            set_test!(
                enable_auto_sleep,
                $create,
                CTRL_REG2,
                BF::SLPE,
                enable_auto_sleep
            );
            set_test!(
                disable_auto_sleep,
                $create,
                CTRL_REG2,
                0,
                disable_auto_sleep
            );

            set_test!(
                set_awake_data_rate_50hz,
                $create,
                CTRL_REG1,
                0,
                set_auto_sleep_data_rate,
                AutoSleepDataRate::Hz50
            );
            set_test!(
                set_awake_data_rate_12_5hz,
                $create,
                CTRL_REG1,
                BF::ASLP_RATE0,
                set_auto_sleep_data_rate,
                AutoSleepDataRate::Hz12_5
            );
            set_test!(
                set_awake_data_rate_6_25hz,
                $create,
                CTRL_REG1,
                BF::ASLP_RATE1,
                set_auto_sleep_data_rate,
                AutoSleepDataRate::Hz6_25
            );
            set_test!(
                set_awake_data_rate_1_56hz,
                $create,
                CTRL_REG1,
                BF::ASLP_RATE1 | BF::ASLP_RATE0,
                set_auto_sleep_data_rate,
                AutoSleepDataRate::Hz1_56
            );

            set_test!(
                set_auto_sleep_count,
                $create,
                ASLP_COUNT,
                0xAB,
                set_auto_sleep_count,
                0xAB
            );
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);
