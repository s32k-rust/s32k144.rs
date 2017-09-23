use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ERM Configuration Register 0"]
    pub cr0: CR0,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - ERM Status Register 0"]
    pub sr0: SR0,
    _reserved1: [u8; 236usize],
    #[doc = "0x100 - ERM Memory n Error Address Register"]
    pub ear0: EAR0,
    _reserved2: [u8; 12usize],
    #[doc = "0x110 - ERM Memory n Error Address Register"]
    pub ear1: EAR1,
}
#[doc = "ERM Configuration Register 0"]
pub struct CR0 {
    register: VolatileCell<u32>,
}
#[doc = "ERM Configuration Register 0"]
pub mod cr0;
#[doc = "ERM Status Register 0"]
pub struct SR0 {
    register: VolatileCell<u32>,
}
#[doc = "ERM Status Register 0"]
pub mod sr0;
#[doc = "ERM Memory n Error Address Register"]
pub struct EAR0 {
    register: VolatileCell<u32>,
}
#[doc = "ERM Memory n Error Address Register"]
pub mod ear0;
#[doc = "ERM Memory n Error Address Register"]
pub struct EAR1 {
    register: VolatileCell<u32>,
}
#[doc = "ERM Memory n Error Address Register"]
pub mod ear1;
