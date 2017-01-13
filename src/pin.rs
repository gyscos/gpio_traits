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

impl <'a, O: Output> Output for &'a mut O {
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
pub struct Not<P> (pub P);

impl <O: Output> Output for Not<O> {
    fn high(&mut self) {
        self.0.low();
    }

    fn low(&mut self) {
        self.0.high()
    }
}

impl <I: Input> Input for Not<I> {
    fn is_high(&mut self) -> bool {
        !self.0.is_high()
    }
}

pub struct InputFn<F: FnMut() -> bool> (pub F);

impl<F> Input for InputFn<F>
    where F: FnMut() -> bool
{
    fn is_high(&mut self) -> bool {
        (self.0)()
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
