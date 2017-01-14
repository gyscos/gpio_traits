//! Inter-Integrated Circuit (I²C).

use pin;

/// A I²C device
pub trait I2C {
    fn start(&mut self);
    fn stop(&mut self);
}

pub struct BitBanging<SCL, SDA>
    where SCL: pin::Output,
          SDA: pin::Output + pin::Input
{
    scl: SCL,
    sda: SDA,
}

impl<SCL, SDA> I2C for BitBanging<SCL, SDA>
    where SCL: pin::Output,
          SDA: pin::Output + pin::Input
{
    fn start(&mut self) {
        self.sda.high();
        self.scl.high();
        self.sda.low();
        self.scl.low();
    }

    fn stop(&mut self) {
        self.sda.low();
        self.scl.high();
        self.sda.high();
        self.scl.low();
    }
}
