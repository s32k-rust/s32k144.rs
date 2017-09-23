use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Processor X Type Register"]
    pub cpx_type: CPXTYPE,
    #[doc = "0x04 - Processor X Number Register"]
    pub cpx_num: CPXNUM,
    #[doc = "0x08 - Processor X Master Register"]
    pub cpx_master: CPXMASTER,
    #[doc = "0x0c - Processor X Count Register"]
    pub cpx_count: CPXCOUNT,
    #[doc = "0x10 - Processor X Configuration Register 0"]
    pub cpx_cfg0: CPXCFG0,
    #[doc = "0x14 - Processor X Configuration Register 1"]
    pub cpx_cfg1: CPXCFG1,
    #[doc = "0x18 - Processor X Configuration Register 2"]
    pub cpx_cfg2: CPXCFG2,
    #[doc = "0x1c - Processor X Configuration Register 3"]
    pub cpx_cfg3: CPXCFG3,
    #[doc = "0x20 - Processor 0 Type Register"]
    pub cp0type: CP0TYPE,
    #[doc = "0x24 - Processor 0 Number Register"]
    pub cp0num: CP0NUM,
    #[doc = "0x28 - Processor 0 Master Register"]
    pub cp0master: CP0MASTER,
    #[doc = "0x2c - Processor 0 Count Register"]
    pub cp0count: CP0COUNT,
    #[doc = "0x30 - Processor 0 Configuration Register 0"]
    pub cp0cfg0: CP0CFG0,
    #[doc = "0x34 - Processor 0 Configuration Register 1"]
    pub cp0cfg1: CP0CFG1,
    #[doc = "0x38 - Processor 0 Configuration Register 2"]
    pub cp0cfg2: CP0CFG2,
    #[doc = "0x3c - Processor 0 Configuration Register 3"]
    pub cp0cfg3: CP0CFG3,
    _reserved0: [u8; 960usize],
    #[doc = "0x400 - On-Chip Memory Descriptor Register"]
    pub ocmdr0: OCMDR0,
    #[doc = "0x404 - On-Chip Memory Descriptor Register"]
    pub ocmdr1: OCMDR1,
    #[doc = "0x408 - On-Chip Memory Descriptor Register"]
    pub ocmdr2: OCMDR2,
    #[doc = "0x40c - On-Chip Memory Descriptor Register"]
    pub ocmdr3: OCMDR3,
}
#[doc = "Processor X Type Register"]
pub struct CPXTYPE {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Type Register"]
pub mod cpx_type;
#[doc = "Processor X Number Register"]
pub struct CPXNUM {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Number Register"]
pub mod cpx_num;
#[doc = "Processor X Master Register"]
pub struct CPXMASTER {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Master Register"]
pub mod cpx_master;
#[doc = "Processor X Count Register"]
pub struct CPXCOUNT {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Count Register"]
pub mod cpx_count;
#[doc = "Processor X Configuration Register 0"]
pub struct CPXCFG0 {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Configuration Register 0"]
pub mod cpx_cfg0;
#[doc = "Processor X Configuration Register 1"]
pub struct CPXCFG1 {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Configuration Register 1"]
pub mod cpx_cfg1;
#[doc = "Processor X Configuration Register 2"]
pub struct CPXCFG2 {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Configuration Register 2"]
pub mod cpx_cfg2;
#[doc = "Processor X Configuration Register 3"]
pub struct CPXCFG3 {
    register: VolatileCell<u32>,
}
#[doc = "Processor X Configuration Register 3"]
pub mod cpx_cfg3;
#[doc = "Processor 0 Type Register"]
pub struct CP0TYPE {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Type Register"]
pub mod cp0type;
#[doc = "Processor 0 Number Register"]
pub struct CP0NUM {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Number Register"]
pub mod cp0num;
#[doc = "Processor 0 Master Register"]
pub struct CP0MASTER {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Master Register"]
pub mod cp0master;
#[doc = "Processor 0 Count Register"]
pub struct CP0COUNT {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Count Register"]
pub mod cp0count;
#[doc = "Processor 0 Configuration Register 0"]
pub struct CP0CFG0 {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Configuration Register 0"]
pub mod cp0cfg0;
#[doc = "Processor 0 Configuration Register 1"]
pub struct CP0CFG1 {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Configuration Register 1"]
pub mod cp0cfg1;
#[doc = "Processor 0 Configuration Register 2"]
pub struct CP0CFG2 {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Configuration Register 2"]
pub mod cp0cfg2;
#[doc = "Processor 0 Configuration Register 3"]
pub struct CP0CFG3 {
    register: VolatileCell<u32>,
}
#[doc = "Processor 0 Configuration Register 3"]
pub mod cp0cfg3;
#[doc = "On-Chip Memory Descriptor Register"]
pub struct OCMDR0 {
    register: VolatileCell<u32>,
}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr0;
#[doc = "On-Chip Memory Descriptor Register"]
pub struct OCMDR1 {
    register: VolatileCell<u32>,
}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr1;
#[doc = "On-Chip Memory Descriptor Register"]
pub struct OCMDR2 {
    register: VolatileCell<u32>,
}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr2;
#[doc = "On-Chip Memory Descriptor Register"]
pub struct OCMDR3 {
    register: VolatileCell<u32>,
}
#[doc = "On-Chip Memory Descriptor Register"]
pub mod ocmdr3;
