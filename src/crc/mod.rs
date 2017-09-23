use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Data register"] pub data: DATA,
    #[doc = "0x04 - CRC Polynomial register"] pub gpoly: GPOLY,
    #[doc = "0x08 - CRC Control register"] pub ctrl: CTRL,
}
#[doc = "CRC Data register"]
pub struct DATA {
    register: VolatileCell<u32>,
}
#[doc = "CRC Data register"]
pub mod data;
#[doc = "CRC_DATAL register."]
pub struct DATAL {
    register: VolatileCell<u16>,
}
#[doc = "CRC_DATAL register."]
pub mod datal;
#[doc = "CRC_DATALL register."]
pub struct DATALL {
    register: VolatileCell<u8>,
}
#[doc = "CRC_DATALL register."]
pub mod datall;
#[doc = "CRC_DATALU register."]
pub struct DATALU {
    register: VolatileCell<u8>,
}
#[doc = "CRC_DATALU register."]
pub mod datalu;
#[doc = "CRC_DATAH register."]
pub struct DATAH {
    register: VolatileCell<u16>,
}
#[doc = "CRC_DATAH register."]
pub mod datah;
#[doc = "CRC_DATAHL register."]
pub struct DATAHL {
    register: VolatileCell<u8>,
}
#[doc = "CRC_DATAHL register."]
pub mod datahl;
#[doc = "CRC_DATAHU register."]
pub struct DATAHU {
    register: VolatileCell<u8>,
}
#[doc = "CRC_DATAHU register."]
pub mod datahu;
#[doc = "CRC Polynomial register"]
pub struct GPOLY {
    register: VolatileCell<u32>,
}
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CRC Control register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "CRC Control register"]
pub mod ctrl;
