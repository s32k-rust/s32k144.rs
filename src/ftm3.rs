#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status And Control"]
    pub sc: SC,
    #[doc = "0x04 - Counter"]
    pub cnt: CNT,
    #[doc = "0x08 - Modulo"]
    pub mod_: MOD,
    #[doc = "0x0c - Channel (n) Status And Control"]
    pub c0sc: C0SC,
    #[doc = "0x10 - Channel (n) Value"]
    pub c0v: C0V,
    #[doc = "0x14 - Channel (n) Status And Control"]
    pub c1sc: C1SC,
    #[doc = "0x18 - Channel (n) Value"]
    pub c1v: C1V,
    #[doc = "0x1c - Channel (n) Status And Control"]
    pub c2sc: C2SC,
    #[doc = "0x20 - Channel (n) Value"]
    pub c2v: C2V,
    #[doc = "0x24 - Channel (n) Status And Control"]
    pub c3sc: C3SC,
    #[doc = "0x28 - Channel (n) Value"]
    pub c3v: C3V,
    #[doc = "0x2c - Channel (n) Status And Control"]
    pub c4sc: C4SC,
    #[doc = "0x30 - Channel (n) Value"]
    pub c4v: C4V,
    #[doc = "0x34 - Channel (n) Status And Control"]
    pub c5sc: C5SC,
    #[doc = "0x38 - Channel (n) Value"]
    pub c5v: C5V,
    #[doc = "0x3c - Channel (n) Status And Control"]
    pub c6sc: C6SC,
    #[doc = "0x40 - Channel (n) Value"]
    pub c6v: C6V,
    #[doc = "0x44 - Channel (n) Status And Control"]
    pub c7sc: C7SC,
    #[doc = "0x48 - Channel (n) Value"]
    pub c7v: C7V,
    #[doc = "0x4c - Counter Initial Value"]
    pub cntin: CNTIN,
    #[doc = "0x50 - Capture And Compare Status"]
    pub status: STATUS,
    #[doc = "0x54 - Features Mode Selection"]
    pub mode: MODE,
    #[doc = "0x58 - Synchronization"]
    pub sync: SYNC,
    #[doc = "0x5c - Initial State For Channels Output"]
    pub outinit: OUTINIT,
    #[doc = "0x60 - Output Mask"]
    pub outmask: OUTMASK,
    #[doc = "0x64 - Function For Linked Channels"]
    pub combine: COMBINE,
    #[doc = "0x68 - Deadtime Configuration"]
    pub deadtime: DEADTIME,
    #[doc = "0x6c - FTM External Trigger"]
    pub exttrig: EXTTRIG,
    #[doc = "0x70 - Channels Polarity"]
    pub pol: POL,
    #[doc = "0x74 - Fault Mode Status"]
    pub fms: FMS,
    #[doc = "0x78 - Input Capture Filter Control"]
    pub filter: FILTER,
    #[doc = "0x7c - Fault Control"]
    pub fltctrl: FLTCTRL,
    #[doc = "0x80 - Quadrature Decoder Control And Status"]
    pub qdctrl: QDCTRL,
    #[doc = "0x84 - Configuration"]
    pub conf: CONF,
    #[doc = "0x88 - FTM Fault Input Polarity"]
    pub fltpol: FLTPOL,
    #[doc = "0x8c - Synchronization Configuration"]
    pub synconf: SYNCONF,
    #[doc = "0x90 - FTM Inverting Control"]
    pub invctrl: INVCTRL,
    #[doc = "0x94 - FTM Software Output Control"]
    pub swoctrl: SWOCTRL,
    #[doc = "0x98 - FTM PWM Load"]
    pub pwmload: PWMLOAD,
    #[doc = "0x9c - Half Cycle Register"]
    pub hcr: HCR,
    #[doc = "0xa0 - Pair 0 Deadtime Configuration"]
    pub pair0deadtime: PAIR0DEADTIME,
    _reserved41: [u8; 4usize],
    #[doc = "0xa8 - Pair 1 Deadtime Configuration"]
    pub pair1deadtime: PAIR1DEADTIME,
    _reserved42: [u8; 4usize],
    #[doc = "0xb0 - Pair 2 Deadtime Configuration"]
    pub pair2deadtime: PAIR2DEADTIME,
    _reserved43: [u8; 4usize],
    #[doc = "0xb8 - Pair 3 Deadtime Configuration"]
    pub pair3deadtime: PAIR3DEADTIME,
}
#[doc = "Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Status And Control"]
pub mod sc;
#[doc = "Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter"]
pub mod cnt;
#[doc = "Modulo\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Modulo"]
pub mod mod_;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0sc](c0sc) module"]
pub type C0SC = crate::Reg<u32, _C0SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0SC;
#[doc = "`read()` method returns [c0sc::R](c0sc::R) reader structure"]
impl crate::Readable for C0SC {}
#[doc = "`write(|w| ..)` method takes [c0sc::W](c0sc::W) writer structure"]
impl crate::Writable for C0SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c0sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0v](c0v) module"]
pub type C0V = crate::Reg<u32, _C0V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0V;
#[doc = "`read()` method returns [c0v::R](c0v::R) reader structure"]
impl crate::Readable for C0V {}
#[doc = "`write(|w| ..)` method takes [c0v::W](c0v::W) writer structure"]
impl crate::Writable for C0V {}
#[doc = "Channel (n) Value"]
pub mod c0v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1sc](c1sc) module"]
pub type C1SC = crate::Reg<u32, _C1SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1SC;
#[doc = "`read()` method returns [c1sc::R](c1sc::R) reader structure"]
impl crate::Readable for C1SC {}
#[doc = "`write(|w| ..)` method takes [c1sc::W](c1sc::W) writer structure"]
impl crate::Writable for C1SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c1sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1v](c1v) module"]
pub type C1V = crate::Reg<u32, _C1V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1V;
#[doc = "`read()` method returns [c1v::R](c1v::R) reader structure"]
impl crate::Readable for C1V {}
#[doc = "`write(|w| ..)` method takes [c1v::W](c1v::W) writer structure"]
impl crate::Writable for C1V {}
#[doc = "Channel (n) Value"]
pub mod c1v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2sc](c2sc) module"]
pub type C2SC = crate::Reg<u32, _C2SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2SC;
#[doc = "`read()` method returns [c2sc::R](c2sc::R) reader structure"]
impl crate::Readable for C2SC {}
#[doc = "`write(|w| ..)` method takes [c2sc::W](c2sc::W) writer structure"]
impl crate::Writable for C2SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c2sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2v](c2v) module"]
pub type C2V = crate::Reg<u32, _C2V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2V;
#[doc = "`read()` method returns [c2v::R](c2v::R) reader structure"]
impl crate::Readable for C2V {}
#[doc = "`write(|w| ..)` method takes [c2v::W](c2v::W) writer structure"]
impl crate::Writable for C2V {}
#[doc = "Channel (n) Value"]
pub mod c2v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3sc](c3sc) module"]
pub type C3SC = crate::Reg<u32, _C3SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3SC;
#[doc = "`read()` method returns [c3sc::R](c3sc::R) reader structure"]
impl crate::Readable for C3SC {}
#[doc = "`write(|w| ..)` method takes [c3sc::W](c3sc::W) writer structure"]
impl crate::Writable for C3SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c3sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c3v](c3v) module"]
pub type C3V = crate::Reg<u32, _C3V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C3V;
#[doc = "`read()` method returns [c3v::R](c3v::R) reader structure"]
impl crate::Readable for C3V {}
#[doc = "`write(|w| ..)` method takes [c3v::W](c3v::W) writer structure"]
impl crate::Writable for C3V {}
#[doc = "Channel (n) Value"]
pub mod c3v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4sc](c4sc) module"]
pub type C4SC = crate::Reg<u32, _C4SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4SC;
#[doc = "`read()` method returns [c4sc::R](c4sc::R) reader structure"]
impl crate::Readable for C4SC {}
#[doc = "`write(|w| ..)` method takes [c4sc::W](c4sc::W) writer structure"]
impl crate::Writable for C4SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c4sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c4v](c4v) module"]
pub type C4V = crate::Reg<u32, _C4V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C4V;
#[doc = "`read()` method returns [c4v::R](c4v::R) reader structure"]
impl crate::Readable for C4V {}
#[doc = "`write(|w| ..)` method takes [c4v::W](c4v::W) writer structure"]
impl crate::Writable for C4V {}
#[doc = "Channel (n) Value"]
pub mod c4v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5sc](c5sc) module"]
pub type C5SC = crate::Reg<u32, _C5SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5SC;
#[doc = "`read()` method returns [c5sc::R](c5sc::R) reader structure"]
impl crate::Readable for C5SC {}
#[doc = "`write(|w| ..)` method takes [c5sc::W](c5sc::W) writer structure"]
impl crate::Writable for C5SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c5sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5v](c5v) module"]
pub type C5V = crate::Reg<u32, _C5V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C5V;
#[doc = "`read()` method returns [c5v::R](c5v::R) reader structure"]
impl crate::Readable for C5V {}
#[doc = "`write(|w| ..)` method takes [c5v::W](c5v::W) writer structure"]
impl crate::Writable for C5V {}
#[doc = "Channel (n) Value"]
pub mod c5v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6sc](c6sc) module"]
pub type C6SC = crate::Reg<u32, _C6SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6SC;
#[doc = "`read()` method returns [c6sc::R](c6sc::R) reader structure"]
impl crate::Readable for C6SC {}
#[doc = "`write(|w| ..)` method takes [c6sc::W](c6sc::W) writer structure"]
impl crate::Writable for C6SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c6sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c6v](c6v) module"]
pub type C6V = crate::Reg<u32, _C6V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C6V;
#[doc = "`read()` method returns [c6v::R](c6v::R) reader structure"]
impl crate::Readable for C6V {}
#[doc = "`write(|w| ..)` method takes [c6v::W](c6v::W) writer structure"]
impl crate::Writable for C6V {}
#[doc = "Channel (n) Value"]
pub mod c6v;
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7sc](c7sc) module"]
pub type C7SC = crate::Reg<u32, _C7SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7SC;
#[doc = "`read()` method returns [c7sc::R](c7sc::R) reader structure"]
impl crate::Readable for C7SC {}
#[doc = "`write(|w| ..)` method takes [c7sc::W](c7sc::W) writer structure"]
impl crate::Writable for C7SC {}
#[doc = "Channel (n) Status And Control"]
pub mod c7sc;
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7v](c7v) module"]
pub type C7V = crate::Reg<u32, _C7V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C7V;
#[doc = "`read()` method returns [c7v::R](c7v::R) reader structure"]
impl crate::Readable for C7V {}
#[doc = "`write(|w| ..)` method takes [c7v::W](c7v::W) writer structure"]
impl crate::Writable for C7V {}
#[doc = "Channel (n) Value"]
pub mod c7v;
#[doc = "Counter Initial Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntin](cntin) module"]
pub type CNTIN = crate::Reg<u32, _CNTIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTIN;
#[doc = "`read()` method returns [cntin::R](cntin::R) reader structure"]
impl crate::Readable for CNTIN {}
#[doc = "`write(|w| ..)` method takes [cntin::W](cntin::W) writer structure"]
impl crate::Writable for CNTIN {}
#[doc = "Counter Initial Value"]
pub mod cntin;
#[doc = "Capture And Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Capture And Compare Status"]
pub mod status;
#[doc = "Features Mode Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Features Mode Selection"]
pub mod mode;
#[doc = "Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "Synchronization"]
pub mod sync;
#[doc = "Initial State For Channels Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outinit](outinit) module"]
pub type OUTINIT = crate::Reg<u32, _OUTINIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTINIT;
#[doc = "`read()` method returns [outinit::R](outinit::R) reader structure"]
impl crate::Readable for OUTINIT {}
#[doc = "`write(|w| ..)` method takes [outinit::W](outinit::W) writer structure"]
impl crate::Writable for OUTINIT {}
#[doc = "Initial State For Channels Output"]
pub mod outinit;
#[doc = "Output Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outmask](outmask) module"]
pub type OUTMASK = crate::Reg<u32, _OUTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTMASK;
#[doc = "`read()` method returns [outmask::R](outmask::R) reader structure"]
impl crate::Readable for OUTMASK {}
#[doc = "`write(|w| ..)` method takes [outmask::W](outmask::W) writer structure"]
impl crate::Writable for OUTMASK {}
#[doc = "Output Mask"]
pub mod outmask;
#[doc = "Function For Linked Channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combine](combine) module"]
pub type COMBINE = crate::Reg<u32, _COMBINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBINE;
#[doc = "`read()` method returns [combine::R](combine::R) reader structure"]
impl crate::Readable for COMBINE {}
#[doc = "`write(|w| ..)` method takes [combine::W](combine::W) writer structure"]
impl crate::Writable for COMBINE {}
#[doc = "Function For Linked Channels"]
pub mod combine;
#[doc = "Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deadtime](deadtime) module"]
pub type DEADTIME = crate::Reg<u32, _DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEADTIME;
#[doc = "`read()` method returns [deadtime::R](deadtime::R) reader structure"]
impl crate::Readable for DEADTIME {}
#[doc = "`write(|w| ..)` method takes [deadtime::W](deadtime::W) writer structure"]
impl crate::Writable for DEADTIME {}
#[doc = "Deadtime Configuration"]
pub mod deadtime;
#[doc = "FTM External Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exttrig](exttrig) module"]
pub type EXTTRIG = crate::Reg<u32, _EXTTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTTRIG;
#[doc = "`read()` method returns [exttrig::R](exttrig::R) reader structure"]
impl crate::Readable for EXTTRIG {}
#[doc = "`write(|w| ..)` method takes [exttrig::W](exttrig::W) writer structure"]
impl crate::Writable for EXTTRIG {}
#[doc = "FTM External Trigger"]
pub mod exttrig;
#[doc = "Channels Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](pol) module"]
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
#[doc = "`read()` method returns [pol::R](pol::R) reader structure"]
impl crate::Readable for POL {}
#[doc = "`write(|w| ..)` method takes [pol::W](pol::W) writer structure"]
impl crate::Writable for POL {}
#[doc = "Channels Polarity"]
pub mod pol;
#[doc = "Fault Mode Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fms](fms) module"]
pub type FMS = crate::Reg<u32, _FMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMS;
#[doc = "`read()` method returns [fms::R](fms::R) reader structure"]
impl crate::Readable for FMS {}
#[doc = "`write(|w| ..)` method takes [fms::W](fms::W) writer structure"]
impl crate::Writable for FMS {}
#[doc = "Fault Mode Status"]
pub mod fms;
#[doc = "Input Capture Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](filter) module"]
pub type FILTER = crate::Reg<u32, _FILTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FILTER;
#[doc = "`read()` method returns [filter::R](filter::R) reader structure"]
impl crate::Readable for FILTER {}
#[doc = "`write(|w| ..)` method takes [filter::W](filter::W) writer structure"]
impl crate::Writable for FILTER {}
#[doc = "Input Capture Filter Control"]
pub mod filter;
#[doc = "Fault Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltctrl](fltctrl) module"]
pub type FLTCTRL = crate::Reg<u32, _FLTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTCTRL;
#[doc = "`read()` method returns [fltctrl::R](fltctrl::R) reader structure"]
impl crate::Readable for FLTCTRL {}
#[doc = "`write(|w| ..)` method takes [fltctrl::W](fltctrl::W) writer structure"]
impl crate::Writable for FLTCTRL {}
#[doc = "Fault Control"]
pub mod fltctrl;
#[doc = "Quadrature Decoder Control And Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdctrl](qdctrl) module"]
pub type QDCTRL = crate::Reg<u32, _QDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDCTRL;
#[doc = "`read()` method returns [qdctrl::R](qdctrl::R) reader structure"]
impl crate::Readable for QDCTRL {}
#[doc = "`write(|w| ..)` method takes [qdctrl::W](qdctrl::W) writer structure"]
impl crate::Writable for QDCTRL {}
#[doc = "Quadrature Decoder Control And Status"]
pub mod qdctrl;
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](conf) module"]
pub type CONF = crate::Reg<u32, _CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONF;
#[doc = "`read()` method returns [conf::R](conf::R) reader structure"]
impl crate::Readable for CONF {}
#[doc = "`write(|w| ..)` method takes [conf::W](conf::W) writer structure"]
impl crate::Writable for CONF {}
#[doc = "Configuration"]
pub mod conf;
#[doc = "FTM Fault Input Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltpol](fltpol) module"]
pub type FLTPOL = crate::Reg<u32, _FLTPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTPOL;
#[doc = "`read()` method returns [fltpol::R](fltpol::R) reader structure"]
impl crate::Readable for FLTPOL {}
#[doc = "`write(|w| ..)` method takes [fltpol::W](fltpol::W) writer structure"]
impl crate::Writable for FLTPOL {}
#[doc = "FTM Fault Input Polarity"]
pub mod fltpol;
#[doc = "Synchronization Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synconf](synconf) module"]
pub type SYNCONF = crate::Reg<u32, _SYNCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCONF;
#[doc = "`read()` method returns [synconf::R](synconf::R) reader structure"]
impl crate::Readable for SYNCONF {}
#[doc = "`write(|w| ..)` method takes [synconf::W](synconf::W) writer structure"]
impl crate::Writable for SYNCONF {}
#[doc = "Synchronization Configuration"]
pub mod synconf;
#[doc = "FTM Inverting Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invctrl](invctrl) module"]
pub type INVCTRL = crate::Reg<u32, _INVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INVCTRL;
#[doc = "`read()` method returns [invctrl::R](invctrl::R) reader structure"]
impl crate::Readable for INVCTRL {}
#[doc = "`write(|w| ..)` method takes [invctrl::W](invctrl::W) writer structure"]
impl crate::Writable for INVCTRL {}
#[doc = "FTM Inverting Control"]
pub mod invctrl;
#[doc = "FTM Software Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swoctrl](swoctrl) module"]
pub type SWOCTRL = crate::Reg<u32, _SWOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWOCTRL;
#[doc = "`read()` method returns [swoctrl::R](swoctrl::R) reader structure"]
impl crate::Readable for SWOCTRL {}
#[doc = "`write(|w| ..)` method takes [swoctrl::W](swoctrl::W) writer structure"]
impl crate::Writable for SWOCTRL {}
#[doc = "FTM Software Output Control"]
pub mod swoctrl;
#[doc = "FTM PWM Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmload](pwmload) module"]
pub type PWMLOAD = crate::Reg<u32, _PWMLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMLOAD;
#[doc = "`read()` method returns [pwmload::R](pwmload::R) reader structure"]
impl crate::Readable for PWMLOAD {}
#[doc = "`write(|w| ..)` method takes [pwmload::W](pwmload::W) writer structure"]
impl crate::Writable for PWMLOAD {}
#[doc = "FTM PWM Load"]
pub mod pwmload;
#[doc = "Half Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcr](hcr) module"]
pub type HCR = crate::Reg<u32, _HCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCR;
#[doc = "`read()` method returns [hcr::R](hcr::R) reader structure"]
impl crate::Readable for HCR {}
#[doc = "`write(|w| ..)` method takes [hcr::W](hcr::W) writer structure"]
impl crate::Writable for HCR {}
#[doc = "Half Cycle Register"]
pub mod hcr;
#[doc = "Pair 0 Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pair0deadtime](pair0deadtime) module"]
pub type PAIR0DEADTIME = crate::Reg<u32, _PAIR0DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIR0DEADTIME;
#[doc = "`read()` method returns [pair0deadtime::R](pair0deadtime::R) reader structure"]
impl crate::Readable for PAIR0DEADTIME {}
#[doc = "`write(|w| ..)` method takes [pair0deadtime::W](pair0deadtime::W) writer structure"]
impl crate::Writable for PAIR0DEADTIME {}
#[doc = "Pair 0 Deadtime Configuration"]
pub mod pair0deadtime;
#[doc = "Pair 1 Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pair1deadtime](pair1deadtime) module"]
pub type PAIR1DEADTIME = crate::Reg<u32, _PAIR1DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIR1DEADTIME;
#[doc = "`read()` method returns [pair1deadtime::R](pair1deadtime::R) reader structure"]
impl crate::Readable for PAIR1DEADTIME {}
#[doc = "`write(|w| ..)` method takes [pair1deadtime::W](pair1deadtime::W) writer structure"]
impl crate::Writable for PAIR1DEADTIME {}
#[doc = "Pair 1 Deadtime Configuration"]
pub mod pair1deadtime;
#[doc = "Pair 2 Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pair2deadtime](pair2deadtime) module"]
pub type PAIR2DEADTIME = crate::Reg<u32, _PAIR2DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIR2DEADTIME;
#[doc = "`read()` method returns [pair2deadtime::R](pair2deadtime::R) reader structure"]
impl crate::Readable for PAIR2DEADTIME {}
#[doc = "`write(|w| ..)` method takes [pair2deadtime::W](pair2deadtime::W) writer structure"]
impl crate::Writable for PAIR2DEADTIME {}
#[doc = "Pair 2 Deadtime Configuration"]
pub mod pair2deadtime;
#[doc = "Pair 3 Deadtime Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pair3deadtime](pair3deadtime) module"]
pub type PAIR3DEADTIME = crate::Reg<u32, _PAIR3DEADTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAIR3DEADTIME;
#[doc = "`read()` method returns [pair3deadtime::R](pair3deadtime::R) reader structure"]
impl crate::Readable for PAIR3DEADTIME {}
#[doc = "`write(|w| ..)` method takes [pair3deadtime::W](pair3deadtime::W) writer structure"]
impl crate::Writable for PAIR3DEADTIME {}
#[doc = "Pair 3 Deadtime Configuration"]
pub mod pair3deadtime;
