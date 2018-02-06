use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    pub pdor: PDOR,
    #[doc = "0x04 - Port Set Output Register"]
    pub psor: PSOR,
    #[doc = "0x08 - Port Clear Output Register"]
    pub pcor: PCOR,
    #[doc = "0x0c - Port Toggle Output Register"]
    pub ptor: PTOR,
    #[doc = "0x10 - Port Data Input Register"]
    pub pdir: PDIR,
    #[doc = "0x14 - Port Data Direction Register"]
    pub pddr: PDDR,
    #[doc = "0x18 - Port Input Disable Register"]
    pub pidr: PIDR,
}
#[doc = "Port Data Output Register"]
pub struct PDOR {
    register: VolatileCell<u32>,
}
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "Port Set Output Register"]
pub struct PSOR {
    register: VolatileCell<u32>,
}
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "Port Clear Output Register"]
pub struct PCOR {
    register: VolatileCell<u32>,
}
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "Port Toggle Output Register"]
pub struct PTOR {
    register: VolatileCell<u32>,
}
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "Port Data Input Register"]
pub struct PDIR {
    register: VolatileCell<u32>,
}
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "Port Data Direction Register"]
pub struct PDDR {
    register: VolatileCell<u32>,
}
#[doc = "Port Data Direction Register"]
pub mod pddr;
#[doc = "Port Input Disable Register"]
pub struct PIDR {
    register: VolatileCell<u32>,
}
#[doc = "Port Input Disable Register"]
pub mod pidr;
