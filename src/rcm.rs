#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - System Reset Status Register"]
    pub srs: SRS,
    #[doc = "0x0c - Reset Pin Control register"]
    pub rpc: RPC,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Sticky System Reset Status Register"]
    pub ssrs: SSRS,
    #[doc = "0x1c - System Reset Interrupt Enable Register"]
    pub srie: SRIE,
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
#[doc = "System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](srs) module"]
pub type SRS = crate::Reg<u32, _SRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS;
#[doc = "`read()` method returns [srs::R](srs::R) reader structure"]
impl crate::Readable for SRS {}
#[doc = "System Reset Status Register"]
pub mod srs;
#[doc = "Reset Pin Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc](rpc) module"]
pub type RPC = crate::Reg<u32, _RPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC;
#[doc = "`read()` method returns [rpc::R](rpc::R) reader structure"]
impl crate::Readable for RPC {}
#[doc = "`write(|w| ..)` method takes [rpc::W](rpc::W) writer structure"]
impl crate::Writable for RPC {}
#[doc = "Reset Pin Control register"]
pub mod rpc;
#[doc = "Sticky System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrs](ssrs) module"]
pub type SSRS = crate::Reg<u32, _SSRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRS;
#[doc = "`read()` method returns [ssrs::R](ssrs::R) reader structure"]
impl crate::Readable for SSRS {}
#[doc = "`write(|w| ..)` method takes [ssrs::W](ssrs::W) writer structure"]
impl crate::Writable for SSRS {}
#[doc = "Sticky System Reset Status Register"]
pub mod ssrs;
#[doc = "System Reset Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srie](srie) module"]
pub type SRIE = crate::Reg<u32, _SRIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIE;
#[doc = "`read()` method returns [srie::R](srie::R) reader structure"]
impl crate::Readable for SRIE {}
#[doc = "`write(|w| ..)` method takes [srie::W](srie::W) writer structure"]
impl crate::Writable for SRIE {}
#[doc = "System Reset Interrupt Enable Register"]
pub mod srie;
