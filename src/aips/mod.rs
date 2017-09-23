use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: MPRA,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: PACRA,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: PACRB,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: PACRC,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: PACRD,
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Register"]
    pub opacra: OPACRA,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Register"]
    pub opacrb: OPACRB,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Register"]
    pub opacrc: OPACRC,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Register"]
    pub opacrd: OPACRD,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Register"]
    pub opacre: OPACRE,
    #[doc = "0x54 - Off-Platform Peripheral Access Control Register"]
    pub opacrf: OPACRF,
    #[doc = "0x58 - Off-Platform Peripheral Access Control Register"]
    pub opacrg: OPACRG,
    #[doc = "0x5c - Off-Platform Peripheral Access Control Register"]
    pub opacrh: OPACRH,
    #[doc = "0x60 - Off-Platform Peripheral Access Control Register"]
    pub opacri: OPACRI,
    #[doc = "0x64 - Off-Platform Peripheral Access Control Register"]
    pub opacrj: OPACRJ,
    #[doc = "0x68 - Off-Platform Peripheral Access Control Register"]
    pub opacrk: OPACRK,
    #[doc = "0x6c - Off-Platform Peripheral Access Control Register"]
    pub opacrl: OPACRL,
}
#[doc = "Master Privilege Register A"]
pub struct MPRA {
    register: VolatileCell<u32>,
}
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "Peripheral Access Control Register"]
pub struct PACRA {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "Peripheral Access Control Register"]
pub struct PACRB {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "Peripheral Access Control Register"]
pub struct PACRC {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "Peripheral Access Control Register"]
pub struct PACRD {
    register: VolatileCell<u32>,
}
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRA {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacra;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRB {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrb;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRC {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrc;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRD {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrd;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRE {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacre;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRF {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrf;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRG {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrg;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRH {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrh;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRI {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacri;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRJ {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrj;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRK {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrk;
#[doc = "Off-Platform Peripheral Access Control Register"]
pub struct OPACRL {
    register: VolatileCell<u32>,
}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrl;
