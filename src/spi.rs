//! Serial Peripheral Interface (SPI).

use pin;
use bits;

/// A SPI device
pub trait Master {
    // TODO: write bits?
    // though most SPI implementations I found were also limited to bytes.

    // TODO: Write &mut [u8]?
    /// Simultaneously read and write.
    fn read_write(&mut self, data: u8) -> u8;
}

/// Dummy SPI device
pub struct Dummy;

impl Master for Dummy {
    /// Ignores input and always return `0`.
    fn read_write(&mut self, _: u8) -> u8 {
        0
    }
}


#[cfg(feature = "debug")]
/// Debug SPI device that prints input.
pub struct Debug;

#[cfg(feature = "debug")]
impl Master for Debug {
    /// Prints input and returns 0.
    fn read_write(&mut self, data: u8) -> u8 {
        println!("SPI: {:08b}", data);
        0
    }
}

// TODO: handle baudrate configuration?
// TODO: handle sleep between clock cycles
/// A bit-banging implementation of SPI on gpio pins.
pub struct BitBanging<SCK, MOSI, MISO>
    where SCK: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input
{
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
}

impl<SCK, MOSI, MISO> BitBanging<SCK, MOSI, MISO>
    where SCK: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input
{
    /// Creates a bit-banging SPI device using the given pins.
    ///
    /// * `mosi`: Master Out, Slave In
    /// * `miso`: Master In, Slave Out
    pub fn new(sck: SCK, mosi: MOSI, miso: MISO) -> Self {
        BitBanging {
            sck: sck,
            mosi: mosi,
            miso: miso,
        }
    }

    fn write_bit(&mut self, bit: pin::PinState) -> pin::PinState {
        self.sck.low();
        self.mosi.write(bit);
        // DELAY
        self.sck.high();
        self.miso.read()
        // DELAY?
    }
}

impl<SCK, MOSI, MISO> Master for BitBanging<SCK, MOSI, MISO>
    where SCK: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input
{
    fn read_write(&mut self, data: u8) -> u8 {

        bits::msb_to_byte(bits::foreach_msb(data, |bit| self.write_bit(bit)))
    }
}
