use crate::UnscaledMeasurement;

pub fn convert_10bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x as i16) / (1 << 6),
        y: (y as i16) / (1 << 6),
        z: (z as i16) / (1 << 6),
    }
}

pub fn convert_12bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x as i16) / (1 << 4),
        y: (y as i16) / (1 << 4),
        z: (z as i16) / (1 << 4),
    }
}

pub fn convert_14bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x as i16) / (1 << 2),
        y: (y as i16) / (1 << 2),
        z: (z as i16) / (1 << 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_positive_10bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 0x10D,
                y: 0x10E,
                z: 0x10F
            },
            convert_10bit(0x10D << 6, 0x10E << 6, 0x10F << 6)
        );
    }
    #[test]
    fn can_convert_negative_10bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -256,
                z: -512
            },
            convert_10bit(
                0b11_1111_1111 << 6,
                0b11_0000_0000 << 6,
                0b10_0000_0000 << 6
            )
        );
    }

    #[test]
    fn can_convert_positive_12bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 0x10D << 2,
                y: 0x10E << 2,
                z: 0x10F << 2
            },
            convert_12bit(0x10D << 6, 0x10E << 6, 0x10F << 6)
        );
    }

    #[test]
    fn can_convert_negative_12bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -1024,
                z: -2048
            },
            convert_12bit(
                0b1111_1111_1111 << 4,
                0b1100_0000_0000 << 4,
                0b1000_0000_0000 << 4
            )
        );
    }

    #[test]
    fn can_convert_positive_14bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: 0x10D << 4,
                y: 0x10E << 4,
                z: 0x10F << 4
            },
            convert_14bit(0x10D << 6, 0x10E << 6, 0x10F << 6)
        );
    }

    #[test]
    fn can_convert_negative_14bit() {
        assert_eq!(
            UnscaledMeasurement {
                x: -1,
                y: -4096,
                z: -8192
            },
            convert_14bit(
                0b11_1111_1111_1111 << 2,
                0b11_0000_0000_0000 << 2,
                0b10_0000_0000_0000 << 2
            )
        );
    }
}
