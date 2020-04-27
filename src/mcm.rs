#[doc = r"Register block"]
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
    _reserved4: [u8; 28usize],
    #[doc = "0x30 - Process ID Register"]
    pub pid: PID,
    _reserved5: [u8; 12usize],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
    _reserved6: [u8; 956usize],
    #[doc = "0x400 - Local Memory Descriptor Register"]
    pub lmdr0: LMDR0,
    #[doc = "0x404 - Local Memory Descriptor Register"]
    pub lmdr1: LMDR1,
    #[doc = "0x408 - Local Memory Descriptor Register2"]
    pub lmdr2: LMDR2,
    _reserved9: [u8; 116usize],
    #[doc = "0x480 - LMEM Parity and ECC Control Register"]
    pub lmpecr: LMPECR,
    _reserved10: [u8; 4usize],
    #[doc = "0x488 - LMEM Parity and ECC Interrupt Register"]
    pub lmpeir: LMPEIR,
    _reserved11: [u8; 4usize],
    #[doc = "0x490 - LMEM Fault Address Register"]
    pub lmfar: LMFAR,
    #[doc = "0x494 - LMEM Fault Attribute Register"]
    pub lmfatr: LMFATR,
    _reserved13: [u8; 8usize],
    #[doc = "0x4a0 - LMEM Fault Data High Register"]
    pub lmfdhr: LMFDHR,
    #[doc = "0x4a4 - LMEM Fault Data Low Register"]
    pub lmfdlr: LMFDLR,
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plasc](plasc) module"]
pub type PLASC = crate::Reg<u16, _PLASC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLASC;
#[doc = "`read()` method returns [plasc::R](plasc::R) reader structure"]
impl crate::Readable for PLASC {}
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plamc](plamc) module"]
pub type PLAMC = crate::Reg<u16, _PLAMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLAMC;
#[doc = "`read()` method returns [plamc::R](plamc::R) reader structure"]
impl crate::Readable for PLAMC {}
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "Core Platform Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpcr](cpcr) module"]
pub type CPCR = crate::Reg<u32, _CPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPCR;
#[doc = "`read()` method returns [cpcr::R](cpcr::R) reader structure"]
impl crate::Readable for CPCR {}
#[doc = "`write(|w| ..)` method takes [cpcr::W](cpcr::W) writer structure"]
impl crate::Writable for CPCR {}
#[doc = "Core Platform Control Register"]
pub mod cpcr;
#[doc = "Interrupt Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iscr](iscr) module"]
pub type ISCR = crate::Reg<u32, _ISCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISCR;
#[doc = "`read()` method returns [iscr::R](iscr::R) reader structure"]
impl crate::Readable for ISCR {}
#[doc = "`write(|w| ..)` method takes [iscr::W](iscr::W) writer structure"]
impl crate::Writable for ISCR {}
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
#[doc = "Process ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "`write(|w| ..)` method takes [pid::W](pid::W) writer structure"]
impl crate::Writable for PID {}
#[doc = "Process ID Register"]
pub mod pid;
#[doc = "Compute Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpo](cpo) module"]
pub type CPO = crate::Reg<u32, _CPO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPO;
#[doc = "`read()` method returns [cpo::R](cpo::R) reader structure"]
impl crate::Readable for CPO {}
#[doc = "`write(|w| ..)` method takes [cpo::W](cpo::W) writer structure"]
impl crate::Writable for CPO {}
#[doc = "Compute Operation Control Register"]
pub mod cpo;
#[doc = "Local Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr0](lmdr0) module"]
pub type LMDR0 = crate::Reg<u32, _LMDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMDR0;
#[doc = "`read()` method returns [lmdr0::R](lmdr0::R) reader structure"]
impl crate::Readable for LMDR0 {}
#[doc = "`write(|w| ..)` method takes [lmdr0::W](lmdr0::W) writer structure"]
impl crate::Writable for LMDR0 {}
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr0;
#[doc = "Local Memory Descriptor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr1](lmdr1) module"]
pub type LMDR1 = crate::Reg<u32, _LMDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMDR1;
#[doc = "`read()` method returns [lmdr1::R](lmdr1::R) reader structure"]
impl crate::Readable for LMDR1 {}
#[doc = "`write(|w| ..)` method takes [lmdr1::W](lmdr1::W) writer structure"]
impl crate::Writable for LMDR1 {}
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr1;
#[doc = "Local Memory Descriptor Register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmdr2](lmdr2) module"]
pub type LMDR2 = crate::Reg<u32, _LMDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMDR2;
#[doc = "`read()` method returns [lmdr2::R](lmdr2::R) reader structure"]
impl crate::Readable for LMDR2 {}
#[doc = "`write(|w| ..)` method takes [lmdr2::W](lmdr2::W) writer structure"]
impl crate::Writable for LMDR2 {}
#[doc = "Local Memory Descriptor Register2"]
pub mod lmdr2;
#[doc = "LMEM Parity and ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmpecr](lmpecr) module"]
pub type LMPECR = crate::Reg<u32, _LMPECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMPECR;
#[doc = "`read()` method returns [lmpecr::R](lmpecr::R) reader structure"]
impl crate::Readable for LMPECR {}
#[doc = "`write(|w| ..)` method takes [lmpecr::W](lmpecr::W) writer structure"]
impl crate::Writable for LMPECR {}
#[doc = "LMEM Parity and ECC Control Register"]
pub mod lmpecr;
#[doc = "LMEM Parity and ECC Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmpeir](lmpeir) module"]
pub type LMPEIR = crate::Reg<u32, _LMPEIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMPEIR;
#[doc = "`read()` method returns [lmpeir::R](lmpeir::R) reader structure"]
impl crate::Readable for LMPEIR {}
#[doc = "`write(|w| ..)` method takes [lmpeir::W](lmpeir::W) writer structure"]
impl crate::Writable for LMPEIR {}
#[doc = "LMEM Parity and ECC Interrupt Register"]
pub mod lmpeir;
#[doc = "LMEM Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfar](lmfar) module"]
pub type LMFAR = crate::Reg<u32, _LMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMFAR;
#[doc = "`read()` method returns [lmfar::R](lmfar::R) reader structure"]
impl crate::Readable for LMFAR {}
#[doc = "LMEM Fault Address Register"]
pub mod lmfar;
#[doc = "LMEM Fault Attribute Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfatr](lmfatr) module"]
pub type LMFATR = crate::Reg<u32, _LMFATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMFATR;
#[doc = "`read()` method returns [lmfatr::R](lmfatr::R) reader structure"]
impl crate::Readable for LMFATR {}
#[doc = "LMEM Fault Attribute Register"]
pub mod lmfatr;
#[doc = "LMEM Fault Data High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfdhr](lmfdhr) module"]
pub type LMFDHR = crate::Reg<u32, _LMFDHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMFDHR;
#[doc = "`read()` method returns [lmfdhr::R](lmfdhr::R) reader structure"]
impl crate::Readable for LMFDHR {}
#[doc = "LMEM Fault Data High Register"]
pub mod lmfdhr;
#[doc = "LMEM Fault Data Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfdlr](lmfdlr) module"]
pub type LMFDLR = crate::Reg<u32, _LMFDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMFDLR;
#[doc = "`read()` method returns [lmfdlr::R](lmfdlr::R) reader structure"]
impl crate::Readable for LMFDLR {}
#[doc = "LMEM Fault Data Low Register"]
pub mod lmfdlr;
