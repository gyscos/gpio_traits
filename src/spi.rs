use pin;

// TODO: handle baudrate configuration?
// TODO: handle read?
/// A SPI device
pub trait Serial {
    /// Simultaneously read and write.
    fn write(&mut self, data: u8) -> u8;
}

/// Dummy SPI device
pub struct DummySerial;

impl Serial for DummySerial {
    /// Ignores input and always return `0`.
    fn write(&mut self, _: u8) -> u8 { 0 }
}


#[cfg(feature = "debug")]
/// Debug SPI device that prints input.
pub struct DebugSerial;

#[cfg(feature = "debug")]
impl Serial for DebugSerial {
    /// Prints input and returns 0.
    fn write(&mut self, data: u8) -> u8 {
        println!("SPI: {:08b}", data);
        0
    }
}

// TODO: handle sleep between clock cycles
/// A bit-banging implementation of SPI on gpio pins.
pub struct BitBangingSerial<SCK, SS, MOSI, MISO>
    where SCK: pin::Output,
          SS: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input
{
    sck: SCK,
    ss: SS,
    mosi: MOSI,
    miso: MISO,
}

impl<SCK, SS, MOSI, MISO> BitBangingSerial<SCK, SS, MOSI, MISO>
    where SCK: pin::Output,
          SS: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input,

{
    /// Creates a bit-banging SPI device using the given pins.
    ///
    /// * `mosi`: Master Out, Slave In
    /// * `miso`: Master In, Slave Out
    pub fn new(sck: SCK, ss: SS, mosi: MOSI, miso: MISO) -> Self {
        BitBangingSerial {
            sck: sck,
            ss: ss,
            mosi: mosi,
            miso: miso,
        }
    }

    fn write_bit(&mut self, bit: bool) -> pin::PinState {
        self.sck.low();
        self.mosi.write(bit.into());
        self.sck.high();
        self.miso.read()
    }
}

impl<SCK, SS, MOSI, MISO> Serial for BitBangingSerial<SCK, SS, MOSI, MISO>
    where SCK: pin::Output,
          SS: pin::Output,
          MOSI: pin::Output,
          MISO: pin::Input,
{
    fn write(&mut self, data: u8) -> u8 {
        self.ss.low();

        let result = ((self.write_bit((data & 0b10000000) != 0) as u8) << 7) |
        ((self.write_bit((data & 0b01000000) != 0) as u8) << 6) |
        ((self.write_bit((data & 0b00100000) != 0) as u8) << 5) |
        ((self.write_bit((data & 0b00010000) != 0) as u8) << 4) |
        ((self.write_bit((data & 0b00001000) != 0) as u8) << 3) |
        ((self.write_bit((data & 0b00000100) != 0) as u8) << 2) |
        ((self.write_bit((data & 0b00000010) != 0) as u8) << 1) |
        (self.write_bit((data & 0b00000001) != 0) as u8);

        self.ss.high();

        result
    }
}
