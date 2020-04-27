#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Chip Control register"]
    pub chipctl: CHIPCTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - FTM Option Register 0"]
    pub ftmopt0: FTMOPT0,
    #[doc = "0x10 - LPO Clock Select Register"]
    pub lpoclks: LPOCLKS,
    _reserved3: [u8; 4usize],
    #[doc = "0x18 - ADC Options Register"]
    pub adcopt: ADCOPT,
    #[doc = "0x1c - FTM Option Register 1"]
    pub ftmopt1: FTMOPT1,
    #[doc = "0x20 - Miscellaneous control register 0"]
    pub misctrl0: MISCTRL0,
    #[doc = "0x24 - System Device Identification Register"]
    pub sdid: SDID,
    _reserved7: [u8; 24usize],
    #[doc = "0x40 - Platform Clock Gating Control Register"]
    pub platcgc: PLATCGC,
    _reserved8: [u8; 8usize],
    #[doc = "0x4c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    _reserved9: [u8; 4usize],
    #[doc = "0x54 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x58 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x5c - Unique Identification Register Mid Low"]
    pub uidml: UIDML,
    #[doc = "0x60 - Unique Identification Register Low"]
    pub uidl: UIDL,
    _reserved13: [u8; 4usize],
    #[doc = "0x68 - System Clock Divider Register 4"]
    pub clkdiv4: CLKDIV4,
    #[doc = "0x6c - Miscellaneous Control register 1"]
    pub misctrl1: MISCTRL1,
}
#[doc = "Chip Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipctl](chipctl) module"]
pub type CHIPCTL = crate::Reg<u32, _CHIPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPCTL;
#[doc = "`read()` method returns [chipctl::R](chipctl::R) reader structure"]
impl crate::Readable for CHIPCTL {}
#[doc = "`write(|w| ..)` method takes [chipctl::W](chipctl::W) writer structure"]
impl crate::Writable for CHIPCTL {}
#[doc = "Chip Control register"]
pub mod chipctl;
#[doc = "FTM Option Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftmopt0](ftmopt0) module"]
pub type FTMOPT0 = crate::Reg<u32, _FTMOPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTMOPT0;
#[doc = "`read()` method returns [ftmopt0::R](ftmopt0::R) reader structure"]
impl crate::Readable for FTMOPT0 {}
#[doc = "`write(|w| ..)` method takes [ftmopt0::W](ftmopt0::W) writer structure"]
impl crate::Writable for FTMOPT0 {}
#[doc = "FTM Option Register 0"]
pub mod ftmopt0;
#[doc = "LPO Clock Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpoclks](lpoclks) module"]
pub type LPOCLKS = crate::Reg<u32, _LPOCLKS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPOCLKS;
#[doc = "`read()` method returns [lpoclks::R](lpoclks::R) reader structure"]
impl crate::Readable for LPOCLKS {}
#[doc = "`write(|w| ..)` method takes [lpoclks::W](lpoclks::W) writer structure"]
impl crate::Writable for LPOCLKS {}
#[doc = "LPO Clock Select Register"]
pub mod lpoclks;
#[doc = "ADC Options Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcopt](adcopt) module"]
pub type ADCOPT = crate::Reg<u32, _ADCOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCOPT;
#[doc = "`read()` method returns [adcopt::R](adcopt::R) reader structure"]
impl crate::Readable for ADCOPT {}
#[doc = "`write(|w| ..)` method takes [adcopt::W](adcopt::W) writer structure"]
impl crate::Writable for ADCOPT {}
#[doc = "ADC Options Register"]
pub mod adcopt;
#[doc = "FTM Option Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftmopt1](ftmopt1) module"]
pub type FTMOPT1 = crate::Reg<u32, _FTMOPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTMOPT1;
#[doc = "`read()` method returns [ftmopt1::R](ftmopt1::R) reader structure"]
impl crate::Readable for FTMOPT1 {}
#[doc = "`write(|w| ..)` method takes [ftmopt1::W](ftmopt1::W) writer structure"]
impl crate::Writable for FTMOPT1 {}
#[doc = "FTM Option Register 1"]
pub mod ftmopt1;
#[doc = "Miscellaneous control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misctrl0](misctrl0) module"]
pub type MISCTRL0 = crate::Reg<u32, _MISCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISCTRL0;
#[doc = "`read()` method returns [misctrl0::R](misctrl0::R) reader structure"]
impl crate::Readable for MISCTRL0 {}
#[doc = "`write(|w| ..)` method takes [misctrl0::W](misctrl0::W) writer structure"]
impl crate::Writable for MISCTRL0 {}
#[doc = "Miscellaneous control register 0"]
pub mod misctrl0;
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdid](sdid) module"]
pub type SDID = crate::Reg<u32, _SDID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDID;
#[doc = "`read()` method returns [sdid::R](sdid::R) reader structure"]
impl crate::Readable for SDID {}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "Platform Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [platcgc](platcgc) module"]
pub type PLATCGC = crate::Reg<u32, _PLATCGC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLATCGC;
#[doc = "`read()` method returns [platcgc::R](platcgc::R) reader structure"]
impl crate::Readable for PLATCGC {}
#[doc = "`write(|w| ..)` method takes [platcgc::W](platcgc::W) writer structure"]
impl crate::Writable for PLATCGC {}
#[doc = "Platform Clock Gating Control Register"]
pub mod platcgc;
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1](fcfg1) module"]
pub type FCFG1 = crate::Reg<u32, _FCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1;
#[doc = "`read()` method returns [fcfg1::R](fcfg1::R) reader structure"]
impl crate::Readable for FCFG1 {}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](fcfg1::W) writer structure"]
impl crate::Writable for FCFG1 {}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Unique Identification Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidh](uidh) module"]
pub type UIDH = crate::Reg<u32, _UIDH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDH;
#[doc = "`read()` method returns [uidh::R](uidh::R) reader structure"]
impl crate::Readable for UIDH {}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid-High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidmh](uidmh) module"]
pub type UIDMH = crate::Reg<u32, _UIDMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDMH;
#[doc = "`read()` method returns [uidmh::R](uidmh::R) reader structure"]
impl crate::Readable for UIDMH {}
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "Unique Identification Register Mid Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidml](uidml) module"]
pub type UIDML = crate::Reg<u32, _UIDML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDML;
#[doc = "`read()` method returns [uidml::R](uidml::R) reader structure"]
impl crate::Readable for UIDML {}
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "Unique Identification Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidl](uidl) module"]
pub type UIDL = crate::Reg<u32, _UIDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UIDL;
#[doc = "`read()` method returns [uidl::R](uidl::R) reader structure"]
impl crate::Readable for UIDL {}
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "System Clock Divider Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv4](clkdiv4) module"]
pub type CLKDIV4 = crate::Reg<u32, _CLKDIV4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV4;
#[doc = "`read()` method returns [clkdiv4::R](clkdiv4::R) reader structure"]
impl crate::Readable for CLKDIV4 {}
#[doc = "`write(|w| ..)` method takes [clkdiv4::W](clkdiv4::W) writer structure"]
impl crate::Writable for CLKDIV4 {}
#[doc = "System Clock Divider Register 4"]
pub mod clkdiv4;
#[doc = "Miscellaneous Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misctrl1](misctrl1) module"]
pub type MISCTRL1 = crate::Reg<u32, _MISCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISCTRL1;
#[doc = "`read()` method returns [misctrl1::R](misctrl1::R) reader structure"]
impl crate::Readable for MISCTRL1 {}
#[doc = "`write(|w| ..)` method takes [misctrl1::W](misctrl1::W) writer structure"]
impl crate::Writable for MISCTRL1 {}
#[doc = "Miscellaneous Control register 1"]
pub mod misctrl1;
