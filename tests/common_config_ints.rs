mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;
use mma8x5x::{EnabledInterrupts, InterruptPinConfiguration, InterruptPinPolarity, WakeInterrupts};

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;
            set_test!(
                set_int_pin_pol_active_low,
                $create,
                CTRL_REG3,
                0,
                set_interrupt_pin_polarity,
                InterruptPinPolarity::ActiveLow
            );
            set_test!(
                set_int_pin_pol_active_high,
                $create,
                CTRL_REG3,
                BF::IPOL,
                set_interrupt_pin_polarity,
                InterruptPinPolarity::ActiveHigh
            );

            set_test!(
                set_int_pin_config_push_pull,
                $create,
                CTRL_REG3,
                0,
                set_interrupt_pin_configuration,
                InterruptPinConfiguration::PushPull
            );
            set_test!(
                set_int_pin_config_open_drain,
                $create,
                CTRL_REG3,
                BF::PP_OD,
                set_interrupt_pin_configuration,
                InterruptPinConfiguration::OpenDrain
            );
            set_test!(
                set_enabled_interrupts_all,
                $create,
                CTRL_REG4,
                BF::INT_EN_ASLP
                    | BF::INT_EN_DRDY
                    | BF::INT_EN_FF_MT
                    | BF::INT_EN_FIFO
                    | BF::INT_EN_LNDPRT
                    | BF::INT_EN_PULSE
                    | BF::INT_EN_TRANS,
                set_enabled_interrupts,
                EnabledInterrupts {
                    auto_sleep: true,
                    fifo: true,
                    transient: true,
                    portrait_landscape: true,
                    pulse: true,
                    freefall_motion: true,
                    data_ready: true
                }
            );
            set_test!(
                set_wake_interrupts_all,
                $create,
                CTRL_REG3,
                BF::WAKE_FF_MT | BF::WAKE_LNDPRT | BF::WAKE_PULSE | BF::WAKE_TRANS,
                set_wake_interrupts,
                WakeInterrupts {
                    transient: true,
                    portrait_landscape: true,
                    pulse: true,
                    freefall_motion: true,
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
