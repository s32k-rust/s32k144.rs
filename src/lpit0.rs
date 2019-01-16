#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x0c - Module Status Register"]
    pub msr: MSR,
    #[doc = "0x10 - Module Interrupt Enable Register"]
    pub mier: MIER,
    #[doc = "0x14 - Set Timer Enable Register"]
    pub setten: SETTEN,
    #[doc = "0x18 - Clear Timer Enable Register"]
    pub clrten: CLRTEN,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Timer Value Register"]
    pub tval0: TVAL0,
    #[doc = "0x24 - Current Timer Value"]
    pub cval0: CVAL0,
    #[doc = "0x28 - Timer Control Register"]
    pub tctrl0: TCTRL0,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - Timer Value Register"]
    pub tval1: TVAL1,
    #[doc = "0x34 - Current Timer Value"]
    pub cval1: CVAL1,
    #[doc = "0x38 - Timer Control Register"]
    pub tctrl1: TCTRL1,
    _reserved2: [u8; 4usize],
    #[doc = "0x40 - Timer Value Register"]
    pub tval2: TVAL2,
    #[doc = "0x44 - Current Timer Value"]
    pub cval2: CVAL2,
    #[doc = "0x48 - Timer Control Register"]
    pub tctrl2: TCTRL2,
    _reserved3: [u8; 4usize],
    #[doc = "0x50 - Timer Value Register"]
    pub tval3: TVAL3,
    #[doc = "0x54 - Current Timer Value"]
    pub cval3: CVAL3,
    #[doc = "0x58 - Timer Control Register"]
    pub tctrl3: TCTRL3,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Module Control Register"]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "Module Status Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Status Register"]
pub mod msr;
#[doc = "Module Interrupt Enable Register"]
pub struct MIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Module Interrupt Enable Register"]
pub mod mier;
#[doc = "Set Timer Enable Register"]
pub struct SETTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set Timer Enable Register"]
pub mod setten;
#[doc = "Clear Timer Enable Register"]
pub struct CLRTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear Timer Enable Register"]
pub mod clrten;
#[doc = "Timer Value Register"]
pub struct TVAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value Register"]
pub mod tval0;
#[doc = "Current Timer Value"]
pub struct CVAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Timer Value"]
pub mod cval0;
#[doc = "Timer Control Register"]
pub struct TCTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod tctrl0;
#[doc = "Timer Value Register"]
pub struct TVAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value Register"]
pub mod tval1;
#[doc = "Current Timer Value"]
pub struct CVAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Timer Value"]
pub mod cval1;
#[doc = "Timer Control Register"]
pub struct TCTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod tctrl1;
#[doc = "Timer Value Register"]
pub struct TVAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value Register"]
pub mod tval2;
#[doc = "Current Timer Value"]
pub struct CVAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Timer Value"]
pub mod cval2;
#[doc = "Timer Control Register"]
pub struct TCTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod tctrl2;
#[doc = "Timer Value Register"]
pub struct TVAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Value Register"]
pub mod tval3;
#[doc = "Current Timer Value"]
pub struct CVAL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Timer Value"]
pub mod cval3;
#[doc = "Timer Control Register"]
pub struct TCTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod tctrl3;
