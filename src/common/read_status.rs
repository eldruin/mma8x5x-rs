//! MLX90614-specific functions

use crate::{
    register_access::{BitFlags as BF, Register},
    DataStatus, Error, FrontBackOrientation, Mma8x5x, PortraitLandscapeOrientation,
    PortraitLandscapeStatus, SystemMode,
};
use embedded_hal::blocking::i2c;

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
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
}
