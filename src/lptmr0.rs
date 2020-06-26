#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Timer Control Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - Low Power Timer Prescale Register"]
    pub psr: PSR,
    #[doc = "0x08 - Low Power Timer Compare Register"]
    pub cmr: CMR,
    #[doc = "0x0c - Low Power Timer Counter Register"]
    pub cnr: CNR,
}
#[doc = "Low Power Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "Low Power Timer Control Status Register"]
pub mod csr;
#[doc = "Low Power Timer Prescale Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "`write(|w| ..)` method takes [psr::W](psr::W) writer structure"]
impl crate::Writable for PSR {}
#[doc = "Low Power Timer Prescale Register"]
pub mod psr;
#[doc = "Low Power Timer Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](cmr) module"]
pub type CMR = crate::Reg<u32, _CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMR;
#[doc = "`read()` method returns [cmr::R](cmr::R) reader structure"]
impl crate::Readable for CMR {}
#[doc = "`write(|w| ..)` method takes [cmr::W](cmr::W) writer structure"]
impl crate::Writable for CMR {}
#[doc = "Low Power Timer Compare Register"]
pub mod cmr;
#[doc = "Low Power Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr](cnr) module"]
pub type CNR = crate::Reg<u32, _CNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNR;
#[doc = "`read()` method returns [cnr::R](cnr::R) reader structure"]
impl crate::Readable for CNR {}
#[doc = "`write(|w| ..)` method takes [cnr::W](cnr::W) writer structure"]
impl crate::Writable for CNR {}
#[doc = "Low Power Timer Counter Register"]
pub mod cnr;
