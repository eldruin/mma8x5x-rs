//! MLX90614-specific functions

use crate::{
    register_access::{BitFlags as BF, Register},
    DataStatus, Error, FrontBackOrientation, InterruptStatus, Mma8x5x,
    PortraitLandscapeOrientation, PortraitLandscapeStatus, SystemMode,
};
use embedded_hal::i2c::{I2c, SevenBitAddress};

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// Read current system mode
    pub fn system_mode(&mut self) -> Result<SystemMode, Error<E>> {
        let sysmod = self.read_reg(Register::SYSMOD)?;
        match sysmod & 0b11 {
            0 => Ok(SystemMode::Standby),
            1 => Ok(SystemMode::Wake),
            _ => Ok(SystemMode::Sleep),
        }
    }

    /// Read current data status
    pub fn data_status(&mut self) -> Result<DataStatus, Error<E>> {
        let st = self.read_reg(Register::STATUS)?;
        Ok(DataStatus {
            xyz_overwrite: (st & BF::XYZOW) != 0,
            z_overwrite: (st & BF::ZOW) != 0,
            y_overwrite: (st & BF::YOW) != 0,
            x_overwrite: (st & BF::XOW) != 0,
            xyz_new_data: (st & BF::XYZDR) != 0,
            z_new_data: (st & BF::ZDR) != 0,
            y_new_data: (st & BF::YDR) != 0,
            x_new_data: (st & BF::XDR) != 0,
        })
    }

    /// Read current portrait/landscape status
    pub fn portrait_landscape_status(&mut self) -> Result<PortraitLandscapeStatus, Error<E>> {
        let st = self.read_reg(Register::PL_STATUS)?;
        Ok(get_pl_status(st))
    }

    /// Read current interrupt status
    pub fn interrupt_status(&mut self) -> Result<InterruptStatus, Error<E>> {
        let int_src = self.read_reg(Register::INT_SOURCE)?;
        Ok(get_interrupt_status(int_src))
    }
}

fn get_pl_status(pl_status: u8) -> PortraitLandscapeStatus {
    let pl = match pl_status & (BF::LAPO0 | BF::LAPO1) {
        0 => PortraitLandscapeOrientation::PortraitUp,
        BF::LAPO0 => PortraitLandscapeOrientation::PortraitDown,
        BF::LAPO1 => PortraitLandscapeOrientation::LandscapeRight,
        _ => PortraitLandscapeOrientation::LandscapeLeft,
    };
    let fb = if (pl_status & BF::BAFRO) != 0 {
        FrontBackOrientation::Back
    } else {
        FrontBackOrientation::Front
    };
    PortraitLandscapeStatus {
        something_changed: (pl_status & BF::NEWLP) != 0,
        z_tilt_angle_lookout: (pl_status & BF::LO) != 0,
        portrait_landscape: pl,
        front_back: fb,
    }
}

fn get_interrupt_status(int_src: u8) -> InterruptStatus {
    InterruptStatus {
        auto_sleep: (int_src & BF::SRC_ASLP) != 0,
        fifo: (int_src & BF::SRC_FIFO) != 0,
        transient: (int_src & BF::SRC_TRANS) != 0,
        portrait_landscape: (int_src & BF::SRC_LNDPRT) != 0,
        pulse: (int_src & BF::SRC_PULSE) != 0,
        freefall_motion: (int_src & BF::SRC_FF_MT) != 0,
        data_ready: (int_src & BF::SRC_DRDY) != 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pl_status_default() {
        assert_eq!(PortraitLandscapeStatus::default(), get_pl_status(0));
    }

    #[test]
    fn pl_status_unaffected_by_others() {
        assert_eq!(
            PortraitLandscapeStatus::default(),
            get_pl_status(!(BF::NEWLP | BF::LO | BF::LAPO1 | BF::LAPO0 | BF::BAFRO))
        );
    }

    #[test]
    fn pl_status_sth_changed() {
        assert_eq!(
            PortraitLandscapeStatus {
                something_changed: true,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::NEWLP)
        );
    }

    #[test]
    fn pl_status_z_tilt() {
        assert_eq!(
            PortraitLandscapeStatus {
                z_tilt_angle_lookout: true,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::LO)
        );
    }

    #[test]
    fn pl_status_back() {
        assert_eq!(
            PortraitLandscapeStatus {
                front_back: FrontBackOrientation::Back,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::BAFRO)
        );
    }

    #[test]
    fn pl_status_portrait_down() {
        assert_eq!(
            PortraitLandscapeStatus {
                portrait_landscape: PortraitLandscapeOrientation::PortraitDown,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::LAPO0)
        );
    }

    #[test]
    fn pl_status_landscape_right() {
        assert_eq!(
            PortraitLandscapeStatus {
                portrait_landscape: PortraitLandscapeOrientation::LandscapeRight,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::LAPO1)
        );
    }

    #[test]
    fn pl_status_landscape_left() {
        assert_eq!(
            PortraitLandscapeStatus {
                portrait_landscape: PortraitLandscapeOrientation::LandscapeLeft,
                ..PortraitLandscapeStatus::default()
            },
            get_pl_status(BF::LAPO1 | BF::LAPO0)
        );
    }

    mod int_status {
        use super::*;
        #[test]
        fn int_status_default() {
            assert_eq!(InterruptStatus::default(), get_interrupt_status(0));
        }

        #[test]
        fn int_status_unaffected_by_others() {
            assert_eq!(
                InterruptStatus::default(),
                get_interrupt_status(
                    !(BF::SRC_ASLP
                        | BF::SRC_DRDY
                        | BF::SRC_FF_MT
                        | BF::SRC_FIFO
                        | BF::SRC_LNDPRT
                        | BF::SRC_PULSE
                        | BF::SRC_TRANS)
                )
            );
        }

        #[test]
        fn int_status_all() {
            assert_eq!(
                InterruptStatus {
                    auto_sleep: true,
                    fifo: true,
                    transient: true,
                    portrait_landscape: true,
                    pulse: true,
                    freefall_motion: true,
                    data_ready: true
                },
                get_interrupt_status(
                    BF::SRC_ASLP
                        | BF::SRC_DRDY
                        | BF::SRC_FF_MT
                        | BF::SRC_FIFO
                        | BF::SRC_LNDPRT
                        | BF::SRC_PULSE
                        | BF::SRC_TRANS
                )
            );
        }

        macro_rules! int_status_test {
            ($name:ident, $bit_flag:ident) => {
                #[test]
                fn $name() {
                    assert_eq!(
                        InterruptStatus {
                            $name: true,
                            ..InterruptStatus::default()
                        },
                        get_interrupt_status(BF::$bit_flag)
                    );
                }
            };
        }
        int_status_test!(auto_sleep, SRC_ASLP);
        int_status_test!(fifo, SRC_FIFO);
        int_status_test!(transient, SRC_TRANS);
        int_status_test!(portrait_landscape, SRC_LNDPRT);
        int_status_test!(pulse, SRC_PULSE);
        int_status_test!(freefall_motion, SRC_FF_MT);
        int_status_test!(data_ready, SRC_DRDY);
    }
}
