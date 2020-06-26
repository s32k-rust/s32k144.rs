#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    pub pdor: PDOR,
    #[doc = "0x04 - Port Set Output Register"]
    pub psor: PSOR,
    #[doc = "0x08 - Port Clear Output Register"]
    pub pcor: PCOR,
    #[doc = "0x0c - Port Toggle Output Register"]
    pub ptor: PTOR,
    #[doc = "0x10 - Port Data Input Register"]
    pub pdir: PDIR,
    #[doc = "0x14 - Port Data Direction Register"]
    pub pddr: PDDR,
    #[doc = "0x18 - Port Input Disable Register"]
    pub pidr: PIDR,
}
#[doc = "Port Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdor](pdor) module"]
pub type PDOR = crate::Reg<u32, _PDOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDOR;
#[doc = "`read()` method returns [pdor::R](pdor::R) reader structure"]
impl crate::Readable for PDOR {}
#[doc = "`write(|w| ..)` method takes [pdor::W](pdor::W) writer structure"]
impl crate::Writable for PDOR {}
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "Port Set Output Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psor](psor) module"]
pub type PSOR = crate::Reg<u32, _PSOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSOR;
#[doc = "`write(|w| ..)` method takes [psor::W](psor::W) writer structure"]
impl crate::Writable for PSOR {}
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "Port Clear Output Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcor](pcor) module"]
pub type PCOR = crate::Reg<u32, _PCOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCOR;
#[doc = "`write(|w| ..)` method takes [pcor::W](pcor::W) writer structure"]
impl crate::Writable for PCOR {}
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "Port Toggle Output Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptor](ptor) module"]
pub type PTOR = crate::Reg<u32, _PTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTOR;
#[doc = "`write(|w| ..)` method takes [ptor::W](ptor::W) writer structure"]
impl crate::Writable for PTOR {}
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "Port Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdir](pdir) module"]
pub type PDIR = crate::Reg<u32, _PDIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDIR;
#[doc = "`read()` method returns [pdir::R](pdir::R) reader structure"]
impl crate::Readable for PDIR {}
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "Port Data Direction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddr](pddr) module"]
pub type PDDR = crate::Reg<u32, _PDDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDDR;
#[doc = "`read()` method returns [pddr::R](pddr::R) reader structure"]
impl crate::Readable for PDDR {}
#[doc = "`write(|w| ..)` method takes [pddr::W](pddr::W) writer structure"]
impl crate::Writable for PDDR {}
#[doc = "Port Data Direction Register"]
pub mod pddr;
#[doc = "Port Input Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](pidr) module"]
pub type PIDR = crate::Reg<u32, _PIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR;
#[doc = "`read()` method returns [pidr::R](pidr::R) reader structure"]
impl crate::Readable for PIDR {}
#[doc = "`write(|w| ..)` method takes [pidr::W](pidr::W) writer structure"]
impl crate::Writable for PIDR {}
#[doc = "Port Input Disable Register"]
pub mod pidr;
