mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::InterruptStatus;

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            get_test!(
                nothing,
                $create,
                INT_SOURCE,
                0,
                interrupt_status,
                InterruptStatus::default()
            );
            get_test!(
                all,
                $create,
                INT_SOURCE,
                BF::SRC_ASLP
                    | BF::SRC_DRDY
                    | BF::SRC_FF_MT
                    | BF::SRC_FIFO
                    | BF::SRC_LNDPRT
                    | BF::SRC_PULSE
                    | BF::SRC_TRANS,
                interrupt_status,
                InterruptStatus {
                    auto_sleep: true,
                    fifo: true,
                    transient: true,
                    portrait_landscape: true,
                    pulse: true,
                    freefall_motion: true,
                    data_ready: true
                }
            );
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);
