#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - Module Control Register"]
    pub mcr: MCR,
    #[doc = "0x0c - Module Status Register"]
    pub msr: MSR,
    #[doc = "0x10 - Module Interrupt Enable Register"]
    pub mier: MIER,
    #[doc = "0x14 - Set Timer Enable Register"]
    pub setten: SETTEN,
    #[doc = "0x18 - Clear Timer Enable Register"]
    pub clrten: CLRTEN,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Timer Value Register"]
    pub tval0: TVAL0,
    #[doc = "0x24 - Current Timer Value"]
    pub cval0: CVAL0,
    #[doc = "0x28 - Timer Control Register"]
    pub tctrl0: TCTRL0,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Timer Value Register"]
    pub tval1: TVAL1,
    #[doc = "0x34 - Current Timer Value"]
    pub cval1: CVAL1,
    #[doc = "0x38 - Timer Control Register"]
    pub tctrl1: TCTRL1,
    _reserved13: [u8; 4usize],
    #[doc = "0x40 - Timer Value Register"]
    pub tval2: TVAL2,
    #[doc = "0x44 - Current Timer Value"]
    pub cval2: CVAL2,
    #[doc = "0x48 - Timer Control Register"]
    pub tctrl2: TCTRL2,
    _reserved16: [u8; 4usize],
    #[doc = "0x50 - Timer Value Register"]
    pub tval3: TVAL3,
    #[doc = "0x54 - Current Timer Value"]
    pub cval3: CVAL3,
    #[doc = "0x58 - Timer Control Register"]
    pub tctrl3: TCTRL3,
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "Module Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "Module Status Register"]
pub mod msr;
#[doc = "Module Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mier](mier) module"]
pub type MIER = crate::Reg<u32, _MIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIER;
#[doc = "`read()` method returns [mier::R](mier::R) reader structure"]
impl crate::Readable for MIER {}
#[doc = "`write(|w| ..)` method takes [mier::W](mier::W) writer structure"]
impl crate::Writable for MIER {}
#[doc = "Module Interrupt Enable Register"]
pub mod mier;
#[doc = "Set Timer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setten](setten) module"]
pub type SETTEN = crate::Reg<u32, _SETTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETTEN;
#[doc = "`read()` method returns [setten::R](setten::R) reader structure"]
impl crate::Readable for SETTEN {}
#[doc = "`write(|w| ..)` method takes [setten::W](setten::W) writer structure"]
impl crate::Writable for SETTEN {}
#[doc = "Set Timer Enable Register"]
pub mod setten;
#[doc = "Clear Timer Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrten](clrten) module"]
pub type CLRTEN = crate::Reg<u32, _CLRTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRTEN;
#[doc = "`read()` method returns [clrten::R](clrten::R) reader structure"]
impl crate::Readable for CLRTEN {}
#[doc = "`write(|w| ..)` method takes [clrten::W](clrten::W) writer structure"]
impl crate::Writable for CLRTEN {}
#[doc = "Clear Timer Enable Register"]
pub mod clrten;
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval0](tval0) module"]
pub type TVAL0 = crate::Reg<u32, _TVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TVAL0;
#[doc = "`read()` method returns [tval0::R](tval0::R) reader structure"]
impl crate::Readable for TVAL0 {}
#[doc = "`write(|w| ..)` method takes [tval0::W](tval0::W) writer structure"]
impl crate::Writable for TVAL0 {}
#[doc = "Timer Value Register"]
pub mod tval0;
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval0](cval0) module"]
pub type CVAL0 = crate::Reg<u32, _CVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL0;
#[doc = "`read()` method returns [cval0::R](cval0::R) reader structure"]
impl crate::Readable for CVAL0 {}
#[doc = "Current Timer Value"]
pub mod cval0;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl0](tctrl0) module"]
pub type TCTRL0 = crate::Reg<u32, _TCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL0;
#[doc = "`read()` method returns [tctrl0::R](tctrl0::R) reader structure"]
impl crate::Readable for TCTRL0 {}
#[doc = "`write(|w| ..)` method takes [tctrl0::W](tctrl0::W) writer structure"]
impl crate::Writable for TCTRL0 {}
#[doc = "Timer Control Register"]
pub mod tctrl0;
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval1](tval1) module"]
pub type TVAL1 = crate::Reg<u32, _TVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TVAL1;
#[doc = "`read()` method returns [tval1::R](tval1::R) reader structure"]
impl crate::Readable for TVAL1 {}
#[doc = "`write(|w| ..)` method takes [tval1::W](tval1::W) writer structure"]
impl crate::Writable for TVAL1 {}
#[doc = "Timer Value Register"]
pub mod tval1;
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval1](cval1) module"]
pub type CVAL1 = crate::Reg<u32, _CVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL1;
#[doc = "`read()` method returns [cval1::R](cval1::R) reader structure"]
impl crate::Readable for CVAL1 {}
#[doc = "Current Timer Value"]
pub mod cval1;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl1](tctrl1) module"]
pub type TCTRL1 = crate::Reg<u32, _TCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL1;
#[doc = "`read()` method returns [tctrl1::R](tctrl1::R) reader structure"]
impl crate::Readable for TCTRL1 {}
#[doc = "`write(|w| ..)` method takes [tctrl1::W](tctrl1::W) writer structure"]
impl crate::Writable for TCTRL1 {}
#[doc = "Timer Control Register"]
pub mod tctrl1;
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval2](tval2) module"]
pub type TVAL2 = crate::Reg<u32, _TVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TVAL2;
#[doc = "`read()` method returns [tval2::R](tval2::R) reader structure"]
impl crate::Readable for TVAL2 {}
#[doc = "`write(|w| ..)` method takes [tval2::W](tval2::W) writer structure"]
impl crate::Writable for TVAL2 {}
#[doc = "Timer Value Register"]
pub mod tval2;
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval2](cval2) module"]
pub type CVAL2 = crate::Reg<u32, _CVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL2;
#[doc = "`read()` method returns [cval2::R](cval2::R) reader structure"]
impl crate::Readable for CVAL2 {}
#[doc = "Current Timer Value"]
pub mod cval2;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl2](tctrl2) module"]
pub type TCTRL2 = crate::Reg<u32, _TCTRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL2;
#[doc = "`read()` method returns [tctrl2::R](tctrl2::R) reader structure"]
impl crate::Readable for TCTRL2 {}
#[doc = "`write(|w| ..)` method takes [tctrl2::W](tctrl2::W) writer structure"]
impl crate::Writable for TCTRL2 {}
#[doc = "Timer Control Register"]
pub mod tctrl2;
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tval3](tval3) module"]
pub type TVAL3 = crate::Reg<u32, _TVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TVAL3;
#[doc = "`read()` method returns [tval3::R](tval3::R) reader structure"]
impl crate::Readable for TVAL3 {}
#[doc = "`write(|w| ..)` method takes [tval3::W](tval3::W) writer structure"]
impl crate::Writable for TVAL3 {}
#[doc = "Timer Value Register"]
pub mod tval3;
#[doc = "Current Timer Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval3](cval3) module"]
pub type CVAL3 = crate::Reg<u32, _CVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVAL3;
#[doc = "`read()` method returns [cval3::R](cval3::R) reader structure"]
impl crate::Readable for CVAL3 {}
#[doc = "Current Timer Value"]
pub mod cval3;
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl3](tctrl3) module"]
pub type TCTRL3 = crate::Reg<u32, _TCTRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTRL3;
#[doc = "`read()` method returns [tctrl3::R](tctrl3::R) reader structure"]
impl crate::Readable for TCTRL3 {}
#[doc = "`write(|w| ..)` method takes [tctrl3::W](tctrl3::W) writer structure"]
impl crate::Writable for TCTRL3 {}
#[doc = "Timer Control Register"]
pub mod tctrl3;
