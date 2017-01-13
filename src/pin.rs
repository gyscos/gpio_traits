/// Possible state for a pin
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum PinState {
    Low = 0,
    High = 1,
}

/// An output GPIO pin.
pub trait Output {
    fn high(&mut self);
    fn low(&mut self);

    fn write(&mut self, bit: PinState) {
        if bit.is_high() {
            self.high();
        } else {
            self.low();
        }
    }
}

/// An input GPIO pin.
pub trait Input {
    fn read(&mut self) -> PinState;
}


impl PinState {
    pub fn is_high(&self) -> bool {
        *self as u8 != 0
    }

    pub fn is_low(&self) -> bool {
        *self as u8 == 0
    }

    pub fn not(&self) -> PinState {
        match *self {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}

impl From<u8> for PinState {
    fn from(bit: u8) -> Self {
        if bit != 0 {
            PinState::High
        } else {
            PinState::Low
        }
    }
}

impl From<bool> for PinState {
    fn from(bit: bool) -> Self {
        if bit { PinState::High } else { PinState::Low }
    }
}

impl<'a, O: Output> Output for &'a mut O {
    fn high(&mut self) {
        (**self).high();
    }

    fn low(&mut self) {
        (**self).low();
    }
}

impl<H, L> Output for (H, L)
    where H: FnMut(),
          L: FnMut()
{
    fn high(&mut self) {
        self.0()
    }

    fn low(&mut self) {
        self.1()
    }
}

/// Inverse any input or output on the wrapped pin.
pub struct Not<P>(pub P);

impl<O: Output> Output for Not<O> {
    fn high(&mut self) {
        self.0.low();
    }

    fn low(&mut self) {
        self.0.high()
    }
}

impl<I: Input> Input for Not<I> {
    fn read(&mut self) -> PinState {
        self.0.read().not()
    }
}

/// Uses an inner `FnMut` to read
pub struct InputFn<F>(pub F) where F: FnMut() -> PinState;
/// Uses an inner `FnMut` to write the state
pub struct OutputFn<F>(pub F) where F: FnMut(PinState);

impl<F> Input for InputFn<F>
    where F: FnMut() -> PinState
{
    fn read(&mut self) -> PinState {
        (self.0)()
    }
}

impl<F> Output for OutputFn<F>
    where F: FnMut(PinState)
{
    fn high(&mut self) {
        (self.0)(PinState::High);
    }

    fn low(&mut self) {
        (self.0)(PinState::Low);
    }
}

impl Output for PinState {
    fn high(&mut self) {
        *self = PinState::High;
    }

    fn low(&mut self) {
        *self = PinState::Low;
    }
}

impl Output for bool {
    fn high(&mut self) {
        *self = true;
    }

    fn low(&mut self) {
        *self = false;
    }
}

/// Dummy pin, dropping input and reading LOW.
pub struct Dummy;

/// Always read High
pub struct HighPin;

/// Always read Low
pub struct LowPin;

impl Output for Dummy {
    fn high(&mut self) {}
    fn low(&mut self) {}
}

impl Input for Dummy {
    /// Always return false
    fn read(&mut self) -> PinState {
        PinState::Low
    }
}

impl Input for HighPin {
    fn read(&mut self) -> PinState {
        PinState::High
    }
}

impl Input for LowPin {
    fn read(&mut self) -> PinState {
        PinState::Low
    }
}

#[cfg(feature = "debug")]
pub struct DebugPin {
    name: ::std::string::String,
}

#[cfg(feature = "debug")]
use ::std::string::ToString;

#[cfg(feature = "debug")]
impl DebugPin {
    pub fn new<S: ToString>(name: S) -> Self {
        DebugPin { name: name.to_string() }
    }
}

#[cfg(feature = "debug")]
impl Output for DebugPin {
    fn high(&mut self) {
        println!("{} HIGH", self.name);
    }

    fn low(&mut self) {
        println!("{} LOW", self.name);
    }
}
