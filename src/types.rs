/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// IC markers
pub mod ic {
    /// MMA8653 IC marker
    pub struct Mma8653;
}
