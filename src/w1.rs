//! 1-Wire Dallas protocol.
use pin;
use bits;

pub trait Master {
    // ?
}

pub const DELAY_SHORT_US: u32 = 10;
pub const DELAY_MID_US: u32 = 30;
pub const DELAY_LONG_US: u32 = 60;
pub const DELAY_RESET_US: u32 = 500;


pub struct BitBanging<W, Sleep>
    where W: pin::Output + pin::Input,
          Sleep: FnMut(u32)
{
    w: W,
    sleep: Sleep,
}

pub struct Enumeration<'a, W, Sleep>
    where W: 'a + pin::Output + pin::Input,
          Sleep: 'a + FnMut(u32)
{
    w1: &'a mut BitBanging<W, Sleep>,
    last_id: u8,
}


impl<W, Sleep> BitBanging<W, Sleep>
    where W: pin::Output + pin::Input,
          Sleep: FnMut(u32)
{
    pub fn new(w: W, sleep: Sleep) -> Self {
        BitBanging {
            w: w,
            sleep: sleep,
        }
    }
}

#[repr(u8)]
pub enum Command {
    Search = 0xF0,
}

impl<W, Sleep> Master for BitBanging<W, Sleep>
    where W: pin::Output + pin::Input,
          Sleep: FnMut(u32)
{
    fn enumerate(&mut self) {}

    fn reset(&mut self) -> bool {
        self.w.low();
        (self.sleep)(DELAY_RESET_US);
        self.w.high();
        (self.sleep)(DELAY_MID_US);

        self.w.read().is_low()
    }

    fn write_bit(&mut self, bit: pin::PinState) {
        self.w.low();
        // DELAY depending on bit
        (self.sleep)(match bit {
            pin::PinState::Low => DELAY_SHORT_US,
            pin::PinState::High => DELAY_LONG_US,
        });
        self.w.high();
    }

    fn read_bit(&mut self) -> pin::PinState {
        self.w.low();
        (self.sleep)(DELAY_SHORT_US);
        self.w.high();
        (self.sleep)(DELAY_MID_US);

        self.w.read()
    }
}
