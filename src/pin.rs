/// An output GPIO pin.
pub trait Output {
    fn high(&mut self);
    fn low(&mut self);
}

/// An input GPIO pib.
pub trait Input {
    /// Returns `TRUE` if the pin is high.
    fn is_high(&mut self) -> bool;
}

impl<U, L> Output for (U, L)
    where U: FnMut(),
          L: FnMut()
{
    fn high(&mut self) {
        self.0()
    }

    fn low(&mut self) {
        self.1()
    }
}

impl<F> Input for F
    where F: FnMut() -> bool
{
    fn is_high(&mut self) -> bool {
        (*self)()
    }
}

impl Input for bool {
    fn is_high(&mut self) -> bool {
        *self
    }
}

/// Dummy pin, dropping input and reading 0.
pub struct DummyPin;

impl Output for DummyPin {
    fn high(&mut self) {}
    fn low(&mut self) {}
}

impl Input for DummyPin {
    /// Always return false
    fn is_high(&mut self) -> bool {
        false
    }
}


#[cfg(test)]
pub struct DebugPin {
    name: ::std::string::String,
}

#[cfg(test)]
use ::std::string::ToString;

#[cfg(test)]
impl DebugPin {
    pub fn new<S: ToString>(name: S) -> Self {
        DebugPin { name: name.to_string() }
    }
}

#[cfg(test)]
impl Output for DebugPin {
    fn high(&mut self) {
        println!("{} HIGH", self.name);
    }

    fn low(&mut self) {
        println!("{} LOW", self.name);
    }
}
