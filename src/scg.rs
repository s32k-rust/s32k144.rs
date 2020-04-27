#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Clock Status Register"]
    pub csr: CSR,
    #[doc = "0x14 - Run Clock Control Register"]
    pub rccr: RCCR,
    #[doc = "0x18 - VLPR Clock Control Register"]
    pub vccr: VCCR,
    #[doc = "0x1c - HSRUN Clock Control Register"]
    pub hccr: HCCR,
    #[doc = "0x20 - SCG CLKOUT Configuration Register"]
    pub clkoutcnfg: CLKOUTCNFG,
    _reserved7: [u8; 220usize],
    #[doc = "0x100 - System OSC Control Status Register"]
    pub sosccsr: SOSCCSR,
    #[doc = "0x104 - System OSC Divide Register"]
    pub soscdiv: SOSCDIV,
    #[doc = "0x108 - System Oscillator Configuration Register"]
    pub sosccfg: SOSCCFG,
    _reserved10: [u8; 244usize],
    #[doc = "0x200 - Slow IRC Control Status Register"]
    pub sirccsr: SIRCCSR,
    #[doc = "0x204 - Slow IRC Divide Register"]
    pub sircdiv: SIRCDIV,
    #[doc = "0x208 - Slow IRC Configuration Register"]
    pub sirccfg: SIRCCFG,
    _reserved13: [u8; 244usize],
    #[doc = "0x300 - Fast IRC Control Status Register"]
    pub firccsr: FIRCCSR,
    #[doc = "0x304 - Fast IRC Divide Register"]
    pub fircdiv: FIRCDIV,
    #[doc = "0x308 - Fast IRC Configuration Register"]
    pub firccfg: FIRCCFG,
    _reserved16: [u8; 756usize],
    #[doc = "0x600 - System PLL Control Status Register"]
    pub spllcsr: SPLLCSR,
    #[doc = "0x604 - System PLL Divide Register"]
    pub splldiv: SPLLDIV,
    #[doc = "0x608 - System PLL Configuration Register"]
    pub spllcfg: SPLLCFG,
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
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "Clock Status Register"]
pub mod csr;
#[doc = "Run Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccr](rccr) module"]
pub type RCCR = crate::Reg<u32, _RCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCCR;
#[doc = "`read()` method returns [rccr::R](rccr::R) reader structure"]
impl crate::Readable for RCCR {}
#[doc = "`write(|w| ..)` method takes [rccr::W](rccr::W) writer structure"]
impl crate::Writable for RCCR {}
#[doc = "Run Clock Control Register"]
pub mod rccr;
#[doc = "VLPR Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vccr](vccr) module"]
pub type VCCR = crate::Reg<u32, _VCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VCCR;
#[doc = "`read()` method returns [vccr::R](vccr::R) reader structure"]
impl crate::Readable for VCCR {}
#[doc = "`write(|w| ..)` method takes [vccr::W](vccr::W) writer structure"]
impl crate::Writable for VCCR {}
#[doc = "VLPR Clock Control Register"]
pub mod vccr;
#[doc = "HSRUN Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccr](hccr) module"]
pub type HCCR = crate::Reg<u32, _HCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCR;
#[doc = "`read()` method returns [hccr::R](hccr::R) reader structure"]
impl crate::Readable for HCCR {}
#[doc = "`write(|w| ..)` method takes [hccr::W](hccr::W) writer structure"]
impl crate::Writable for HCCR {}
#[doc = "HSRUN Clock Control Register"]
pub mod hccr;
#[doc = "SCG CLKOUT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutcnfg](clkoutcnfg) module"]
pub type CLKOUTCNFG = crate::Reg<u32, _CLKOUTCNFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCNFG;
#[doc = "`read()` method returns [clkoutcnfg::R](clkoutcnfg::R) reader structure"]
impl crate::Readable for CLKOUTCNFG {}
#[doc = "`write(|w| ..)` method takes [clkoutcnfg::W](clkoutcnfg::W) writer structure"]
impl crate::Writable for CLKOUTCNFG {}
#[doc = "SCG CLKOUT Configuration Register"]
pub mod clkoutcnfg;
#[doc = "System OSC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sosccsr](sosccsr) module"]
pub type SOSCCSR = crate::Reg<u32, _SOSCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOSCCSR;
#[doc = "`read()` method returns [sosccsr::R](sosccsr::R) reader structure"]
impl crate::Readable for SOSCCSR {}
#[doc = "`write(|w| ..)` method takes [sosccsr::W](sosccsr::W) writer structure"]
impl crate::Writable for SOSCCSR {}
#[doc = "System OSC Control Status Register"]
pub mod sosccsr;
#[doc = "System OSC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soscdiv](soscdiv) module"]
pub type SOSCDIV = crate::Reg<u32, _SOSCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOSCDIV;
#[doc = "`read()` method returns [soscdiv::R](soscdiv::R) reader structure"]
impl crate::Readable for SOSCDIV {}
#[doc = "`write(|w| ..)` method takes [soscdiv::W](soscdiv::W) writer structure"]
impl crate::Writable for SOSCDIV {}
#[doc = "System OSC Divide Register"]
pub mod soscdiv;
#[doc = "System Oscillator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sosccfg](sosccfg) module"]
pub type SOSCCFG = crate::Reg<u32, _SOSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOSCCFG;
#[doc = "`read()` method returns [sosccfg::R](sosccfg::R) reader structure"]
impl crate::Readable for SOSCCFG {}
#[doc = "`write(|w| ..)` method takes [sosccfg::W](sosccfg::W) writer structure"]
impl crate::Writable for SOSCCFG {}
#[doc = "System Oscillator Configuration Register"]
pub mod sosccfg;
#[doc = "Slow IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sirccsr](sirccsr) module"]
pub type SIRCCSR = crate::Reg<u32, _SIRCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCCSR;
#[doc = "`read()` method returns [sirccsr::R](sirccsr::R) reader structure"]
impl crate::Readable for SIRCCSR {}
#[doc = "`write(|w| ..)` method takes [sirccsr::W](sirccsr::W) writer structure"]
impl crate::Writable for SIRCCSR {}
#[doc = "Slow IRC Control Status Register"]
pub mod sirccsr;
#[doc = "Slow IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sircdiv](sircdiv) module"]
pub type SIRCDIV = crate::Reg<u32, _SIRCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCDIV;
#[doc = "`read()` method returns [sircdiv::R](sircdiv::R) reader structure"]
impl crate::Readable for SIRCDIV {}
#[doc = "`write(|w| ..)` method takes [sircdiv::W](sircdiv::W) writer structure"]
impl crate::Writable for SIRCDIV {}
#[doc = "Slow IRC Divide Register"]
pub mod sircdiv;
#[doc = "Slow IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sirccfg](sirccfg) module"]
pub type SIRCCFG = crate::Reg<u32, _SIRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIRCCFG;
#[doc = "`read()` method returns [sirccfg::R](sirccfg::R) reader structure"]
impl crate::Readable for SIRCCFG {}
#[doc = "`write(|w| ..)` method takes [sirccfg::W](sirccfg::W) writer structure"]
impl crate::Writable for SIRCCFG {}
#[doc = "Slow IRC Configuration Register"]
pub mod sirccfg;
#[doc = "Fast IRC Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firccsr](firccsr) module"]
pub type FIRCCSR = crate::Reg<u32, _FIRCCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCCSR;
#[doc = "`read()` method returns [firccsr::R](firccsr::R) reader structure"]
impl crate::Readable for FIRCCSR {}
#[doc = "`write(|w| ..)` method takes [firccsr::W](firccsr::W) writer structure"]
impl crate::Writable for FIRCCSR {}
#[doc = "Fast IRC Control Status Register"]
pub mod firccsr;
#[doc = "Fast IRC Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fircdiv](fircdiv) module"]
pub type FIRCDIV = crate::Reg<u32, _FIRCDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCDIV;
#[doc = "`read()` method returns [fircdiv::R](fircdiv::R) reader structure"]
impl crate::Readable for FIRCDIV {}
#[doc = "`write(|w| ..)` method takes [fircdiv::W](fircdiv::W) writer structure"]
impl crate::Writable for FIRCDIV {}
#[doc = "Fast IRC Divide Register"]
pub mod fircdiv;
#[doc = "Fast IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [firccfg](firccfg) module"]
pub type FIRCCFG = crate::Reg<u32, _FIRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIRCCFG;
#[doc = "`read()` method returns [firccfg::R](firccfg::R) reader structure"]
impl crate::Readable for FIRCCFG {}
#[doc = "`write(|w| ..)` method takes [firccfg::W](firccfg::W) writer structure"]
impl crate::Writable for FIRCCFG {}
#[doc = "Fast IRC Configuration Register"]
pub mod firccfg;
#[doc = "System PLL Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllcsr](spllcsr) module"]
pub type SPLLCSR = crate::Reg<u32, _SPLLCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCSR;
#[doc = "`read()` method returns [spllcsr::R](spllcsr::R) reader structure"]
impl crate::Readable for SPLLCSR {}
#[doc = "`write(|w| ..)` method takes [spllcsr::W](spllcsr::W) writer structure"]
impl crate::Writable for SPLLCSR {}
#[doc = "System PLL Control Status Register"]
pub mod spllcsr;
#[doc = "System PLL Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [splldiv](splldiv) module"]
pub type SPLLDIV = crate::Reg<u32, _SPLLDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLDIV;
#[doc = "`read()` method returns [splldiv::R](splldiv::R) reader structure"]
impl crate::Readable for SPLLDIV {}
#[doc = "`write(|w| ..)` method takes [splldiv::W](splldiv::W) writer structure"]
impl crate::Writable for SPLLDIV {}
#[doc = "System PLL Divide Register"]
pub mod splldiv;
#[doc = "System PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllcfg](spllcfg) module"]
pub type SPLLCFG = crate::Reg<u32, _SPLLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCFG;
#[doc = "`read()` method returns [spllcfg::R](spllcfg::R) reader structure"]
impl crate::Readable for SPLLCFG {}
#[doc = "`write(|w| ..)` method takes [spllcfg::W](spllcfg::W) writer structure"]
impl crate::Writable for SPLLCFG {}
#[doc = "System PLL Configuration Register"]
pub mod spllcfg;
