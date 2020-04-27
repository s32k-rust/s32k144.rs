#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub lmem_pcccr: LMEM_PCCCR,
    #[doc = "0x04 - Cache line control register"]
    pub lmem_pcclcr: LMEM_PCCLCR,
    #[doc = "0x08 - Cache search address register"]
    pub lmem_pccsar: LMEM_PCCSAR,
    #[doc = "0x0c - Cache read/write value register"]
    pub lmem_pcccvr: LMEM_PCCCVR,
    _reserved4: [u8; 16usize],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: PCCRMR,
}
#[doc = "Cache control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmem_pcccr](lmem_pcccr) module"]
pub type LMEM_PCCCR = crate::Reg<u32, _LMEM_PCCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMEM_PCCCR;
#[doc = "`read()` method returns [lmem_pcccr::R](lmem_pcccr::R) reader structure"]
impl crate::Readable for LMEM_PCCCR {}
#[doc = "`write(|w| ..)` method takes [lmem_pcccr::W](lmem_pcccr::W) writer structure"]
impl crate::Writable for LMEM_PCCCR {}
#[doc = "Cache control register"]
pub mod lmem_pcccr;
#[doc = "Cache line control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmem_pcclcr](lmem_pcclcr) module"]
pub type LMEM_PCCLCR = crate::Reg<u32, _LMEM_PCCLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMEM_PCCLCR;
#[doc = "`read()` method returns [lmem_pcclcr::R](lmem_pcclcr::R) reader structure"]
impl crate::Readable for LMEM_PCCLCR {}
#[doc = "`write(|w| ..)` method takes [lmem_pcclcr::W](lmem_pcclcr::W) writer structure"]
impl crate::Writable for LMEM_PCCLCR {}
#[doc = "Cache line control register"]
pub mod lmem_pcclcr;
#[doc = "Cache search address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmem_pccsar](lmem_pccsar) module"]
pub type LMEM_PCCSAR = crate::Reg<u32, _LMEM_PCCSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMEM_PCCSAR;
#[doc = "`read()` method returns [lmem_pccsar::R](lmem_pccsar::R) reader structure"]
impl crate::Readable for LMEM_PCCSAR {}
#[doc = "`write(|w| ..)` method takes [lmem_pccsar::W](lmem_pccsar::W) writer structure"]
impl crate::Writable for LMEM_PCCSAR {}
#[doc = "Cache search address register"]
pub mod lmem_pccsar;
#[doc = "Cache read/write value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmem_pcccvr](lmem_pcccvr) module"]
pub type LMEM_PCCCVR = crate::Reg<u32, _LMEM_PCCCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LMEM_PCCCVR;
#[doc = "`read()` method returns [lmem_pcccvr::R](lmem_pcccvr::R) reader structure"]
impl crate::Readable for LMEM_PCCCVR {}
#[doc = "`write(|w| ..)` method takes [lmem_pcccvr::W](lmem_pcccvr::W) writer structure"]
impl crate::Writable for LMEM_PCCCVR {}
#[doc = "Cache read/write value register"]
pub mod lmem_pcccvr;
#[doc = "Cache regions mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pccrmr](pccrmr) module"]
pub type PCCRMR = crate::Reg<u32, _PCCRMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCCRMR;
#[doc = "`read()` method returns [pccrmr::R](pccrmr::R) reader structure"]
impl crate::Readable for PCCRMR {}
#[doc = "`write(|w| ..)` method takes [pccrmr::W](pccrmr::W) writer structure"]
impl crate::Writable for PCCRMR {}
#[doc = "Cache regions mode register"]
pub mod pccrmr;
