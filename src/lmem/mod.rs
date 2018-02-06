use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub lmem_pcccr: LMEM_PCCCR,
    #[doc = "0x04 - Cache line control register"]
    pub lmem_pcclcr: LMEM_PCCLCR,
    #[doc = "0x08 - Cache search address register"]
    pub lmem_pccsar: LMEM_PCCSAR,
    #[doc = "0x0c - Cache read/write value register"]
    pub lmem_pcccvr: LMEM_PCCCVR,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: PCCRMR,
}
#[doc = "Cache control register"]
pub struct LMEM_PCCCR {
    register: VolatileCell<u32>,
}
#[doc = "Cache control register"]
pub mod lmem_pcccr;
#[doc = "Cache line control register"]
pub struct LMEM_PCCLCR {
    register: VolatileCell<u32>,
}
#[doc = "Cache line control register"]
pub mod lmem_pcclcr;
#[doc = "Cache search address register"]
pub struct LMEM_PCCSAR {
    register: VolatileCell<u32>,
}
#[doc = "Cache search address register"]
pub mod lmem_pccsar;
#[doc = "Cache read/write value register"]
pub struct LMEM_PCCCVR {
    register: VolatileCell<u32>,
}
#[doc = "Cache read/write value register"]
pub mod lmem_pcccvr;
#[doc = "Cache regions mode register"]
pub struct PCCRMR {
    register: VolatileCell<u32>,
}
#[doc = "Cache regions mode register"]
pub mod pccrmr;
