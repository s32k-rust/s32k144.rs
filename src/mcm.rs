#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Core Platform Control Register"]
    pub cpcr: CPCR,
    #[doc = "0x10 - Interrupt Status and Control Register"]
    pub iscr: ISCR,
    _reserved1: [u8; 28usize],
    #[doc = "0x30 - Process ID Register"]
    pub pid: PID,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
    _reserved3: [u8; 956usize],
    #[doc = "0x400 - Local Memory Descriptor Register"]
    pub lmdr0: LMDR0,
    #[doc = "0x404 - Local Memory Descriptor Register"]
    pub lmdr1: LMDR1,
    #[doc = "0x408 - Local Memory Descriptor Register2"]
    pub lmdr2: LMDR2,
    _reserved4: [u8; 116usize],
    #[doc = "0x480 - LMEM Parity and ECC Control Register"]
    pub lmpecr: LMPECR,
    _reserved5: [u8; 4usize],
    #[doc = "0x488 - LMEM Parity and ECC Interrupt Register"]
    pub lmpeir: LMPEIR,
    _reserved6: [u8; 4usize],
    #[doc = "0x490 - LMEM Fault Address Register"]
    pub lmfar: LMFAR,
    #[doc = "0x494 - LMEM Fault Attribute Register"]
    pub lmfatr: LMFATR,
    _reserved7: [u8; 8usize],
    #[doc = "0x4a0 - LMEM Fault Data High Register"]
    pub lmfdhr: LMFDHR,
    #[doc = "0x4a4 - LMEM Fault Data Low Register"]
    pub lmfdlr: LMFDLR,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub struct PLASC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub struct PLAMC {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Core Platform Control Register"]
pub struct CPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Core Platform Control Register"]
pub mod cpcr;
#[doc = "Interrupt Status and Control Register"]
pub struct ISCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
#[doc = "Process ID Register"]
pub struct PID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Process ID Register"]
pub mod pid;
#[doc = "Compute Operation Control Register"]
pub struct CPO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
#[doc = "Local Memory Descriptor Register"]
pub struct LMDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr0;
#[doc = "Local Memory Descriptor Register"]
pub struct LMDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr1;
#[doc = "Local Memory Descriptor Register2"]
pub struct LMDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Local Memory Descriptor Register2"]
pub mod lmdr2;
#[doc = "LMEM Parity and ECC Control Register"]
pub struct LMPECR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Parity and ECC Control Register"]
pub mod lmpecr;
#[doc = "LMEM Parity and ECC Interrupt Register"]
pub struct LMPEIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Parity and ECC Interrupt Register"]
pub mod lmpeir;
#[doc = "LMEM Fault Address Register"]
pub struct LMFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Fault Address Register"]
pub mod lmfar;
#[doc = "LMEM Fault Attribute Register"]
pub struct LMFATR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Fault Attribute Register"]
pub mod lmfatr;
#[doc = "LMEM Fault Data High Register"]
pub struct LMFDHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Fault Data High Register"]
pub mod lmfdhr;
#[doc = "LMEM Fault Data Low Register"]
pub struct LMFDLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LMEM Fault Data Low Register"]
pub mod lmfdlr;
