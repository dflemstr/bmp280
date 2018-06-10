#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Register {
    DigT1 = 0x88,
    DigT2 = 0x8a,
    DigT3 = 0x8c,

    DigP1 = 0x8e,
    DigP2 = 0x90,
    DigP3 = 0x92,
    DigP4 = 0x94,
    DigP5 = 0x96,
    DigP6 = 0x98,
    DigP7 = 0x9a,
    DigP8 = 0x9c,
    DigP9 = 0x9e,

    ChipId = 0xd0,
    Version = 0xd1,
    SoftReset = 0xe0,

    Cal26 = 0xe1,

    Control = 0xf4,
    Config = 0xf5,
    PressureData = 0xf7,
    TempData = 0xfa,
}
