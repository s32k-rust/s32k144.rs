#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Error Injection Module Configuration Register"]
    pub eimcr: EIMCR,
    #[doc = "0x04 - Error Injection Channel Enable register"]
    pub eichen: EICHEN,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Error Injection Channel Descriptor n, Word0"]
    pub eichd0_word0: EICHD0_WORD0,
    #[doc = "0x104 - Error Injection Channel Descriptor n, Word1"]
    pub eichd0_word1: EICHD0_WORD1,
    _reserved4: [u8; 248usize],
    #[doc = "0x200 - Error Injection Channel Descriptor n, Word0"]
    pub eichd1_word0: EICHD1_WORD0,
    #[doc = "0x204 - Error Injection Channel Descriptor n, Word1"]
    pub eichd1_word1: EICHD1_WORD1,
}
#[doc = "Error Injection Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimcr](eimcr) module"]
pub type EIMCR = crate::Reg<u32, _EIMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIMCR;
#[doc = "`read()` method returns [eimcr::R](eimcr::R) reader structure"]
impl crate::Readable for EIMCR {}
#[doc = "`write(|w| ..)` method takes [eimcr::W](eimcr::W) writer structure"]
impl crate::Writable for EIMCR {}
#[doc = "Error Injection Module Configuration Register"]
pub mod eimcr;
#[doc = "Error Injection Channel Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichen](eichen) module"]
pub type EICHEN = crate::Reg<u32, _EICHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICHEN;
#[doc = "`read()` method returns [eichen::R](eichen::R) reader structure"]
impl crate::Readable for EICHEN {}
#[doc = "`write(|w| ..)` method takes [eichen::W](eichen::W) writer structure"]
impl crate::Writable for EICHEN {}
#[doc = "Error Injection Channel Enable register"]
pub mod eichen;
#[doc = "Error Injection Channel Descriptor n, Word0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd0_word0](eichd0_word0) module"]
pub type EICHD0_WORD0 = crate::Reg<u32, _EICHD0_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICHD0_WORD0;
#[doc = "`read()` method returns [eichd0_word0::R](eichd0_word0::R) reader structure"]
impl crate::Readable for EICHD0_WORD0 {}
#[doc = "`write(|w| ..)` method takes [eichd0_word0::W](eichd0_word0::W) writer structure"]
impl crate::Writable for EICHD0_WORD0 {}
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd0_word0;
#[doc = "Error Injection Channel Descriptor n, Word1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd0_word1](eichd0_word1) module"]
pub type EICHD0_WORD1 = crate::Reg<u32, _EICHD0_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICHD0_WORD1;
#[doc = "`read()` method returns [eichd0_word1::R](eichd0_word1::R) reader structure"]
impl crate::Readable for EICHD0_WORD1 {}
#[doc = "`write(|w| ..)` method takes [eichd0_word1::W](eichd0_word1::W) writer structure"]
impl crate::Writable for EICHD0_WORD1 {}
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd0_word1;
#[doc = "Error Injection Channel Descriptor n, Word0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd1_word0](eichd1_word0) module"]
pub type EICHD1_WORD0 = crate::Reg<u32, _EICHD1_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICHD1_WORD0;
#[doc = "`read()` method returns [eichd1_word0::R](eichd1_word0::R) reader structure"]
impl crate::Readable for EICHD1_WORD0 {}
#[doc = "`write(|w| ..)` method takes [eichd1_word0::W](eichd1_word0::W) writer structure"]
impl crate::Writable for EICHD1_WORD0 {}
#[doc = "Error Injection Channel Descriptor n, Word0"]
pub mod eichd1_word0;
#[doc = "Error Injection Channel Descriptor n, Word1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd1_word1](eichd1_word1) module"]
pub type EICHD1_WORD1 = crate::Reg<u32, _EICHD1_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EICHD1_WORD1;
#[doc = "`read()` method returns [eichd1_word1::R](eichd1_word1::R) reader structure"]
impl crate::Readable for EICHD1_WORD1 {}
#[doc = "`write(|w| ..)` method takes [eichd1_word1::W](eichd1_word1::W) writer structure"]
impl crate::Writable for EICHD1_WORD1 {}
#[doc = "Error Injection Channel Descriptor n, Word1"]
pub mod eichd1_word1;
