This crate defines some generic trait useful in embedded applications.

These traits are meant to be used by driver libraries to abstract over actual IO,
and separately implemented for specific hardware.

For instance, a display driver may be generic for `<S: spi::Serial>`, and will not care
about the actual implementation of the SPI communication. On the other side, a user will
implement `spi::Serial` for his own board configuration, and give that to the driver.
