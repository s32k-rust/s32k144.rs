use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error Injection Module Configuration Register"]
    pub eimcr: EIMCR,
    #[doc = "0x04 - Error Injection Channel Enable register"]
    pub eichen: EICHEN,
    _reserved0: [u8; 248usize],
    #[doc = "0x100 - Error Injection Channel Descriptor n, Word0"]
    pub eichd0_word0: EICHD0_WORD0,
    #[doc = "0x104 - Error Injection Channel Descriptor n, Word1"]
    pub eichd0_word1: EICHD0_WORD1,
    _reserved1: [u8; 248usize],
    #[doc = "0x200 - Error Injection Channel Descriptor n, Word0"]
    pub eichd1_word0: EICHD1_WORD0,
    #[doc = "0x204 - Error Injection Channel Descriptor n, Word1"]
    pub eichd1_word1: EICHD1_WORD1,
}
#[doc = "Error Injection Module Configuration Register"]
pub struct EIMCR {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Module Configuration Register"]
pub mod eimcr;
#[doc = "Error Injection Channel Enable register"]
pub struct EICHEN {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Channel Enable register"]
pub mod eichen;
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub struct EICHD0_WORD0 {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd0_word0;
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub struct EICHD0_WORD1 {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd0_word1;
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub struct EICHD1_WORD0 {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd1_word0;
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub struct EICHD1_WORD1 {
    register: VolatileCell<u32>,
}
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd1_word1;
