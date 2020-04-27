#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - SMC Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - Power Mode Protection register"]
    pub pmprot: PMPROT,
    #[doc = "0x0c - Power Mode Control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x10 - Stop Control Register"]
    pub stopctrl: STOPCTRL,
    #[doc = "0x14 - Power Mode Status register"]
    pub pmstat: PMSTAT,
}
#[doc = "SMC Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](verid) module"]
pub type VERID = crate::Reg<u32, _VERID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERID;
#[doc = "`read()` method returns [verid::R](verid::R) reader structure"]
impl crate::Readable for VERID {}
#[doc = "SMC Version ID Register"]
pub mod verid;
#[doc = "SMC Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](param) module"]
pub type PARAM = crate::Reg<u32, _PARAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PARAM;
#[doc = "`read()` method returns [param::R](param::R) reader structure"]
impl crate::Readable for PARAM {}
#[doc = "SMC Parameter Register"]
pub mod param;
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmprot](pmprot) module"]
pub type PMPROT = crate::Reg<u32, _PMPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMPROT;
#[doc = "`read()` method returns [pmprot::R](pmprot::R) reader structure"]
impl crate::Readable for PMPROT {}
#[doc = "`write(|w| ..)` method takes [pmprot::W](pmprot::W) writer structure"]
impl crate::Writable for PMPROT {}
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "Power Mode Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctrl](pmctrl) module"]
pub type PMCTRL = crate::Reg<u32, _PMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCTRL;
#[doc = "`read()` method returns [pmctrl::R](pmctrl::R) reader structure"]
impl crate::Readable for PMCTRL {}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](pmctrl::W) writer structure"]
impl crate::Writable for PMCTRL {}
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stopctrl](stopctrl) module"]
pub type STOPCTRL = crate::Reg<u32, _STOPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STOPCTRL;
#[doc = "`read()` method returns [stopctrl::R](stopctrl::R) reader structure"]
impl crate::Readable for STOPCTRL {}
#[doc = "`write(|w| ..)` method takes [stopctrl::W](stopctrl::W) writer structure"]
impl crate::Writable for STOPCTRL {}
#[doc = "Stop Control Register"]
pub mod stopctrl;
#[doc = "Power Mode Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstat](pmstat) module"]
pub type PMSTAT = crate::Reg<u32, _PMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTAT;
#[doc = "`read()` method returns [pmstat::R](pmstat::R) reader structure"]
impl crate::Readable for PMSTAT {}
#[doc = "Power Mode Status register"]
pub mod pmstat;
