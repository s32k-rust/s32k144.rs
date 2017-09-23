use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x01 - Service Register"]
    pub serv: SERV,
    #[doc = "0x02 - Compare Low Register"]
    pub cmpl: CMPL,
    #[doc = "0x03 - Compare High Register"]
    pub cmph: CMPH,
    _reserved0: [u8; 1usize],
    #[doc = "0x05 - Clock Prescaler Register"]
    pub clkprescaler: CLKPRESCALER,
}
#[doc = "Control Register"]
pub struct CTRL {
    register: VolatileCell<u8>,
}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Service Register"]
pub struct SERV {
    register: VolatileCell<u8>,
}
#[doc = "Service Register"]
pub mod serv;
#[doc = "Compare Low Register"]
pub struct CMPL {
    register: VolatileCell<u8>,
}
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "Compare High Register"]
pub struct CMPH {
    register: VolatileCell<u8>,
}
#[doc = "Compare High Register"]
pub mod cmph;
#[doc = "Clock Prescaler Register"]
pub struct CLKPRESCALER {
    register: VolatileCell<u8>,
}
#[doc = "Clock Prescaler Register"]
pub mod clkprescaler;
