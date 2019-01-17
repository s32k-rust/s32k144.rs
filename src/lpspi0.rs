#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Control Register"]
    pub cr: CR,
    #[doc = "0x14 - Status Register"]
    pub sr: SR,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x1c - DMA Enable Register"]
    pub der: DER,
    #[doc = "0x20 - Configuration Register 0"]
    pub cfgr0: CFGR0,
    #[doc = "0x24 - Configuration Register 1"]
    pub cfgr1: CFGR1,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - Data Match Register 0"]
    pub dmr0: DMR0,
    #[doc = "0x34 - Data Match Register 1"]
    pub dmr1: DMR1,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - Clock Configuration Register"]
    pub ccr: CCR,
    _reserved3: [u8; 20usize],
    #[doc = "0x58 - FIFO Control Register"]
    pub fcr: FCR,
    #[doc = "0x5c - FIFO Status Register"]
    pub fsr: FSR,
    #[doc = "0x60 - Transmit Command Register"]
    pub tcr: TCR,
    #[doc = "0x64 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved4: [u8; 8usize],
    #[doc = "0x70 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x74 - Receive Data Register"]
    pub rdr: RDR,
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
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "DMA Enable Register"]
pub struct DER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Enable Register"]
pub mod der;
#[doc = "Configuration Register 0"]
pub struct CFGR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register 0"]
pub mod cfgr0;
#[doc = "Configuration Register 1"]
pub struct CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register 1"]
pub mod cfgr1;
#[doc = "Data Match Register 0"]
pub struct DMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Match Register 0"]
pub mod dmr0;
#[doc = "Data Match Register 1"]
pub struct DMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Match Register 1"]
pub mod dmr1;
#[doc = "Clock Configuration Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Configuration Register"]
pub mod ccr;
#[doc = "FIFO Control Register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "FIFO Status Register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Status Register"]
pub mod fsr;
#[doc = "Transmit Command Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Command Register"]
pub mod tcr;
#[doc = "Transmit Data Register"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "Receive Status Register"]
pub struct RSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Receive Data Register"]
pub struct RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod rdr;
