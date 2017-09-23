use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"] pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"] pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"] pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"] pub fopt: FOPT,
    #[doc = "0x04 - Flash Common Command Object Registers"] pub fccob3: FCCOB3,
    #[doc = "0x05 - Flash Common Command Object Registers"] pub fccob2: FCCOB2,
    #[doc = "0x06 - Flash Common Command Object Registers"] pub fccob1: FCCOB1,
    #[doc = "0x07 - Flash Common Command Object Registers"] pub fccob0: FCCOB0,
    #[doc = "0x08 - Flash Common Command Object Registers"] pub fccob7: FCCOB7,
    #[doc = "0x09 - Flash Common Command Object Registers"] pub fccob6: FCCOB6,
    #[doc = "0x0a - Flash Common Command Object Registers"] pub fccob5: FCCOB5,
    #[doc = "0x0b - Flash Common Command Object Registers"] pub fccob4: FCCOB4,
    #[doc = "0x0c - Flash Common Command Object Registers"] pub fccobb: FCCOBB,
    #[doc = "0x0d - Flash Common Command Object Registers"] pub fccoba: FCCOBA,
    #[doc = "0x0e - Flash Common Command Object Registers"] pub fccob9: FCCOB9,
    #[doc = "0x0f - Flash Common Command Object Registers"] pub fccob8: FCCOB8,
    #[doc = "0x10 - Program Flash Protection Registers"] pub fprot3: FPROT3,
    #[doc = "0x11 - Program Flash Protection Registers"] pub fprot2: FPROT2,
    #[doc = "0x12 - Program Flash Protection Registers"] pub fprot1: FPROT1,
    #[doc = "0x13 - Program Flash Protection Registers"] pub fprot0: FPROT0,
    _reserved0: [u8; 2usize],
    #[doc = "0x16 - EEPROM Protection Register"] pub feprot: FEPROT,
    #[doc = "0x17 - Data Flash Protection Register"] pub fdprot: FDPROT,
    _reserved1: [u8; 20usize],
    #[doc = "0x2c - Flash CSEc Status Register"] pub fcsestat: FCSESTAT,
    _reserved2: [u8; 1usize],
    #[doc = "0x2e - Flash Error Status Register"] pub ferstat: FERSTAT,
    #[doc = "0x2f - Flash Error Configuration Register"] pub fercnfg: FERCNFG,
}
#[doc = "Flash Status Register"]
pub struct FSTAT {
    register: VolatileCell<u8>,
}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register"]
pub struct FCNFG {
    register: VolatileCell<u8>,
}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Security Register"]
pub struct FSEC {
    register: VolatileCell<u8>,
}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Option Register"]
pub struct FOPT {
    register: VolatileCell<u8>,
}
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB3 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob3;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB2 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob2;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB1 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob1;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB0 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob0;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB7 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob7;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB6 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob6;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB5 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob5;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB4 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob4;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOBB {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccobb;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOBA {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccoba;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB9 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob9;
#[doc = "Flash Common Command Object Registers"]
pub struct FCCOB8 {
    register: VolatileCell<u8>,
}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob8;
#[doc = "Program Flash Protection Registers"]
pub struct FPROT3 {
    register: VolatileCell<u8>,
}
#[doc = "Program Flash Protection Registers"]
pub mod fprot3;
#[doc = "Program Flash Protection Registers"]
pub struct FPROT2 {
    register: VolatileCell<u8>,
}
#[doc = "Program Flash Protection Registers"]
pub mod fprot2;
#[doc = "Program Flash Protection Registers"]
pub struct FPROT1 {
    register: VolatileCell<u8>,
}
#[doc = "Program Flash Protection Registers"]
pub mod fprot1;
#[doc = "Program Flash Protection Registers"]
pub struct FPROT0 {
    register: VolatileCell<u8>,
}
#[doc = "Program Flash Protection Registers"]
pub mod fprot0;
#[doc = "EEPROM Protection Register"]
pub struct FEPROT {
    register: VolatileCell<u8>,
}
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "Data Flash Protection Register"]
pub struct FDPROT {
    register: VolatileCell<u8>,
}
#[doc = "Data Flash Protection Register"]
pub mod fdprot;
#[doc = "Flash CSEc Status Register"]
pub struct FCSESTAT {
    register: VolatileCell<u8>,
}
#[doc = "Flash CSEc Status Register"]
pub mod fcsestat;
#[doc = "Flash Error Status Register"]
pub struct FERSTAT {
    register: VolatileCell<u8>,
}
#[doc = "Flash Error Status Register"]
pub mod ferstat;
#[doc = "Flash Error Configuration Register"]
pub struct FERCNFG {
    register: VolatileCell<u8>,
}
#[doc = "Flash Error Configuration Register"]
pub mod fercnfg;
