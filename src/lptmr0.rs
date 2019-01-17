#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Timer Control Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - Low Power Timer Prescale Register"]
    pub psr: PSR,
    #[doc = "0x08 - Low Power Timer Compare Register"]
    pub cmr: CMR,
    #[doc = "0x0c - Low Power Timer Counter Register"]
    pub cnr: CNR,
}
#[doc = "Low Power Timer Control Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Timer Control Status Register"]
pub mod csr;
#[doc = "Low Power Timer Prescale Register"]
pub struct PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Timer Prescale Register"]
pub mod psr;
#[doc = "Low Power Timer Compare Register"]
pub struct CMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Timer Compare Register"]
pub mod cmr;
#[doc = "Low Power Timer Counter Register"]
pub struct CNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Low Power Timer Counter Register"]
pub mod cnr;
