#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x04 - Flash Common Command Object Registers"]
    pub fccob3: FCCOB3,
    #[doc = "0x05 - Flash Common Command Object Registers"]
    pub fccob2: FCCOB2,
    #[doc = "0x06 - Flash Common Command Object Registers"]
    pub fccob1: FCCOB1,
    #[doc = "0x07 - Flash Common Command Object Registers"]
    pub fccob0: FCCOB0,
    #[doc = "0x08 - Flash Common Command Object Registers"]
    pub fccob7: FCCOB7,
    #[doc = "0x09 - Flash Common Command Object Registers"]
    pub fccob6: FCCOB6,
    #[doc = "0x0a - Flash Common Command Object Registers"]
    pub fccob5: FCCOB5,
    #[doc = "0x0b - Flash Common Command Object Registers"]
    pub fccob4: FCCOB4,
    #[doc = "0x0c - Flash Common Command Object Registers"]
    pub fccobb: FCCOBB,
    #[doc = "0x0d - Flash Common Command Object Registers"]
    pub fccoba: FCCOBA,
    #[doc = "0x0e - Flash Common Command Object Registers"]
    pub fccob9: FCCOB9,
    #[doc = "0x0f - Flash Common Command Object Registers"]
    pub fccob8: FCCOB8,
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot3: FPROT3,
    #[doc = "0x11 - Program Flash Protection Registers"]
    pub fprot2: FPROT2,
    #[doc = "0x12 - Program Flash Protection Registers"]
    pub fprot1: FPROT1,
    #[doc = "0x13 - Program Flash Protection Registers"]
    pub fprot0: FPROT0,
    _reserved20: [u8; 2usize],
    #[doc = "0x16 - EEPROM Protection Register"]
    pub feprot: FEPROT,
    #[doc = "0x17 - Data Flash Protection Register"]
    pub fdprot: FDPROT,
    _reserved22: [u8; 20usize],
    #[doc = "0x2c - Flash CSEc Status Register"]
    pub fcsestat: FCSESTAT,
    _reserved23: [u8; 1usize],
    #[doc = "0x2e - Flash Error Status Register"]
    pub ferstat: FERSTAT,
    #[doc = "0x2f - Flash Error Configuration Register"]
    pub fercnfg: FERCNFG,
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](fstat) module"]
pub type FSTAT = crate::Reg<u8, _FSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSTAT;
#[doc = "`read()` method returns [fstat::R](fstat::R) reader structure"]
impl crate::Readable for FSTAT {}
#[doc = "`write(|w| ..)` method takes [fstat::W](fstat::W) writer structure"]
impl crate::Writable for FSTAT {}
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "Flash Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcnfg](fcnfg) module"]
pub type FCNFG = crate::Reg<u8, _FCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCNFG;
#[doc = "`read()` method returns [fcnfg::R](fcnfg::R) reader structure"]
impl crate::Readable for FCNFG {}
#[doc = "`write(|w| ..)` method takes [fcnfg::W](fcnfg::W) writer structure"]
impl crate::Writable for FCNFG {}
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](fsec) module"]
pub type FSEC = crate::Reg<u8, _FSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEC;
#[doc = "`read()` method returns [fsec::R](fsec::R) reader structure"]
impl crate::Readable for FSEC {}
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](fopt) module"]
pub type FOPT = crate::Reg<u8, _FOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FOPT;
#[doc = "`read()` method returns [fopt::R](fopt::R) reader structure"]
impl crate::Readable for FOPT {}
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob3](fccob3) module"]
pub type FCCOB3 = crate::Reg<u8, _FCCOB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB3;
#[doc = "`read()` method returns [fccob3::R](fccob3::R) reader structure"]
impl crate::Readable for FCCOB3 {}
#[doc = "`write(|w| ..)` method takes [fccob3::W](fccob3::W) writer structure"]
impl crate::Writable for FCCOB3 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob3;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob2](fccob2) module"]
pub type FCCOB2 = crate::Reg<u8, _FCCOB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB2;
#[doc = "`read()` method returns [fccob2::R](fccob2::R) reader structure"]
impl crate::Readable for FCCOB2 {}
#[doc = "`write(|w| ..)` method takes [fccob2::W](fccob2::W) writer structure"]
impl crate::Writable for FCCOB2 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob2;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob1](fccob1) module"]
pub type FCCOB1 = crate::Reg<u8, _FCCOB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB1;
#[doc = "`read()` method returns [fccob1::R](fccob1::R) reader structure"]
impl crate::Readable for FCCOB1 {}
#[doc = "`write(|w| ..)` method takes [fccob1::W](fccob1::W) writer structure"]
impl crate::Writable for FCCOB1 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob1;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob0](fccob0) module"]
pub type FCCOB0 = crate::Reg<u8, _FCCOB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB0;
#[doc = "`read()` method returns [fccob0::R](fccob0::R) reader structure"]
impl crate::Readable for FCCOB0 {}
#[doc = "`write(|w| ..)` method takes [fccob0::W](fccob0::W) writer structure"]
impl crate::Writable for FCCOB0 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob0;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob7](fccob7) module"]
pub type FCCOB7 = crate::Reg<u8, _FCCOB7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB7;
#[doc = "`read()` method returns [fccob7::R](fccob7::R) reader structure"]
impl crate::Readable for FCCOB7 {}
#[doc = "`write(|w| ..)` method takes [fccob7::W](fccob7::W) writer structure"]
impl crate::Writable for FCCOB7 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob7;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob6](fccob6) module"]
pub type FCCOB6 = crate::Reg<u8, _FCCOB6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB6;
#[doc = "`read()` method returns [fccob6::R](fccob6::R) reader structure"]
impl crate::Readable for FCCOB6 {}
#[doc = "`write(|w| ..)` method takes [fccob6::W](fccob6::W) writer structure"]
impl crate::Writable for FCCOB6 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob6;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob5](fccob5) module"]
pub type FCCOB5 = crate::Reg<u8, _FCCOB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB5;
#[doc = "`read()` method returns [fccob5::R](fccob5::R) reader structure"]
impl crate::Readable for FCCOB5 {}
#[doc = "`write(|w| ..)` method takes [fccob5::W](fccob5::W) writer structure"]
impl crate::Writable for FCCOB5 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob5;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob4](fccob4) module"]
pub type FCCOB4 = crate::Reg<u8, _FCCOB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB4;
#[doc = "`read()` method returns [fccob4::R](fccob4::R) reader structure"]
impl crate::Readable for FCCOB4 {}
#[doc = "`write(|w| ..)` method takes [fccob4::W](fccob4::W) writer structure"]
impl crate::Writable for FCCOB4 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob4;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccobb](fccobb) module"]
pub type FCCOBB = crate::Reg<u8, _FCCOBB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOBB;
#[doc = "`read()` method returns [fccobb::R](fccobb::R) reader structure"]
impl crate::Readable for FCCOBB {}
#[doc = "`write(|w| ..)` method takes [fccobb::W](fccobb::W) writer structure"]
impl crate::Writable for FCCOBB {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccobb;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccoba](fccoba) module"]
pub type FCCOBA = crate::Reg<u8, _FCCOBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOBA;
#[doc = "`read()` method returns [fccoba::R](fccoba::R) reader structure"]
impl crate::Readable for FCCOBA {}
#[doc = "`write(|w| ..)` method takes [fccoba::W](fccoba::W) writer structure"]
impl crate::Writable for FCCOBA {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccoba;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob9](fccob9) module"]
pub type FCCOB9 = crate::Reg<u8, _FCCOB9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB9;
#[doc = "`read()` method returns [fccob9::R](fccob9::R) reader structure"]
impl crate::Readable for FCCOB9 {}
#[doc = "`write(|w| ..)` method takes [fccob9::W](fccob9::W) writer structure"]
impl crate::Writable for FCCOB9 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob9;
#[doc = "Flash Common Command Object Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fccob8](fccob8) module"]
pub type FCCOB8 = crate::Reg<u8, _FCCOB8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCCOB8;
#[doc = "`read()` method returns [fccob8::R](fccob8::R) reader structure"]
impl crate::Readable for FCCOB8 {}
#[doc = "`write(|w| ..)` method takes [fccob8::W](fccob8::W) writer structure"]
impl crate::Writable for FCCOB8 {}
#[doc = "Flash Common Command Object Registers"]
pub mod fccob8;
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot3](fprot3) module"]
pub type FPROT3 = crate::Reg<u8, _FPROT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT3;
#[doc = "`read()` method returns [fprot3::R](fprot3::R) reader structure"]
impl crate::Readable for FPROT3 {}
#[doc = "`write(|w| ..)` method takes [fprot3::W](fprot3::W) writer structure"]
impl crate::Writable for FPROT3 {}
#[doc = "Program Flash Protection Registers"]
pub mod fprot3;
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot2](fprot2) module"]
pub type FPROT2 = crate::Reg<u8, _FPROT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT2;
#[doc = "`read()` method returns [fprot2::R](fprot2::R) reader structure"]
impl crate::Readable for FPROT2 {}
#[doc = "`write(|w| ..)` method takes [fprot2::W](fprot2::W) writer structure"]
impl crate::Writable for FPROT2 {}
#[doc = "Program Flash Protection Registers"]
pub mod fprot2;
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot1](fprot1) module"]
pub type FPROT1 = crate::Reg<u8, _FPROT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT1;
#[doc = "`read()` method returns [fprot1::R](fprot1::R) reader structure"]
impl crate::Readable for FPROT1 {}
#[doc = "`write(|w| ..)` method takes [fprot1::W](fprot1::W) writer structure"]
impl crate::Writable for FPROT1 {}
#[doc = "Program Flash Protection Registers"]
pub mod fprot1;
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot0](fprot0) module"]
pub type FPROT0 = crate::Reg<u8, _FPROT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPROT0;
#[doc = "`read()` method returns [fprot0::R](fprot0::R) reader structure"]
impl crate::Readable for FPROT0 {}
#[doc = "`write(|w| ..)` method takes [fprot0::W](fprot0::W) writer structure"]
impl crate::Writable for FPROT0 {}
#[doc = "Program Flash Protection Registers"]
pub mod fprot0;
#[doc = "EEPROM Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feprot](feprot) module"]
pub type FEPROT = crate::Reg<u8, _FEPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEPROT;
#[doc = "`read()` method returns [feprot::R](feprot::R) reader structure"]
impl crate::Readable for FEPROT {}
#[doc = "`write(|w| ..)` method takes [feprot::W](feprot::W) writer structure"]
impl crate::Writable for FEPROT {}
#[doc = "EEPROM Protection Register"]
pub mod feprot;
#[doc = "Data Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdprot](fdprot) module"]
pub type FDPROT = crate::Reg<u8, _FDPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDPROT;
#[doc = "`read()` method returns [fdprot::R](fdprot::R) reader structure"]
impl crate::Readable for FDPROT {}
#[doc = "`write(|w| ..)` method takes [fdprot::W](fdprot::W) writer structure"]
impl crate::Writable for FDPROT {}
#[doc = "Data Flash Protection Register"]
pub mod fdprot;
#[doc = "Flash CSEc Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcsestat](fcsestat) module"]
pub type FCSESTAT = crate::Reg<u8, _FCSESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCSESTAT;
#[doc = "`read()` method returns [fcsestat::R](fcsestat::R) reader structure"]
impl crate::Readable for FCSESTAT {}
#[doc = "Flash CSEc Status Register"]
pub mod fcsestat;
#[doc = "Flash Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ferstat](ferstat) module"]
pub type FERSTAT = crate::Reg<u8, _FERSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FERSTAT;
#[doc = "`read()` method returns [ferstat::R](ferstat::R) reader structure"]
impl crate::Readable for FERSTAT {}
#[doc = "`write(|w| ..)` method takes [ferstat::W](ferstat::W) writer structure"]
impl crate::Writable for FERSTAT {}
#[doc = "Flash Error Status Register"]
pub mod ferstat;
#[doc = "Flash Error Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fercnfg](fercnfg) module"]
pub type FERCNFG = crate::Reg<u8, _FERCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FERCNFG;
#[doc = "`read()` method returns [fercnfg::R](fercnfg::R) reader structure"]
impl crate::Readable for FERCNFG {}
#[doc = "`write(|w| ..)` method takes [fercnfg::W](fercnfg::W) writer structure"]
impl crate::Writable for FERCNFG {}
#[doc = "Flash Error Configuration Register"]
pub mod fercnfg;
