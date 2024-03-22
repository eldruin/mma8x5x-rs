mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::DebounceCounterMode;

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            set_test!(
                enable_pl,
                $create,
                PL_CFG,
                BF::PL_EN | BF::DBCNTM,
                enable_portrait_landscape_detection
            );
            set_test!(
                disable_pl,
                $create,
                PL_CFG,
                BF::DBCNTM,
                disable_portrait_landscape_detection
            );

            set_test!(
                set_debounce_counter_dec,
                $create,
                PL_CFG,
                0,
                set_debounce_counter_mode,
                DebounceCounterMode::Decrement
            );
            set_test!(
                set_debounce_counter_clear,
                $create,
                PL_CFG,
                BF::DBCNTM,
                set_debounce_counter_mode,
                DebounceCounterMode::Clear
            );
            set_test!(
                set_debounce_counter,
                $create,
                PL_COUNT,
                0xAB,
                set_debounce_counter,
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
