use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Master Control Register"]
    pub mcr: MCR,
    #[doc = "0x14 - Master Status Register"]
    pub msr: MSR,
    #[doc = "0x18 - Master Interrupt Enable Register"]
    pub mier: MIER,
    #[doc = "0x1c - Master DMA Enable Register"]
    pub mder: MDER,
    #[doc = "0x20 - Master Configuration Register 0"]
    pub mcfgr0: MCFGR0,
    #[doc = "0x24 - Master Configuration Register 1"]
    pub mcfgr1: MCFGR1,
    #[doc = "0x28 - Master Configuration Register 2"]
    pub mcfgr2: MCFGR2,
    #[doc = "0x2c - Master Configuration Register 3"]
    pub mcfgr3: MCFGR3,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Master Data Match Register"]
    pub mdmr: MDMR,
    _reserved2: [u8; 4usize],
    #[doc = "0x48 - Master Clock Configuration Register 0"]
    pub mccr0: MCCR0,
    _reserved3: [u8; 4usize],
    #[doc = "0x50 - Master Clock Configuration Register 1"]
    pub mccr1: MCCR1,
    _reserved4: [u8; 4usize],
    #[doc = "0x58 - Master FIFO Control Register"]
    pub mfcr: MFCR,
    #[doc = "0x5c - Master FIFO Status Register"]
    pub mfsr: MFSR,
    #[doc = "0x60 - Master Transmit Data Register"]
    pub mtdr: MTDR,
    _reserved5: [u8; 12usize],
    #[doc = "0x70 - Master Receive Data Register"]
    pub mrdr: MRDR,
    _reserved6: [u8; 156usize],
    #[doc = "0x110 - Slave Control Register"]
    pub scr: SCR,
    #[doc = "0x114 - Slave Status Register"]
    pub ssr: SSR,
    #[doc = "0x118 - Slave Interrupt Enable Register"]
    pub sier: SIER,
    #[doc = "0x11c - Slave DMA Enable Register"]
    pub sder: SDER,
    _reserved7: [u8; 4usize],
    #[doc = "0x124 - Slave Configuration Register 1"]
    pub scfgr1: SCFGR1,
    #[doc = "0x128 - Slave Configuration Register 2"]
    pub scfgr2: SCFGR2,
    _reserved8: [u8; 20usize],
    #[doc = "0x140 - Slave Address Match Register"]
    pub samr: SAMR,
    _reserved9: [u8; 12usize],
    #[doc = "0x150 - Slave Address Status Register"]
    pub sasr: SASR,
    #[doc = "0x154 - Slave Transmit ACK Register"]
    pub star: STAR,
    _reserved10: [u8; 8usize],
    #[doc = "0x160 - Slave Transmit Data Register"]
    pub stdr: STDR,
    _reserved11: [u8; 12usize],
    #[doc = "0x170 - Slave Receive Data Register"]
    pub srdr: SRDR,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Master Control Register"]
pub struct MCR {
    register: VolatileCell<u32>,
}
#[doc = "Master Control Register"]
pub mod mcr;
#[doc = "Master Status Register"]
pub struct MSR {
    register: VolatileCell<u32>,
}
#[doc = "Master Status Register"]
pub mod msr;
#[doc = "Master Interrupt Enable Register"]
pub struct MIER {
    register: VolatileCell<u32>,
}
#[doc = "Master Interrupt Enable Register"]
pub mod mier;
#[doc = "Master DMA Enable Register"]
pub struct MDER {
    register: VolatileCell<u32>,
}
#[doc = "Master DMA Enable Register"]
pub mod mder;
#[doc = "Master Configuration Register 0"]
pub struct MCFGR0 {
    register: VolatileCell<u32>,
}
#[doc = "Master Configuration Register 0"]
pub mod mcfgr0;
#[doc = "Master Configuration Register 1"]
pub struct MCFGR1 {
    register: VolatileCell<u32>,
}
#[doc = "Master Configuration Register 1"]
pub mod mcfgr1;
#[doc = "Master Configuration Register 2"]
pub struct MCFGR2 {
    register: VolatileCell<u32>,
}
#[doc = "Master Configuration Register 2"]
pub mod mcfgr2;
#[doc = "Master Configuration Register 3"]
pub struct MCFGR3 {
    register: VolatileCell<u32>,
}
#[doc = "Master Configuration Register 3"]
pub mod mcfgr3;
#[doc = "Master Data Match Register"]
pub struct MDMR {
    register: VolatileCell<u32>,
}
#[doc = "Master Data Match Register"]
pub mod mdmr;
#[doc = "Master Clock Configuration Register 0"]
pub struct MCCR0 {
    register: VolatileCell<u32>,
}
#[doc = "Master Clock Configuration Register 0"]
pub mod mccr0;
#[doc = "Master Clock Configuration Register 1"]
pub struct MCCR1 {
    register: VolatileCell<u32>,
}
#[doc = "Master Clock Configuration Register 1"]
pub mod mccr1;
#[doc = "Master FIFO Control Register"]
pub struct MFCR {
    register: VolatileCell<u32>,
}
#[doc = "Master FIFO Control Register"]
pub mod mfcr;
#[doc = "Master FIFO Status Register"]
pub struct MFSR {
    register: VolatileCell<u32>,
}
#[doc = "Master FIFO Status Register"]
pub mod mfsr;
#[doc = "Master Transmit Data Register"]
pub struct MTDR {
    register: VolatileCell<u32>,
}
#[doc = "Master Transmit Data Register"]
pub mod mtdr;
#[doc = "Master Receive Data Register"]
pub struct MRDR {
    register: VolatileCell<u32>,
}
#[doc = "Master Receive Data Register"]
pub mod mrdr;
#[doc = "Slave Control Register"]
pub struct SCR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Control Register"]
pub mod scr;
#[doc = "Slave Status Register"]
pub struct SSR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Status Register"]
pub mod ssr;
#[doc = "Slave Interrupt Enable Register"]
pub struct SIER {
    register: VolatileCell<u32>,
}
#[doc = "Slave Interrupt Enable Register"]
pub mod sier;
#[doc = "Slave DMA Enable Register"]
pub struct SDER {
    register: VolatileCell<u32>,
}
#[doc = "Slave DMA Enable Register"]
pub mod sder;
#[doc = "Slave Configuration Register 1"]
pub struct SCFGR1 {
    register: VolatileCell<u32>,
}
#[doc = "Slave Configuration Register 1"]
pub mod scfgr1;
#[doc = "Slave Configuration Register 2"]
pub struct SCFGR2 {
    register: VolatileCell<u32>,
}
#[doc = "Slave Configuration Register 2"]
pub mod scfgr2;
#[doc = "Slave Address Match Register"]
pub struct SAMR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Address Match Register"]
pub mod samr;
#[doc = "Slave Address Status Register"]
pub struct SASR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Address Status Register"]
pub mod sasr;
#[doc = "Slave Transmit ACK Register"]
pub struct STAR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Transmit ACK Register"]
pub mod star;
#[doc = "Slave Transmit Data Register"]
pub struct STDR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Transmit Data Register"]
pub mod stdr;
#[doc = "Slave Receive Data Register"]
pub struct SRDR {
    register: VolatileCell<u32>,
}
#[doc = "Slave Receive Data Register"]
pub mod srdr;
