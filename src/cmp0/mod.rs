use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub c0: C0,
    #[doc = "0x04 - CMP Control Register 1"]
    pub c1: C1,
    #[doc = "0x08 - CMP Control Register 2"]
    pub c2: C2,
}
#[doc = "CMP Control Register 0"]
pub struct C0 {
    register: VolatileCell<u32>,
}
#[doc = "CMP Control Register 0"]
pub mod c0;
#[doc = "CMP Control Register 1"]
pub struct C1 {
    register: VolatileCell<u32>,
}
#[doc = "CMP Control Register 1"]
pub mod c1;
#[doc = "CMP Control Register 2"]
pub struct C2 {
    register: VolatileCell<u32>,
}
#[doc = "CMP Control Register 2"]
pub mod c2;
