use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"] pub sc: SC,
    #[doc = "0x04 - Modulus register"] pub mod_: MOD,
    #[doc = "0x08 - Counter register"] pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay register"] pub idly: IDLY,
    #[doc = "0x10 - Channel n Control register 1"] pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status register"] pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 register"] pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 register"] pub ch0dly1: CHDLY1,
    #[doc = "0x20 - Channel n Delay 2 register"] pub ch0dly2: CHDLY2,
    #[doc = "0x24 - Channel n Delay 3 register"] pub ch0dly3: CHDLY3,
    #[doc = "0x28 - Channel n Delay 4 register"] pub ch0dly4: CHDLY4,
    #[doc = "0x2c - Channel n Delay 5 register"] pub ch0dly5: CHDLY5,
    #[doc = "0x30 - Channel n Delay 6 register"] pub ch0dly6: CHDLY6,
    #[doc = "0x34 - Channel n Delay 7 register"] pub ch0dly7: CHDLY7,
    #[doc = "0x38 - Channel n Control register 1"] pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status register"] pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 register"] pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 register"] pub ch1dly1: CHDLY1,
    #[doc = "0x48 - Channel n Delay 2 register"] pub ch1dly2: CHDLY2,
    #[doc = "0x4c - Channel n Delay 3 register"] pub ch1dly3: CHDLY3,
    #[doc = "0x50 - Channel n Delay 4 register"] pub ch1dly4: CHDLY4,
    #[doc = "0x54 - Channel n Delay 5 register"] pub ch1dly5: CHDLY5,
    #[doc = "0x58 - Channel n Delay 6 register"] pub ch1dly6: CHDLY6,
    #[doc = "0x5c - Channel n Delay 7 register"] pub ch1dly7: CHDLY7,
    _reserved0: [u8; 304usize],
    #[doc = "0x190 - Pulse-Out n Enable register"] pub poen: POEN,
    #[doc = "0x194 - Pulse-Out n Delay register"] pub podly: PODLY,
}
#[doc = "Status and Control register"]
pub struct SC {
    register: VolatileCell<u32>,
}
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "Modulus register"]
pub struct MOD {
    register: VolatileCell<u32>,
}
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "Counter register"]
pub struct CNT {
    register: VolatileCell<u32>,
}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Interrupt Delay register"]
pub struct IDLY {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "Channel n Control register 1"]
pub struct CHC1 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "Channel n Status register"]
pub struct CHS {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "Channel n Delay 0 register"]
pub struct CHDLY0 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "Channel n Delay 1 register"]
pub struct CHDLY1 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "Channel n Delay 2 register"]
pub struct CHDLY2 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 2 register"]
pub mod chdly2;
#[doc = "Channel n Delay 3 register"]
pub struct CHDLY3 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 3 register"]
pub mod chdly3;
#[doc = "Channel n Delay 4 register"]
pub struct CHDLY4 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 4 register"]
pub mod chdly4;
#[doc = "Channel n Delay 5 register"]
pub struct CHDLY5 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 5 register"]
pub mod chdly5;
#[doc = "Channel n Delay 6 register"]
pub struct CHDLY6 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 6 register"]
pub mod chdly6;
#[doc = "Channel n Delay 7 register"]
pub struct CHDLY7 {
    register: VolatileCell<u32>,
}
#[doc = "Channel n Delay 7 register"]
pub mod chdly7;
#[doc = "Pulse-Out n Enable register"]
pub struct POEN {
    register: VolatileCell<u32>,
}
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "Pulse-Out n Delay register"]
pub struct PODLY {
    register: VolatileCell<u32>,
}
#[doc = "Pulse-Out n Delay register"]
pub mod podly;
#[doc = "PDB0_DLY2 register."]
pub struct DLY2 {
    register: VolatileCell<u16>,
}
#[doc = "PDB0_DLY2 register."]
pub mod dly2;
#[doc = "PDB0_DLY1 register."]
pub struct DLY1 {
    register: VolatileCell<u16>,
}
#[doc = "PDB0_DLY1 register."]
pub mod dly1;
