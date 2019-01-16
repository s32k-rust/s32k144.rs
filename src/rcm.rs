#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - System Reset Status Register"]
    pub srs: SRS,
    #[doc = "0x0c - Reset Pin Control register"]
    pub rpc: RPC,
    _reserved0: [u8; 8usize],
    #[doc = "0x18 - Sticky System Reset Status Register"]
    pub ssrs: SSRS,
    #[doc = "0x1c - System Reset Interrupt Enable Register"]
    pub srie: SRIE,
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
#[doc = "System Reset Status Register"]
pub struct SRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Reset Status Register"]
pub mod srs;
#[doc = "Reset Pin Control register"]
pub struct RPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset Pin Control register"]
pub mod rpc;
#[doc = "Sticky System Reset Status Register"]
pub struct SSRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sticky System Reset Status Register"]
pub mod ssrs;
#[doc = "System Reset Interrupt Enable Register"]
pub struct SRIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Reset Interrupt Enable Register"]
pub mod srie;
