use crate::UnscaledMeasurement;

pub fn convert_10bit(x: u16, y: u16, z: u16) -> UnscaledMeasurement {
    UnscaledMeasurement {
        x: (x as i16) / (1 << 6),
        y: (y as i16) / (1 << 6),
        z: (z as i16) / (1 << 6),
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
}
