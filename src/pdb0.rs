#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status and Control register"]
    pub sc: SC,
    #[doc = "0x04 - Modulus register"]
    pub mod_: MOD,
    #[doc = "0x08 - Counter register"]
    pub cnt: CNT,
    #[doc = "0x0c - Interrupt Delay register"]
    pub idly: IDLY,
    #[doc = "0x10 - Channel n Control register 1"]
    pub ch0c1: CHC1,
    #[doc = "0x14 - Channel n Status register"]
    pub ch0s: CHS,
    #[doc = "0x18 - Channel n Delay 0 register"]
    pub ch0dly0: CHDLY0,
    #[doc = "0x1c - Channel n Delay 1 register"]
    pub ch0dly1: CHDLY1,
    #[doc = "0x20 - Channel n Delay 2 register"]
    pub ch0dly2: CHDLY2,
    #[doc = "0x24 - Channel n Delay 3 register"]
    pub ch0dly3: CHDLY3,
    #[doc = "0x28 - Channel n Delay 4 register"]
    pub ch0dly4: CHDLY4,
    #[doc = "0x2c - Channel n Delay 5 register"]
    pub ch0dly5: CHDLY5,
    #[doc = "0x30 - Channel n Delay 6 register"]
    pub ch0dly6: CHDLY6,
    #[doc = "0x34 - Channel n Delay 7 register"]
    pub ch0dly7: CHDLY7,
    #[doc = "0x38 - Channel n Control register 1"]
    pub ch1c1: CHC1,
    #[doc = "0x3c - Channel n Status register"]
    pub ch1s: CHS,
    #[doc = "0x40 - Channel n Delay 0 register"]
    pub ch1dly0: CHDLY0,
    #[doc = "0x44 - Channel n Delay 1 register"]
    pub ch1dly1: CHDLY1,
    #[doc = "0x48 - Channel n Delay 2 register"]
    pub ch1dly2: CHDLY2,
    #[doc = "0x4c - Channel n Delay 3 register"]
    pub ch1dly3: CHDLY3,
    #[doc = "0x50 - Channel n Delay 4 register"]
    pub ch1dly4: CHDLY4,
    #[doc = "0x54 - Channel n Delay 5 register"]
    pub ch1dly5: CHDLY5,
    #[doc = "0x58 - Channel n Delay 6 register"]
    pub ch1dly6: CHDLY6,
    #[doc = "0x5c - Channel n Delay 7 register"]
    pub ch1dly7: CHDLY7,
    _reserved24: [u8; 304usize],
    #[doc = "0x190 - Pulse-Out n Enable register"]
    pub poen: POEN,
    _reserved_25_dly1: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x194 - PDB0_DLY2 register."]
    #[inline(always)]
    pub fn dly2(&self) -> &DLY2 {
        unsafe { &*(((self as *const Self) as *const u8).add(404usize) as *const DLY2) }
    }
    #[doc = "0x194 - PDB0_DLY2 register."]
    #[inline(always)]
    pub fn dly2_mut(&self) -> &mut DLY2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(404usize) as *mut DLY2) }
    }
    #[doc = "0x194 - Pulse-Out n Delay register"]
    #[inline(always)]
    pub fn podly(&self) -> &PODLY {
        unsafe { &*(((self as *const Self) as *const u8).add(404usize) as *const PODLY) }
    }
    #[doc = "0x194 - Pulse-Out n Delay register"]
    #[inline(always)]
    pub fn podly_mut(&self) -> &mut PODLY {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(404usize) as *mut PODLY) }
    }
    #[doc = "0x196 - PDB0_DLY1 register."]
    #[inline(always)]
    pub fn dly1(&self) -> &DLY1 {
        unsafe { &*(((self as *const Self) as *const u8).add(406usize) as *const DLY1) }
    }
    #[doc = "0x196 - PDB0_DLY1 register."]
    #[inline(always)]
    pub fn dly1_mut(&self) -> &mut DLY1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(406usize) as *mut DLY1) }
    }
}
#[doc = "Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Status and Control register"]
pub mod sc;
#[doc = "Modulus register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Modulus register"]
pub mod mod_;
#[doc = "Counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Interrupt Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idly](idly) module"]
pub type IDLY = crate::Reg<u32, _IDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLY;
#[doc = "`read()` method returns [idly::R](idly::R) reader structure"]
impl crate::Readable for IDLY {}
#[doc = "`write(|w| ..)` method takes [idly::W](idly::W) writer structure"]
impl crate::Writable for IDLY {}
#[doc = "Interrupt Delay register"]
pub mod idly;
#[doc = "Channel n Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc1](chc1) module"]
pub type CHC1 = crate::Reg<u32, _CHC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHC1;
#[doc = "`read()` method returns [chc1::R](chc1::R) reader structure"]
impl crate::Readable for CHC1 {}
#[doc = "`write(|w| ..)` method takes [chc1::W](chc1::W) writer structure"]
impl crate::Writable for CHC1 {}
#[doc = "Channel n Control register 1"]
pub mod chc1;
#[doc = "Channel n Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chs](chs) module"]
pub type CHS = crate::Reg<u32, _CHS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHS;
#[doc = "`read()` method returns [chs::R](chs::R) reader structure"]
impl crate::Readable for CHS {}
#[doc = "`write(|w| ..)` method takes [chs::W](chs::W) writer structure"]
impl crate::Writable for CHS {}
#[doc = "Channel n Status register"]
pub mod chs;
#[doc = "Channel n Delay 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly0](chdly0) module"]
pub type CHDLY0 = crate::Reg<u32, _CHDLY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY0;
#[doc = "`read()` method returns [chdly0::R](chdly0::R) reader structure"]
impl crate::Readable for CHDLY0 {}
#[doc = "`write(|w| ..)` method takes [chdly0::W](chdly0::W) writer structure"]
impl crate::Writable for CHDLY0 {}
#[doc = "Channel n Delay 0 register"]
pub mod chdly0;
#[doc = "Channel n Delay 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly1](chdly1) module"]
pub type CHDLY1 = crate::Reg<u32, _CHDLY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY1;
#[doc = "`read()` method returns [chdly1::R](chdly1::R) reader structure"]
impl crate::Readable for CHDLY1 {}
#[doc = "`write(|w| ..)` method takes [chdly1::W](chdly1::W) writer structure"]
impl crate::Writable for CHDLY1 {}
#[doc = "Channel n Delay 1 register"]
pub mod chdly1;
#[doc = "Channel n Delay 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly2](chdly2) module"]
pub type CHDLY2 = crate::Reg<u32, _CHDLY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY2;
#[doc = "`read()` method returns [chdly2::R](chdly2::R) reader structure"]
impl crate::Readable for CHDLY2 {}
#[doc = "`write(|w| ..)` method takes [chdly2::W](chdly2::W) writer structure"]
impl crate::Writable for CHDLY2 {}
#[doc = "Channel n Delay 2 register"]
pub mod chdly2;
#[doc = "Channel n Delay 3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly3](chdly3) module"]
pub type CHDLY3 = crate::Reg<u32, _CHDLY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY3;
#[doc = "`read()` method returns [chdly3::R](chdly3::R) reader structure"]
impl crate::Readable for CHDLY3 {}
#[doc = "`write(|w| ..)` method takes [chdly3::W](chdly3::W) writer structure"]
impl crate::Writable for CHDLY3 {}
#[doc = "Channel n Delay 3 register"]
pub mod chdly3;
#[doc = "Channel n Delay 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly4](chdly4) module"]
pub type CHDLY4 = crate::Reg<u32, _CHDLY4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY4;
#[doc = "`read()` method returns [chdly4::R](chdly4::R) reader structure"]
impl crate::Readable for CHDLY4 {}
#[doc = "`write(|w| ..)` method takes [chdly4::W](chdly4::W) writer structure"]
impl crate::Writable for CHDLY4 {}
#[doc = "Channel n Delay 4 register"]
pub mod chdly4;
#[doc = "Channel n Delay 5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly5](chdly5) module"]
pub type CHDLY5 = crate::Reg<u32, _CHDLY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY5;
#[doc = "`read()` method returns [chdly5::R](chdly5::R) reader structure"]
impl crate::Readable for CHDLY5 {}
#[doc = "`write(|w| ..)` method takes [chdly5::W](chdly5::W) writer structure"]
impl crate::Writable for CHDLY5 {}
#[doc = "Channel n Delay 5 register"]
pub mod chdly5;
#[doc = "Channel n Delay 6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly6](chdly6) module"]
pub type CHDLY6 = crate::Reg<u32, _CHDLY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY6;
#[doc = "`read()` method returns [chdly6::R](chdly6::R) reader structure"]
impl crate::Readable for CHDLY6 {}
#[doc = "`write(|w| ..)` method takes [chdly6::W](chdly6::W) writer structure"]
impl crate::Writable for CHDLY6 {}
#[doc = "Channel n Delay 6 register"]
pub mod chdly6;
#[doc = "Channel n Delay 7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly7](chdly7) module"]
pub type CHDLY7 = crate::Reg<u32, _CHDLY7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDLY7;
#[doc = "`read()` method returns [chdly7::R](chdly7::R) reader structure"]
impl crate::Readable for CHDLY7 {}
#[doc = "`write(|w| ..)` method takes [chdly7::W](chdly7::W) writer structure"]
impl crate::Writable for CHDLY7 {}
#[doc = "Channel n Delay 7 register"]
pub mod chdly7;
#[doc = "Pulse-Out n Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen](poen) module"]
pub type POEN = crate::Reg<u32, _POEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POEN;
#[doc = "`read()` method returns [poen::R](poen::R) reader structure"]
impl crate::Readable for POEN {}
#[doc = "`write(|w| ..)` method takes [poen::W](poen::W) writer structure"]
impl crate::Writable for POEN {}
#[doc = "Pulse-Out n Enable register"]
pub mod poen;
#[doc = "Pulse-Out n Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [podly](podly) module"]
pub type PODLY = crate::Reg<u32, _PODLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PODLY;
#[doc = "`read()` method returns [podly::R](podly::R) reader structure"]
impl crate::Readable for PODLY {}
#[doc = "`write(|w| ..)` method takes [podly::W](podly::W) writer structure"]
impl crate::Writable for PODLY {}
#[doc = "Pulse-Out n Delay register"]
pub mod podly;
#[doc = "PDB0_DLY2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly2](dly2) module"]
pub type DLY2 = crate::Reg<u16, _DLY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLY2;
#[doc = "`read()` method returns [dly2::R](dly2::R) reader structure"]
impl crate::Readable for DLY2 {}
#[doc = "`write(|w| ..)` method takes [dly2::W](dly2::W) writer structure"]
impl crate::Writable for DLY2 {}
#[doc = "PDB0_DLY2 register."]
pub mod dly2;
#[doc = "PDB0_DLY1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly1](dly1) module"]
pub type DLY1 = crate::Reg<u16, _DLY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLY1;
#[doc = "`read()` method returns [dly1::R](dly1::R) reader structure"]
impl crate::Readable for DLY1 {}
#[doc = "`write(|w| ..)` method takes [dly1::W](dly1::W) writer structure"]
impl crate::Writable for DLY1 {}
#[doc = "PDB0_DLY1 register."]
pub mod dly1;
