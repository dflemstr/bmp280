//! Definitions for BMP280 I2C registers.

/// I2C registers present in a BMP280.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Register {
    /// Digital temperature coefficient 1.
    DigT1 = 0x88,
    /// Digital temperature coefficient 1.
    DigT2 = 0x8a,
    /// Digital temperature coefficient 1.
    DigT3 = 0x8c,

    /// Digital pressure coefficient 1.
    DigP1 = 0x8e,
    /// Digital pressure coefficient 2.
    DigP2 = 0x90,
    /// Digital pressure coefficient 3.
    DigP3 = 0x92,
    /// Digital pressure coefficient 4.
    DigP4 = 0x94,
    /// Digital pressure coefficient 5.
    DigP5 = 0x96,
    /// Digital pressure coefficient 6.
    DigP6 = 0x98,
    /// Digital pressure coefficient 7.
    DigP7 = 0x9a,
    /// Digital pressure coefficient 8.
    DigP8 = 0x9c,
    /// Digital pressure coefficient 9.
    DigP9 = 0x9e,

    /// Contains the chip identifier.
    ChipId = 0xd0,
    /// Contains the chip version.
    Version = 0xd1,
    /// Write to reset the chip.
    SoftReset = 0xe0,

    /// R calibration
    Cal26 = 0xe1,

    /// Control register.
    Control = 0xf4,
    /// Configuration register.
    Config = 0xf5,
    /// 20-bit register containing pressure data.
    PressureData = 0xf7,
    /// 20-bit register containing temperature data.
    TempData = 0xfa,
}
