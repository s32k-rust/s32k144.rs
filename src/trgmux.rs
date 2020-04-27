#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRGMUX DMAMUX0 Register"]
    pub trgmux_dmamux0: TRGMUX_DMAMUX0,
    #[doc = "0x04 - TRGMUX EXTOUT0 Register"]
    pub trgmux_extout0: TRGMUX_EXTOUT0,
    #[doc = "0x08 - TRGMUX EXTOUT1 Register"]
    pub trgmux_extout1: TRGMUX_EXTOUT1,
    #[doc = "0x0c - TRGMUX ADC0 Register"]
    pub trgmux_adc0: TRGMUX_ADC0,
    #[doc = "0x10 - TRGMUX ADC1 Register"]
    pub trgmux_adc1: TRGMUX_ADC1,
    #[doc = "0x14 - TRGMUX Reserved Register 5"]
    pub trgmuxdummy5: TRGMUXDUMMY5,
    #[doc = "0x18 - TRGMUX Reserved Register 6"]
    pub trgmuxdummy6: TRGMUXDUMMY6,
    #[doc = "0x1c - TRGMUX CMP0 Register"]
    pub trgmux_cmp0: TRGMUX_CMP0,
    #[doc = "0x20 - TRGMUX Reserved Register 8"]
    pub trgmuxdummy8: TRGMUXDUMMY8,
    #[doc = "0x24 - TRGMUX Reserved Register 9"]
    pub trgmuxdummy9: TRGMUXDUMMY9,
    #[doc = "0x28 - TRGMUX FTM0 Register"]
    pub trgmux_ftm0: TRGMUX_FTM0,
    #[doc = "0x2c - TRGMUX FTM1 Register"]
    pub trgmux_ftm1: TRGMUX_FTM1,
    #[doc = "0x30 - TRGMUX FTM2 Register"]
    pub trgmux_ftm2: TRGMUX_FTM2,
    #[doc = "0x34 - TRGMUX FTM3 Register"]
    pub trgmux_ftm3: TRGMUX_FTM3,
    #[doc = "0x38 - TRGMUX PDB0 Register"]
    pub trgmux_pdb0: TRGMUX_PDB0,
    #[doc = "0x3c - TRGMUX PDB1 Register"]
    pub trgmux_pdb1: TRGMUX_PDB1,
    #[doc = "0x40 - TRGMUX Reserved Register 16"]
    pub trgmuxdummy16: TRGMUXDUMMY16,
    #[doc = "0x44 - TRGMUX FLEXIO Register"]
    pub trgmux_flexio: TRGMUX_FLEXIO,
    #[doc = "0x48 - TRGMUX LPIT0 Register"]
    pub trgmux_lpit0: TRGMUX_LPIT0,
    #[doc = "0x4c - TRGMUX LPUART0 Register"]
    pub trgmux_lpuart0: TRGMUX_LPUART0,
    #[doc = "0x50 - TRGMUX LPUART1 Register"]
    pub trgmux_lpuart1: TRGMUX_LPUART1,
    #[doc = "0x54 - TRGMUX LPI2C0 Register"]
    pub trgmux_lpi2c0: TRGMUX_LPI2C0,
    #[doc = "0x58 - TRGMUX Reserved Register 22"]
    pub trgmuxdummy22: TRGMUXDUMMY22,
    #[doc = "0x5c - TRGMUX LPSPI0 Register"]
    pub trgmux_lpspi0: TRGMUX_LPSPI0,
    #[doc = "0x60 - TRGMUX LPSPI1 Register"]
    pub trgmux_lpspi1: TRGMUX_LPSPI1,
    #[doc = "0x64 - TRGMUX LPTMR0 Register"]
    pub trgmux_lptmr0: TRGMUX_LPTMR0,
}
#[doc = "TRGMUX DMAMUX0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_dmamux0](trgmux_dmamux0) module"]
pub type TRGMUX_DMAMUX0 = crate::Reg<u32, _TRGMUX_DMAMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_DMAMUX0;
#[doc = "`read()` method returns [trgmux_dmamux0::R](trgmux_dmamux0::R) reader structure"]
impl crate::Readable for TRGMUX_DMAMUX0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_dmamux0::W](trgmux_dmamux0::W) writer structure"]
impl crate::Writable for TRGMUX_DMAMUX0 {}
#[doc = "TRGMUX DMAMUX0 Register"]
pub mod trgmux_dmamux0;
#[doc = "TRGMUX EXTOUT0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_extout0](trgmux_extout0) module"]
pub type TRGMUX_EXTOUT0 = crate::Reg<u32, _TRGMUX_EXTOUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_EXTOUT0;
#[doc = "`read()` method returns [trgmux_extout0::R](trgmux_extout0::R) reader structure"]
impl crate::Readable for TRGMUX_EXTOUT0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_extout0::W](trgmux_extout0::W) writer structure"]
impl crate::Writable for TRGMUX_EXTOUT0 {}
#[doc = "TRGMUX EXTOUT0 Register"]
pub mod trgmux_extout0;
#[doc = "TRGMUX EXTOUT1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_extout1](trgmux_extout1) module"]
pub type TRGMUX_EXTOUT1 = crate::Reg<u32, _TRGMUX_EXTOUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_EXTOUT1;
#[doc = "`read()` method returns [trgmux_extout1::R](trgmux_extout1::R) reader structure"]
impl crate::Readable for TRGMUX_EXTOUT1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_extout1::W](trgmux_extout1::W) writer structure"]
impl crate::Writable for TRGMUX_EXTOUT1 {}
#[doc = "TRGMUX EXTOUT1 Register"]
pub mod trgmux_extout1;
#[doc = "TRGMUX ADC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_adc0](trgmux_adc0) module"]
pub type TRGMUX_ADC0 = crate::Reg<u32, _TRGMUX_ADC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_ADC0;
#[doc = "`read()` method returns [trgmux_adc0::R](trgmux_adc0::R) reader structure"]
impl crate::Readable for TRGMUX_ADC0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_adc0::W](trgmux_adc0::W) writer structure"]
impl crate::Writable for TRGMUX_ADC0 {}
#[doc = "TRGMUX ADC0 Register"]
pub mod trgmux_adc0;
#[doc = "TRGMUX ADC1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_adc1](trgmux_adc1) module"]
pub type TRGMUX_ADC1 = crate::Reg<u32, _TRGMUX_ADC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_ADC1;
#[doc = "`read()` method returns [trgmux_adc1::R](trgmux_adc1::R) reader structure"]
impl crate::Readable for TRGMUX_ADC1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_adc1::W](trgmux_adc1::W) writer structure"]
impl crate::Writable for TRGMUX_ADC1 {}
#[doc = "TRGMUX ADC1 Register"]
pub mod trgmux_adc1;
#[doc = "TRGMUX Reserved Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy5](trgmuxdummy5) module"]
pub type TRGMUXDUMMY5 = crate::Reg<u32, _TRGMUXDUMMY5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY5;
#[doc = "`read()` method returns [trgmuxdummy5::R](trgmuxdummy5::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY5 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy5::W](trgmuxdummy5::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY5 {}
#[doc = "TRGMUX Reserved Register 5"]
pub mod trgmuxdummy5;
#[doc = "TRGMUX Reserved Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy6](trgmuxdummy6) module"]
pub type TRGMUXDUMMY6 = crate::Reg<u32, _TRGMUXDUMMY6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY6;
#[doc = "`read()` method returns [trgmuxdummy6::R](trgmuxdummy6::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY6 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy6::W](trgmuxdummy6::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY6 {}
#[doc = "TRGMUX Reserved Register 6"]
pub mod trgmuxdummy6;
#[doc = "TRGMUX CMP0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_cmp0](trgmux_cmp0) module"]
pub type TRGMUX_CMP0 = crate::Reg<u32, _TRGMUX_CMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_CMP0;
#[doc = "`read()` method returns [trgmux_cmp0::R](trgmux_cmp0::R) reader structure"]
impl crate::Readable for TRGMUX_CMP0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_cmp0::W](trgmux_cmp0::W) writer structure"]
impl crate::Writable for TRGMUX_CMP0 {}
#[doc = "TRGMUX CMP0 Register"]
pub mod trgmux_cmp0;
#[doc = "TRGMUX Reserved Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy8](trgmuxdummy8) module"]
pub type TRGMUXDUMMY8 = crate::Reg<u32, _TRGMUXDUMMY8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY8;
#[doc = "`read()` method returns [trgmuxdummy8::R](trgmuxdummy8::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY8 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy8::W](trgmuxdummy8::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY8 {}
#[doc = "TRGMUX Reserved Register 8"]
pub mod trgmuxdummy8;
#[doc = "TRGMUX Reserved Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy9](trgmuxdummy9) module"]
pub type TRGMUXDUMMY9 = crate::Reg<u32, _TRGMUXDUMMY9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY9;
#[doc = "`read()` method returns [trgmuxdummy9::R](trgmuxdummy9::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY9 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy9::W](trgmuxdummy9::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY9 {}
#[doc = "TRGMUX Reserved Register 9"]
pub mod trgmuxdummy9;
#[doc = "TRGMUX FTM0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_ftm0](trgmux_ftm0) module"]
pub type TRGMUX_FTM0 = crate::Reg<u32, _TRGMUX_FTM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_FTM0;
#[doc = "`read()` method returns [trgmux_ftm0::R](trgmux_ftm0::R) reader structure"]
impl crate::Readable for TRGMUX_FTM0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_ftm0::W](trgmux_ftm0::W) writer structure"]
impl crate::Writable for TRGMUX_FTM0 {}
#[doc = "TRGMUX FTM0 Register"]
pub mod trgmux_ftm0;
#[doc = "TRGMUX FTM1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_ftm1](trgmux_ftm1) module"]
pub type TRGMUX_FTM1 = crate::Reg<u32, _TRGMUX_FTM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_FTM1;
#[doc = "`read()` method returns [trgmux_ftm1::R](trgmux_ftm1::R) reader structure"]
impl crate::Readable for TRGMUX_FTM1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_ftm1::W](trgmux_ftm1::W) writer structure"]
impl crate::Writable for TRGMUX_FTM1 {}
#[doc = "TRGMUX FTM1 Register"]
pub mod trgmux_ftm1;
#[doc = "TRGMUX FTM2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_ftm2](trgmux_ftm2) module"]
pub type TRGMUX_FTM2 = crate::Reg<u32, _TRGMUX_FTM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_FTM2;
#[doc = "`read()` method returns [trgmux_ftm2::R](trgmux_ftm2::R) reader structure"]
impl crate::Readable for TRGMUX_FTM2 {}
#[doc = "`write(|w| ..)` method takes [trgmux_ftm2::W](trgmux_ftm2::W) writer structure"]
impl crate::Writable for TRGMUX_FTM2 {}
#[doc = "TRGMUX FTM2 Register"]
pub mod trgmux_ftm2;
#[doc = "TRGMUX FTM3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_ftm3](trgmux_ftm3) module"]
pub type TRGMUX_FTM3 = crate::Reg<u32, _TRGMUX_FTM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_FTM3;
#[doc = "`read()` method returns [trgmux_ftm3::R](trgmux_ftm3::R) reader structure"]
impl crate::Readable for TRGMUX_FTM3 {}
#[doc = "`write(|w| ..)` method takes [trgmux_ftm3::W](trgmux_ftm3::W) writer structure"]
impl crate::Writable for TRGMUX_FTM3 {}
#[doc = "TRGMUX FTM3 Register"]
pub mod trgmux_ftm3;
#[doc = "TRGMUX PDB0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_pdb0](trgmux_pdb0) module"]
pub type TRGMUX_PDB0 = crate::Reg<u32, _TRGMUX_PDB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_PDB0;
#[doc = "`read()` method returns [trgmux_pdb0::R](trgmux_pdb0::R) reader structure"]
impl crate::Readable for TRGMUX_PDB0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_pdb0::W](trgmux_pdb0::W) writer structure"]
impl crate::Writable for TRGMUX_PDB0 {}
#[doc = "TRGMUX PDB0 Register"]
pub mod trgmux_pdb0;
#[doc = "TRGMUX PDB1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_pdb1](trgmux_pdb1) module"]
pub type TRGMUX_PDB1 = crate::Reg<u32, _TRGMUX_PDB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_PDB1;
#[doc = "`read()` method returns [trgmux_pdb1::R](trgmux_pdb1::R) reader structure"]
impl crate::Readable for TRGMUX_PDB1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_pdb1::W](trgmux_pdb1::W) writer structure"]
impl crate::Writable for TRGMUX_PDB1 {}
#[doc = "TRGMUX PDB1 Register"]
pub mod trgmux_pdb1;
#[doc = "TRGMUX Reserved Register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy16](trgmuxdummy16) module"]
pub type TRGMUXDUMMY16 = crate::Reg<u32, _TRGMUXDUMMY16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY16;
#[doc = "`read()` method returns [trgmuxdummy16::R](trgmuxdummy16::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY16 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy16::W](trgmuxdummy16::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY16 {}
#[doc = "TRGMUX Reserved Register 16"]
pub mod trgmuxdummy16;
#[doc = "TRGMUX FLEXIO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_flexio](trgmux_flexio) module"]
pub type TRGMUX_FLEXIO = crate::Reg<u32, _TRGMUX_FLEXIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_FLEXIO;
#[doc = "`read()` method returns [trgmux_flexio::R](trgmux_flexio::R) reader structure"]
impl crate::Readable for TRGMUX_FLEXIO {}
#[doc = "`write(|w| ..)` method takes [trgmux_flexio::W](trgmux_flexio::W) writer structure"]
impl crate::Writable for TRGMUX_FLEXIO {}
#[doc = "TRGMUX FLEXIO Register"]
pub mod trgmux_flexio;
#[doc = "TRGMUX LPIT0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpit0](trgmux_lpit0) module"]
pub type TRGMUX_LPIT0 = crate::Reg<u32, _TRGMUX_LPIT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPIT0;
#[doc = "`read()` method returns [trgmux_lpit0::R](trgmux_lpit0::R) reader structure"]
impl crate::Readable for TRGMUX_LPIT0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpit0::W](trgmux_lpit0::W) writer structure"]
impl crate::Writable for TRGMUX_LPIT0 {}
#[doc = "TRGMUX LPIT0 Register"]
pub mod trgmux_lpit0;
#[doc = "TRGMUX LPUART0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpuart0](trgmux_lpuart0) module"]
pub type TRGMUX_LPUART0 = crate::Reg<u32, _TRGMUX_LPUART0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPUART0;
#[doc = "`read()` method returns [trgmux_lpuart0::R](trgmux_lpuart0::R) reader structure"]
impl crate::Readable for TRGMUX_LPUART0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpuart0::W](trgmux_lpuart0::W) writer structure"]
impl crate::Writable for TRGMUX_LPUART0 {}
#[doc = "TRGMUX LPUART0 Register"]
pub mod trgmux_lpuart0;
#[doc = "TRGMUX LPUART1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpuart1](trgmux_lpuart1) module"]
pub type TRGMUX_LPUART1 = crate::Reg<u32, _TRGMUX_LPUART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPUART1;
#[doc = "`read()` method returns [trgmux_lpuart1::R](trgmux_lpuart1::R) reader structure"]
impl crate::Readable for TRGMUX_LPUART1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpuart1::W](trgmux_lpuart1::W) writer structure"]
impl crate::Writable for TRGMUX_LPUART1 {}
#[doc = "TRGMUX LPUART1 Register"]
pub mod trgmux_lpuart1;
#[doc = "TRGMUX LPI2C0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpi2c0](trgmux_lpi2c0) module"]
pub type TRGMUX_LPI2C0 = crate::Reg<u32, _TRGMUX_LPI2C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPI2C0;
#[doc = "`read()` method returns [trgmux_lpi2c0::R](trgmux_lpi2c0::R) reader structure"]
impl crate::Readable for TRGMUX_LPI2C0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpi2c0::W](trgmux_lpi2c0::W) writer structure"]
impl crate::Writable for TRGMUX_LPI2C0 {}
#[doc = "TRGMUX LPI2C0 Register"]
pub mod trgmux_lpi2c0;
#[doc = "TRGMUX Reserved Register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmuxdummy22](trgmuxdummy22) module"]
pub type TRGMUXDUMMY22 = crate::Reg<u32, _TRGMUXDUMMY22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUXDUMMY22;
#[doc = "`read()` method returns [trgmuxdummy22::R](trgmuxdummy22::R) reader structure"]
impl crate::Readable for TRGMUXDUMMY22 {}
#[doc = "`write(|w| ..)` method takes [trgmuxdummy22::W](trgmuxdummy22::W) writer structure"]
impl crate::Writable for TRGMUXDUMMY22 {}
#[doc = "TRGMUX Reserved Register 22"]
pub mod trgmuxdummy22;
#[doc = "TRGMUX LPSPI0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpspi0](trgmux_lpspi0) module"]
pub type TRGMUX_LPSPI0 = crate::Reg<u32, _TRGMUX_LPSPI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPSPI0;
#[doc = "`read()` method returns [trgmux_lpspi0::R](trgmux_lpspi0::R) reader structure"]
impl crate::Readable for TRGMUX_LPSPI0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpspi0::W](trgmux_lpspi0::W) writer structure"]
impl crate::Writable for TRGMUX_LPSPI0 {}
#[doc = "TRGMUX LPSPI0 Register"]
pub mod trgmux_lpspi0;
#[doc = "TRGMUX LPSPI1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lpspi1](trgmux_lpspi1) module"]
pub type TRGMUX_LPSPI1 = crate::Reg<u32, _TRGMUX_LPSPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPSPI1;
#[doc = "`read()` method returns [trgmux_lpspi1::R](trgmux_lpspi1::R) reader structure"]
impl crate::Readable for TRGMUX_LPSPI1 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lpspi1::W](trgmux_lpspi1::W) writer structure"]
impl crate::Writable for TRGMUX_LPSPI1 {}
#[doc = "TRGMUX LPSPI1 Register"]
pub mod trgmux_lpspi1;
#[doc = "TRGMUX LPTMR0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgmux_lptmr0](trgmux_lptmr0) module"]
pub type TRGMUX_LPTMR0 = crate::Reg<u32, _TRGMUX_LPTMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRGMUX_LPTMR0;
#[doc = "`read()` method returns [trgmux_lptmr0::R](trgmux_lptmr0::R) reader structure"]
impl crate::Readable for TRGMUX_LPTMR0 {}
#[doc = "`write(|w| ..)` method takes [trgmux_lptmr0::W](trgmux_lptmr0::W) writer structure"]
impl crate::Writable for TRGMUX_LPTMR0 {}
#[doc = "TRGMUX LPTMR0 Register"]
pub mod trgmux_lptmr0;
