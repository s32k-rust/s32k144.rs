#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Control Register n"]
    pub pcr0: PCR0,
    #[doc = "0x04 - Pin Control Register n"]
    pub pcr1: PCR1,
    #[doc = "0x08 - Pin Control Register n"]
    pub pcr2: PCR2,
    #[doc = "0x0c - Pin Control Register n"]
    pub pcr3: PCR3,
    #[doc = "0x10 - Pin Control Register n"]
    pub pcr4: PCR4,
    #[doc = "0x14 - Pin Control Register n"]
    pub pcr5: PCR5,
    #[doc = "0x18 - Pin Control Register n"]
    pub pcr6: PCR6,
    #[doc = "0x1c - Pin Control Register n"]
    pub pcr7: PCR7,
    #[doc = "0x20 - Pin Control Register n"]
    pub pcr8: PCR8,
    #[doc = "0x24 - Pin Control Register n"]
    pub pcr9: PCR9,
    #[doc = "0x28 - Pin Control Register n"]
    pub pcr10: PCR10,
    #[doc = "0x2c - Pin Control Register n"]
    pub pcr11: PCR11,
    #[doc = "0x30 - Pin Control Register n"]
    pub pcr12: PCR12,
    #[doc = "0x34 - Pin Control Register n"]
    pub pcr13: PCR13,
    #[doc = "0x38 - Pin Control Register n"]
    pub pcr14: PCR14,
    #[doc = "0x3c - Pin Control Register n"]
    pub pcr15: PCR15,
    #[doc = "0x40 - Pin Control Register n"]
    pub pcr16: PCR16,
    #[doc = "0x44 - Pin Control Register n"]
    pub pcr17: PCR17,
    #[doc = "0x48 - Pin Control Register n"]
    pub pcr18: PCR18,
    #[doc = "0x4c - Pin Control Register n"]
    pub pcr19: PCR19,
    #[doc = "0x50 - Pin Control Register n"]
    pub pcr20: PCR20,
    #[doc = "0x54 - Pin Control Register n"]
    pub pcr21: PCR21,
    #[doc = "0x58 - Pin Control Register n"]
    pub pcr22: PCR22,
    #[doc = "0x5c - Pin Control Register n"]
    pub pcr23: PCR23,
    #[doc = "0x60 - Pin Control Register n"]
    pub pcr24: PCR24,
    #[doc = "0x64 - Pin Control Register n"]
    pub pcr25: PCR25,
    #[doc = "0x68 - Pin Control Register n"]
    pub pcr26: PCR26,
    #[doc = "0x6c - Pin Control Register n"]
    pub pcr27: PCR27,
    #[doc = "0x70 - Pin Control Register n"]
    pub pcr28: PCR28,
    #[doc = "0x74 - Pin Control Register n"]
    pub pcr29: PCR29,
    #[doc = "0x78 - Pin Control Register n"]
    pub pcr30: PCR30,
    #[doc = "0x7c - Pin Control Register n"]
    pub pcr31: PCR31,
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: GPCLR,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: GPCHR,
    _reserved0: [u8; 24usize],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: ISFR,
    _reserved1: [u8; 28usize],
    #[doc = "0xc0 - Digital Filter Enable Register"]
    pub dfer: DFER,
    #[doc = "0xc4 - Digital Filter Clock Register"]
    pub dfcr: DFCR,
    #[doc = "0xc8 - Digital Filter Width Register"]
    pub dfwr: DFWR,
}
#[doc = "Pin Control Register n"]
pub struct PCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr0;
#[doc = "Pin Control Register n"]
pub struct PCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr1;
#[doc = "Pin Control Register n"]
pub struct PCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr2;
#[doc = "Pin Control Register n"]
pub struct PCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr3;
#[doc = "Pin Control Register n"]
pub struct PCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr4;
#[doc = "Pin Control Register n"]
pub struct PCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr5;
#[doc = "Pin Control Register n"]
pub struct PCR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr6;
#[doc = "Pin Control Register n"]
pub struct PCR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr7;
#[doc = "Pin Control Register n"]
pub struct PCR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr8;
#[doc = "Pin Control Register n"]
pub struct PCR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr9;
#[doc = "Pin Control Register n"]
pub struct PCR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr10;
#[doc = "Pin Control Register n"]
pub struct PCR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr11;
#[doc = "Pin Control Register n"]
pub struct PCR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr12;
#[doc = "Pin Control Register n"]
pub struct PCR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr13;
#[doc = "Pin Control Register n"]
pub struct PCR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr14;
#[doc = "Pin Control Register n"]
pub struct PCR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr15;
#[doc = "Pin Control Register n"]
pub struct PCR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr16;
#[doc = "Pin Control Register n"]
pub struct PCR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr17;
#[doc = "Pin Control Register n"]
pub struct PCR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr18;
#[doc = "Pin Control Register n"]
pub struct PCR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr19;
#[doc = "Pin Control Register n"]
pub struct PCR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr20;
#[doc = "Pin Control Register n"]
pub struct PCR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr21;
#[doc = "Pin Control Register n"]
pub struct PCR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr22;
#[doc = "Pin Control Register n"]
pub struct PCR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr23;
#[doc = "Pin Control Register n"]
pub struct PCR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr24;
#[doc = "Pin Control Register n"]
pub struct PCR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr25;
#[doc = "Pin Control Register n"]
pub struct PCR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr26;
#[doc = "Pin Control Register n"]
pub struct PCR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr27;
#[doc = "Pin Control Register n"]
pub struct PCR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr28;
#[doc = "Pin Control Register n"]
pub struct PCR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr29;
#[doc = "Pin Control Register n"]
pub struct PCR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr30;
#[doc = "Pin Control Register n"]
pub struct PCR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin Control Register n"]
pub mod pcr31;
#[doc = "Global Pin Control Low Register"]
pub struct GPCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "Global Pin Control High Register"]
pub struct GPCHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "Interrupt Status Flag Register"]
pub struct ISFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
#[doc = "Digital Filter Enable Register"]
pub struct DFER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "Digital Filter Clock Register"]
pub struct DFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "Digital Filter Width Register"]
pub struct DFWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
