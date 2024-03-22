use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{
    mode,
    register_access::{BitFlags, Register},
    Config, EnabledInterrupts, Error, InterruptPinConfiguration, InterruptPinPolarity,
    InterruptPinRoutes, InterruptSourcePinRoute, Mma8x5x, WakeInterrupts,
};

/// Interrupt configuration
impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// Set interrupt pin polarity
    pub fn set_interrupt_pin_polarity(
        &mut self,
        polarity: InterruptPinPolarity,
    ) -> Result<(), Error<E>> {
        let config = match polarity {
            InterruptPinPolarity::ActiveLow => self.ctrl_reg3.with_low(BitFlags::IPOL),
            InterruptPinPolarity::ActiveHigh => self.ctrl_reg3.with_high(BitFlags::IPOL),
        };
        self.write_reg(Register::CTRL_REG3, config.bits)?;
        self.ctrl_reg3 = config;
        Ok(())
    }

    /// Set interrupt pin configuration
    pub fn set_interrupt_pin_configuration(
        &mut self,
        configuration: InterruptPinConfiguration,
    ) -> Result<(), Error<E>> {
        let config = match configuration {
            InterruptPinConfiguration::PushPull => self.ctrl_reg3.with_low(BitFlags::PP_OD),
            InterruptPinConfiguration::OpenDrain => self.ctrl_reg3.with_high(BitFlags::PP_OD),
        };
        self.write_reg(Register::CTRL_REG3, config.bits)?;
        self.ctrl_reg3 = config;
        Ok(())
    }

    /// Set enabled interrupts
    pub fn set_enabled_interrupts(&mut self, enabled: EnabledInterrupts) -> Result<(), Error<E>> {
        self.write_reg(Register::CTRL_REG4, get_enabled_int_reg(enabled))
    }

    /// Set interrupt source pin routes
    pub fn set_interrupt_pin_routes(&mut self, routes: InterruptPinRoutes) -> Result<(), Error<E>> {
        self.write_reg(Register::CTRL_REG5, get_int_routes_reg(routes))
    }

    /// Set interrupts that wake the device
    pub fn set_wake_interrupts(&mut self, interrupts: WakeInterrupts) -> Result<(), Error<E>> {
        let config = self.ctrl_reg3.bits
            & !(BitFlags::WAKE_FF_MT
                | BitFlags::WAKE_LNDPRT
                | BitFlags::WAKE_PULSE
                | BitFlags::WAKE_TRANS);
        let config = config | get_wake_int_mask(interrupts);
        self.write_reg(Register::CTRL_REG3, config)?;
        self.ctrl_reg3 = Config { bits: config };
        Ok(())
    }
}

fn get_wake_int_mask(wake_ints: WakeInterrupts) -> u8 {
    (if wake_ints.transient {
        BitFlags::WAKE_TRANS
    } else {
        0
    } | if wake_ints.portrait_landscape {
        BitFlags::WAKE_LNDPRT
    } else {
        0
    } | if wake_ints.pulse {
        BitFlags::WAKE_PULSE
    } else {
        0
    } | if wake_ints.freefall_motion {
        BitFlags::WAKE_FF_MT
    } else {
        0
    })
}

fn get_int_routes_reg(routes: InterruptPinRoutes) -> u8 {
    get_int_source_pin_route_reg(routes.auto_sleep, BitFlags::INT_CFG_ASLP)
        | get_int_source_pin_route_reg(routes.fifo, BitFlags::INT_CFG_FIFO)
        | get_int_source_pin_route_reg(routes.transient, BitFlags::INT_CFG_TRANS)
        | get_int_source_pin_route_reg(routes.portrait_landscape, BitFlags::INT_CFG_LNDPRT)
        | get_int_source_pin_route_reg(routes.pulse, BitFlags::INT_CFG_PULSE)
        | get_int_source_pin_route_reg(routes.freefall_motion, BitFlags::INT_CFG_FF_MT)
        | get_int_source_pin_route_reg(routes.data_ready, BitFlags::INT_CFG_DRDY)
}

fn get_int_source_pin_route_reg(route: InterruptSourcePinRoute, flag: u8) -> u8 {
    match route {
        InterruptSourcePinRoute::Int2 => 0,
        InterruptSourcePinRoute::Int1 => flag,
    }
}

fn get_enabled_int_reg(en_int: EnabledInterrupts) -> u8 {
    (if en_int.auto_sleep {
        BitFlags::INT_EN_ASLP
    } else {
        0
    } | if en_int.fifo {
        BitFlags::INT_EN_FIFO
    } else {
        0
    } | if en_int.transient {
        BitFlags::INT_EN_TRANS
    } else {
        0
    } | if en_int.portrait_landscape {
        BitFlags::INT_EN_LNDPRT
    } else {
        0
    } | if en_int.pulse {
        BitFlags::INT_EN_PULSE
    } else {
        0
    } | if en_int.freefall_motion {
        BitFlags::INT_EN_FF_MT
    } else {
        0
    } | if en_int.data_ready {
        BitFlags::INT_EN_DRDY
    } else {
        0
    })
}

#[cfg(test)]
mod enabled_int_tests {
    use super::*;
    #[test]
    fn default() {
        assert_eq!(0, get_enabled_int_reg(EnabledInterrupts::default()));
    }

    #[test]
    fn all() {
        assert_eq!(
            BitFlags::INT_EN_ASLP
                | BitFlags::INT_EN_DRDY
                | BitFlags::INT_EN_FF_MT
                | BitFlags::INT_EN_FIFO
                | BitFlags::INT_EN_LNDPRT
                | BitFlags::INT_EN_PULSE
                | BitFlags::INT_EN_TRANS,
            get_enabled_int_reg(EnabledInterrupts {
                auto_sleep: true,
                fifo: true,
                transient: true,
                portrait_landscape: true,
                pulse: true,
                freefall_motion: true,
                data_ready: true
            })
        );
    }

    macro_rules! int_en_test {
        ($name:ident, $bit_flag:ident) => {
            #[test]
            fn $name() {
                assert_eq!(
                    BitFlags::$bit_flag,
                    get_enabled_int_reg(EnabledInterrupts {
                        $name: true,
                        ..EnabledInterrupts::default()
                    })
                );
            }
        };
    }
    int_en_test!(auto_sleep, INT_EN_ASLP);
    int_en_test!(fifo, INT_EN_FIFO);
    int_en_test!(transient, INT_EN_TRANS);
    int_en_test!(portrait_landscape, INT_EN_LNDPRT);
    int_en_test!(pulse, INT_EN_PULSE);
    int_en_test!(freefall_motion, INT_EN_FF_MT);
    int_en_test!(data_ready, INT_EN_DRDY);
}

#[cfg(test)]
mod int_routes_tests {
    use super::*;
    #[test]
    fn default() {
        assert_eq!(0, get_int_routes_reg(InterruptPinRoutes::default()));
    }

    #[test]
    fn all() {
        assert_eq!(
            BitFlags::INT_CFG_ASLP
                | BitFlags::INT_CFG_DRDY
                | BitFlags::INT_CFG_FF_MT
                | BitFlags::INT_CFG_FIFO
                | BitFlags::INT_CFG_LNDPRT
                | BitFlags::INT_CFG_PULSE
                | BitFlags::INT_CFG_TRANS,
            get_int_routes_reg(InterruptPinRoutes {
                auto_sleep: InterruptSourcePinRoute::Int1,
                fifo: InterruptSourcePinRoute::Int1,
                transient: InterruptSourcePinRoute::Int1,
                portrait_landscape: InterruptSourcePinRoute::Int1,
                pulse: InterruptSourcePinRoute::Int1,
                freefall_motion: InterruptSourcePinRoute::Int1,
                data_ready: InterruptSourcePinRoute::Int1
            })
        );
    }

    macro_rules! int_route_test {
        ($name:ident, $bit_flag:ident) => {
            #[test]
            fn $name() {
                assert_eq!(
                    BitFlags::$bit_flag,
                    get_int_routes_reg(InterruptPinRoutes {
                        $name: InterruptSourcePinRoute::Int1,
                        ..InterruptPinRoutes::default()
                    })
                );
            }
        };
    }
    int_route_test!(auto_sleep, INT_CFG_ASLP);
    int_route_test!(fifo, INT_CFG_FIFO);
    int_route_test!(transient, INT_CFG_TRANS);
    int_route_test!(portrait_landscape, INT_CFG_LNDPRT);
    int_route_test!(pulse, INT_CFG_PULSE);
    int_route_test!(freefall_motion, INT_CFG_FF_MT);
    int_route_test!(data_ready, INT_CFG_DRDY);
}

#[cfg(test)]
mod wake_int_tests {
    use super::*;
    #[test]
    fn default() {
        assert_eq!(0, get_wake_int_mask(WakeInterrupts::default()));
    }

    #[test]
    fn all() {
        assert_eq!(
            BitFlags::WAKE_FF_MT
                | BitFlags::WAKE_LNDPRT
                | BitFlags::WAKE_PULSE
                | BitFlags::WAKE_TRANS,
            get_wake_int_mask(WakeInterrupts {
                transient: true,
                portrait_landscape: true,
                pulse: true,
                freefall_motion: true,
            })
        );
    }

    macro_rules! wake_int_test {
        ($name:ident, $bit_flag:ident) => {
            #[test]
            fn $name() {
                assert_eq!(
                    BitFlags::$bit_flag,
                    get_wake_int_mask(WakeInterrupts {
                        $name: true,
                        ..WakeInterrupts::default()
                    })
                );
            }
        };
    }
    wake_int_test!(transient, WAKE_TRANS);
    wake_int_test!(portrait_landscape, WAKE_LNDPRT);
    wake_int_test!(pulse, WAKE_PULSE);
    wake_int_test!(freefall_motion, WAKE_FF_MT);
}
