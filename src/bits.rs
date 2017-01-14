//! Methods to play with bits

use pin;

/// Runs the given closure on each bit, most significant first.
///
/// Calls `f` on each bit of `data`, most significant first,
/// then calls `g` on all the results.
pub fn foreach_msb<T, F>(byte: u8, mut f: F) -> [T; 8]
    where F: FnMut(pin::PinState) -> T
{
    [f((byte & 0b10000000).into()),
     f((byte & 0b01000000).into()),
     f((byte & 0b00100000).into()),
     f((byte & 0b00010000).into()),
     f((byte & 0b00001000).into()),
     f((byte & 0b00000100).into()),
     f((byte & 0b00000010).into()),
     f((byte & 0b00000001).into())]
}

/// Runs the given closure on each bit, least significant first.
///
/// Calls `f` on each bit of `data`, least significant first,
/// then calls `g` on all the results.
pub fn foreach_lsb<T, F>(byte: u8, mut f: F) -> [T; 8]
    where F: FnMut(pin::PinState) -> T
{
    [f((byte & 0b00000001).into()),
     f((byte & 0b00000010).into()),
     f((byte & 0b00000100).into()),
     f((byte & 0b00001000).into()),
     f((byte & 0b00010000).into()),
     f((byte & 0b00100000).into()),
     f((byte & 0b01000000).into()),
     f((byte & 0b10000000).into())]
}

/// Rebuild a byte from individual bits, most significant first.
pub fn msb_to_byte(bits: [pin::PinState; 8]) -> u8 {
    (bits[0] as u8) << 7 | (bits[1] as u8) << 6 | (bits[2] as u8) << 5 |
    (bits[3] as u8) << 4 |
    (bits[4] as u8) << 3 | (bits[5] as u8) << 2 | (bits[6] as u8) << 1 |
    (bits[7] as u8)
}

/// Rebuild a byte from individual bits, least significant first.
pub fn lsb_to_byte(bits: [pin::PinState; 8]) -> u8 {
    (bits[7] as u8) << 7 | (bits[6] as u8) << 6 | (bits[5] as u8) << 5 |
    (bits[4] as u8) << 4 |
    (bits[3] as u8) << 3 | (bits[2] as u8) << 2 | (bits[1] as u8) << 1 |
    (bits[0] as u8)
}
