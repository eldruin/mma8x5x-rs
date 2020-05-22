mod base;
use crate::base::{
    destroy, new_mma8451, new_mma8452, new_mma8453, new_mma8652, new_mma8653, BitFlags as BF,
    Register, ADDRESS,
};
use embedded_hal_mock::i2c::Transaction as I2cTrans;
use mma8x5x::DataStatus;

macro_rules! get_data_status {
    ($name:ident, $create:ident, $bit_flag:ident, $status_flag:ident) => {
        get_test!(
            $name,
            $create,
            STATUS,
            BF::$bit_flag,
            data_status,
            DataStatus {
                $status_flag: true,
                ..DataStatus::default()
            }
        );
    };
}

macro_rules! tests {
    ($name:ident, $create:ident) => {
        mod $name {
            use super::*;

            get_test!(
                nothing,
                $create,
                STATUS,
                0,
                data_status,
                DataStatus::default()
            );
            get_test!(
                all,
                $create,
                STATUS,
                BF::XYZOW | BF::ZOW | BF::YOW | BF::XOW | BF::XYZDR | BF::ZDR | BF::YDR | BF::XDR,
                data_status,
                DataStatus {
                    xyz_overwrite: true,
                    z_overwrite: true,
                    y_overwrite: true,
                    x_overwrite: true,
                    xyz_new_data: true,
                    z_new_data: true,
                    y_new_data: true,
                    x_new_data: true,
                }
            );
            get_data_status!(xyzow, $create, XYZOW, xyz_overwrite);
            get_data_status!(xow, $create, XOW, x_overwrite);
            get_data_status!(yow, $create, YOW, y_overwrite);
            get_data_status!(zow, $create, ZOW, z_overwrite);
            get_data_status!(xyzdr, $create, XYZDR, xyz_new_data);
            get_data_status!(xdr, $create, XDR, x_new_data);
            get_data_status!(ydr, $create, YDR, y_new_data);
            get_data_status!(zdr, $create, ZDR, z_new_data);
        }
    };
}

tests!(mma8451, new_mma8451);
tests!(mma8452, new_mma8452);
tests!(mma8453, new_mma8453);
tests!(mma8652, new_mma8652);
tests!(mma8653, new_mma8653);
