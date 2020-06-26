#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - PCC FTFC Register"]
    pub pcc_ftfc: PCC_FTFC,
    #[doc = "0x84 - PCC DMAMUX Register"]
    pub pcc_dmamux: PCC_DMAMUX,
    _reserved2: [u8; 8usize],
    #[doc = "0x90 - PCC FlexCAN0 Register"]
    pub pcc_flex_can0: PCC_FLEXCAN0,
    #[doc = "0x94 - PCC FlexCAN1 Register"]
    pub pcc_flex_can1: PCC_FLEXCAN1,
    #[doc = "0x98 - PCC FTM3 Register"]
    pub pcc_ftm3: PCC_FTM3,
    #[doc = "0x9c - PCC ADC1 Register"]
    pub pcc_adc1: PCC_ADC1,
    _reserved6: [u8; 12usize],
    #[doc = "0xac - PCC FlexCAN2 Register"]
    pub pcc_flex_can2: PCC_FLEXCAN2,
    #[doc = "0xb0 - PCC LPSPI0 Register"]
    pub pcc_lpspi0: PCC_LPSPI0,
    #[doc = "0xb4 - PCC LPSPI1 Register"]
    pub pcc_lpspi1: PCC_LPSPI1,
    #[doc = "0xb8 - PCC LPSPI2 Register"]
    pub pcc_lpspi2: PCC_LPSPI2,
    _reserved10: [u8; 8usize],
    #[doc = "0xc4 - PCC PDB1 Register"]
    pub pcc_pdb1: PCC_PDB1,
    #[doc = "0xc8 - PCC CRC Register"]
    pub pcc_crc: PCC_CRC,
    _reserved12: [u8; 12usize],
    #[doc = "0xd8 - PCC PDB0 Register"]
    pub pcc_pdb0: PCC_PDB0,
    #[doc = "0xdc - PCC LPIT Register"]
    pub pcc_lpit: PCC_LPIT,
    #[doc = "0xe0 - PCC FTM0 Register"]
    pub pcc_ftm0: PCC_FTM0,
    #[doc = "0xe4 - PCC FTM1 Register"]
    pub pcc_ftm1: PCC_FTM1,
    #[doc = "0xe8 - PCC FTM2 Register"]
    pub pcc_ftm2: PCC_FTM2,
    #[doc = "0xec - PCC ADC0 Register"]
    pub pcc_adc0: PCC_ADC0,
    _reserved18: [u8; 4usize],
    #[doc = "0xf4 - PCC RTC Register"]
    pub pcc_rtc: PCC_RTC,
    _reserved19: [u8; 8usize],
    #[doc = "0x100 - PCC LPTMR0 Register"]
    pub pcc_lptmr0: PCC_LPTMR0,
    _reserved20: [u8; 32usize],
    #[doc = "0x124 - PCC PORTA Register"]
    pub pcc_porta: PCC_PORTA,
    #[doc = "0x128 - PCC PORTB Register"]
    pub pcc_portb: PCC_PORTB,
    #[doc = "0x12c - PCC PORTC Register"]
    pub pcc_portc: PCC_PORTC,
    #[doc = "0x130 - PCC PORTD Register"]
    pub pcc_portd: PCC_PORTD,
    #[doc = "0x134 - PCC PORTE Register"]
    pub pcc_porte: PCC_PORTE,
    _reserved25: [u8; 48usize],
    #[doc = "0x168 - PCC FlexIO Register"]
    pub pcc_flexio: PCC_FLEXIO,
    _reserved26: [u8; 24usize],
    #[doc = "0x184 - PCC EWM Register"]
    pub pcc_ewm: PCC_EWM,
    _reserved27: [u8; 16usize],
    #[doc = "0x198 - PCC LPI2C0 Register"]
    pub pcc_lpi2c0: PCC_LPI2C0,
    _reserved28: [u8; 12usize],
    #[doc = "0x1a8 - PCC LPUART0 Register"]
    pub pcc_lpuart0: PCC_LPUART0,
    #[doc = "0x1ac - PCC LPUART1 Register"]
    pub pcc_lpuart1: PCC_LPUART1,
    #[doc = "0x1b0 - PCC LPUART2 Register"]
    pub pcc_lpuart2: PCC_LPUART2,
    _reserved31: [u8; 24usize],
    #[doc = "0x1cc - PCC CMP0 Register"]
    pub pcc_cmp0: PCC_CMP0,
}
#[doc = "PCC FTFC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ftfc](pcc_ftfc) module"]
pub type PCC_FTFC = crate::Reg<u32, _PCC_FTFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FTFC;
#[doc = "`read()` method returns [pcc_ftfc::R](pcc_ftfc::R) reader structure"]
impl crate::Readable for PCC_FTFC {}
#[doc = "`write(|w| ..)` method takes [pcc_ftfc::W](pcc_ftfc::W) writer structure"]
impl crate::Writable for PCC_FTFC {}
#[doc = "PCC FTFC Register"]
pub mod pcc_ftfc;
#[doc = "PCC DMAMUX Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_dmamux](pcc_dmamux) module"]
pub type PCC_DMAMUX = crate::Reg<u32, _PCC_DMAMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_DMAMUX;
#[doc = "`read()` method returns [pcc_dmamux::R](pcc_dmamux::R) reader structure"]
impl crate::Readable for PCC_DMAMUX {}
#[doc = "`write(|w| ..)` method takes [pcc_dmamux::W](pcc_dmamux::W) writer structure"]
impl crate::Writable for PCC_DMAMUX {}
#[doc = "PCC DMAMUX Register"]
pub mod pcc_dmamux;
#[doc = "PCC FlexCAN0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_flex_can0](pcc_flex_can0) module"]
pub type PCC_FLEXCAN0 = crate::Reg<u32, _PCC_FLEXCAN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXCAN0;
#[doc = "`read()` method returns [pcc_flex_can0::R](pcc_flex_can0::R) reader structure"]
impl crate::Readable for PCC_FLEXCAN0 {}
#[doc = "`write(|w| ..)` method takes [pcc_flex_can0::W](pcc_flex_can0::W) writer structure"]
impl crate::Writable for PCC_FLEXCAN0 {}
#[doc = "PCC FlexCAN0 Register"]
pub mod pcc_flex_can0;
#[doc = "PCC FlexCAN1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_flex_can1](pcc_flex_can1) module"]
pub type PCC_FLEXCAN1 = crate::Reg<u32, _PCC_FLEXCAN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXCAN1;
#[doc = "`read()` method returns [pcc_flex_can1::R](pcc_flex_can1::R) reader structure"]
impl crate::Readable for PCC_FLEXCAN1 {}
#[doc = "`write(|w| ..)` method takes [pcc_flex_can1::W](pcc_flex_can1::W) writer structure"]
impl crate::Writable for PCC_FLEXCAN1 {}
#[doc = "PCC FlexCAN1 Register"]
pub mod pcc_flex_can1;
#[doc = "PCC FTM3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ftm3](pcc_ftm3) module"]
pub type PCC_FTM3 = crate::Reg<u32, _PCC_FTM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FTM3;
#[doc = "`read()` method returns [pcc_ftm3::R](pcc_ftm3::R) reader structure"]
impl crate::Readable for PCC_FTM3 {}
#[doc = "`write(|w| ..)` method takes [pcc_ftm3::W](pcc_ftm3::W) writer structure"]
impl crate::Writable for PCC_FTM3 {}
#[doc = "PCC FTM3 Register"]
pub mod pcc_ftm3;
#[doc = "PCC ADC1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_adc1](pcc_adc1) module"]
pub type PCC_ADC1 = crate::Reg<u32, _PCC_ADC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_ADC1;
#[doc = "`read()` method returns [pcc_adc1::R](pcc_adc1::R) reader structure"]
impl crate::Readable for PCC_ADC1 {}
#[doc = "`write(|w| ..)` method takes [pcc_adc1::W](pcc_adc1::W) writer structure"]
impl crate::Writable for PCC_ADC1 {}
#[doc = "PCC ADC1 Register"]
pub mod pcc_adc1;
#[doc = "PCC FlexCAN2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_flex_can2](pcc_flex_can2) module"]
pub type PCC_FLEXCAN2 = crate::Reg<u32, _PCC_FLEXCAN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXCAN2;
#[doc = "`read()` method returns [pcc_flex_can2::R](pcc_flex_can2::R) reader structure"]
impl crate::Readable for PCC_FLEXCAN2 {}
#[doc = "`write(|w| ..)` method takes [pcc_flex_can2::W](pcc_flex_can2::W) writer structure"]
impl crate::Writable for PCC_FLEXCAN2 {}
#[doc = "PCC FlexCAN2 Register"]
pub mod pcc_flex_can2;
#[doc = "PCC LPSPI0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpspi0](pcc_lpspi0) module"]
pub type PCC_LPSPI0 = crate::Reg<u32, _PCC_LPSPI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI0;
#[doc = "`read()` method returns [pcc_lpspi0::R](pcc_lpspi0::R) reader structure"]
impl crate::Readable for PCC_LPSPI0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi0::W](pcc_lpspi0::W) writer structure"]
impl crate::Writable for PCC_LPSPI0 {}
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC LPSPI1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpspi1](pcc_lpspi1) module"]
pub type PCC_LPSPI1 = crate::Reg<u32, _PCC_LPSPI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI1;
#[doc = "`read()` method returns [pcc_lpspi1::R](pcc_lpspi1::R) reader structure"]
impl crate::Readable for PCC_LPSPI1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi1::W](pcc_lpspi1::W) writer structure"]
impl crate::Writable for PCC_LPSPI1 {}
#[doc = "PCC LPSPI1 Register"]
pub mod pcc_lpspi1;
#[doc = "PCC LPSPI2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpspi2](pcc_lpspi2) module"]
pub type PCC_LPSPI2 = crate::Reg<u32, _PCC_LPSPI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPSPI2;
#[doc = "`read()` method returns [pcc_lpspi2::R](pcc_lpspi2::R) reader structure"]
impl crate::Readable for PCC_LPSPI2 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpspi2::W](pcc_lpspi2::W) writer structure"]
impl crate::Writable for PCC_LPSPI2 {}
#[doc = "PCC LPSPI2 Register"]
pub mod pcc_lpspi2;
#[doc = "PCC PDB1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_pdb1](pcc_pdb1) module"]
pub type PCC_PDB1 = crate::Reg<u32, _PCC_PDB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PDB1;
#[doc = "`read()` method returns [pcc_pdb1::R](pcc_pdb1::R) reader structure"]
impl crate::Readable for PCC_PDB1 {}
#[doc = "`write(|w| ..)` method takes [pcc_pdb1::W](pcc_pdb1::W) writer structure"]
impl crate::Writable for PCC_PDB1 {}
#[doc = "PCC PDB1 Register"]
pub mod pcc_pdb1;
#[doc = "PCC CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_crc](pcc_crc) module"]
pub type PCC_CRC = crate::Reg<u32, _PCC_CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_CRC;
#[doc = "`read()` method returns [pcc_crc::R](pcc_crc::R) reader structure"]
impl crate::Readable for PCC_CRC {}
#[doc = "`write(|w| ..)` method takes [pcc_crc::W](pcc_crc::W) writer structure"]
impl crate::Writable for PCC_CRC {}
#[doc = "PCC CRC Register"]
pub mod pcc_crc;
#[doc = "PCC PDB0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_pdb0](pcc_pdb0) module"]
pub type PCC_PDB0 = crate::Reg<u32, _PCC_PDB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PDB0;
#[doc = "`read()` method returns [pcc_pdb0::R](pcc_pdb0::R) reader structure"]
impl crate::Readable for PCC_PDB0 {}
#[doc = "`write(|w| ..)` method takes [pcc_pdb0::W](pcc_pdb0::W) writer structure"]
impl crate::Writable for PCC_PDB0 {}
#[doc = "PCC PDB0 Register"]
pub mod pcc_pdb0;
#[doc = "PCC LPIT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpit](pcc_lpit) module"]
pub type PCC_LPIT = crate::Reg<u32, _PCC_LPIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPIT;
#[doc = "`read()` method returns [pcc_lpit::R](pcc_lpit::R) reader structure"]
impl crate::Readable for PCC_LPIT {}
#[doc = "`write(|w| ..)` method takes [pcc_lpit::W](pcc_lpit::W) writer structure"]
impl crate::Writable for PCC_LPIT {}
#[doc = "PCC LPIT Register"]
pub mod pcc_lpit;
#[doc = "PCC FTM0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ftm0](pcc_ftm0) module"]
pub type PCC_FTM0 = crate::Reg<u32, _PCC_FTM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FTM0;
#[doc = "`read()` method returns [pcc_ftm0::R](pcc_ftm0::R) reader structure"]
impl crate::Readable for PCC_FTM0 {}
#[doc = "`write(|w| ..)` method takes [pcc_ftm0::W](pcc_ftm0::W) writer structure"]
impl crate::Writable for PCC_FTM0 {}
#[doc = "PCC FTM0 Register"]
pub mod pcc_ftm0;
#[doc = "PCC FTM1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ftm1](pcc_ftm1) module"]
pub type PCC_FTM1 = crate::Reg<u32, _PCC_FTM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FTM1;
#[doc = "`read()` method returns [pcc_ftm1::R](pcc_ftm1::R) reader structure"]
impl crate::Readable for PCC_FTM1 {}
#[doc = "`write(|w| ..)` method takes [pcc_ftm1::W](pcc_ftm1::W) writer structure"]
impl crate::Writable for PCC_FTM1 {}
#[doc = "PCC FTM1 Register"]
pub mod pcc_ftm1;
#[doc = "PCC FTM2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ftm2](pcc_ftm2) module"]
pub type PCC_FTM2 = crate::Reg<u32, _PCC_FTM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FTM2;
#[doc = "`read()` method returns [pcc_ftm2::R](pcc_ftm2::R) reader structure"]
impl crate::Readable for PCC_FTM2 {}
#[doc = "`write(|w| ..)` method takes [pcc_ftm2::W](pcc_ftm2::W) writer structure"]
impl crate::Writable for PCC_FTM2 {}
#[doc = "PCC FTM2 Register"]
pub mod pcc_ftm2;
#[doc = "PCC ADC0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_adc0](pcc_adc0) module"]
pub type PCC_ADC0 = crate::Reg<u32, _PCC_ADC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_ADC0;
#[doc = "`read()` method returns [pcc_adc0::R](pcc_adc0::R) reader structure"]
impl crate::Readable for PCC_ADC0 {}
#[doc = "`write(|w| ..)` method takes [pcc_adc0::W](pcc_adc0::W) writer structure"]
impl crate::Writable for PCC_ADC0 {}
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC RTC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_rtc](pcc_rtc) module"]
pub type PCC_RTC = crate::Reg<u32, _PCC_RTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_RTC;
#[doc = "`read()` method returns [pcc_rtc::R](pcc_rtc::R) reader structure"]
impl crate::Readable for PCC_RTC {}
#[doc = "`write(|w| ..)` method takes [pcc_rtc::W](pcc_rtc::W) writer structure"]
impl crate::Writable for PCC_RTC {}
#[doc = "PCC RTC Register"]
pub mod pcc_rtc;
#[doc = "PCC LPTMR0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lptmr0](pcc_lptmr0) module"]
pub type PCC_LPTMR0 = crate::Reg<u32, _PCC_LPTMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPTMR0;
#[doc = "`read()` method returns [pcc_lptmr0::R](pcc_lptmr0::R) reader structure"]
impl crate::Readable for PCC_LPTMR0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lptmr0::W](pcc_lptmr0::W) writer structure"]
impl crate::Writable for PCC_LPTMR0 {}
#[doc = "PCC LPTMR0 Register"]
pub mod pcc_lptmr0;
#[doc = "PCC PORTA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_porta](pcc_porta) module"]
pub type PCC_PORTA = crate::Reg<u32, _PCC_PORTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTA;
#[doc = "`read()` method returns [pcc_porta::R](pcc_porta::R) reader structure"]
impl crate::Readable for PCC_PORTA {}
#[doc = "`write(|w| ..)` method takes [pcc_porta::W](pcc_porta::W) writer structure"]
impl crate::Writable for PCC_PORTA {}
#[doc = "PCC PORTA Register"]
pub mod pcc_porta;
#[doc = "PCC PORTB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_portb](pcc_portb) module"]
pub type PCC_PORTB = crate::Reg<u32, _PCC_PORTB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTB;
#[doc = "`read()` method returns [pcc_portb::R](pcc_portb::R) reader structure"]
impl crate::Readable for PCC_PORTB {}
#[doc = "`write(|w| ..)` method takes [pcc_portb::W](pcc_portb::W) writer structure"]
impl crate::Writable for PCC_PORTB {}
#[doc = "PCC PORTB Register"]
pub mod pcc_portb;
#[doc = "PCC PORTC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_portc](pcc_portc) module"]
pub type PCC_PORTC = crate::Reg<u32, _PCC_PORTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTC;
#[doc = "`read()` method returns [pcc_portc::R](pcc_portc::R) reader structure"]
impl crate::Readable for PCC_PORTC {}
#[doc = "`write(|w| ..)` method takes [pcc_portc::W](pcc_portc::W) writer structure"]
impl crate::Writable for PCC_PORTC {}
#[doc = "PCC PORTC Register"]
pub mod pcc_portc;
#[doc = "PCC PORTD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_portd](pcc_portd) module"]
pub type PCC_PORTD = crate::Reg<u32, _PCC_PORTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTD;
#[doc = "`read()` method returns [pcc_portd::R](pcc_portd::R) reader structure"]
impl crate::Readable for PCC_PORTD {}
#[doc = "`write(|w| ..)` method takes [pcc_portd::W](pcc_portd::W) writer structure"]
impl crate::Writable for PCC_PORTD {}
#[doc = "PCC PORTD Register"]
pub mod pcc_portd;
#[doc = "PCC PORTE Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_porte](pcc_porte) module"]
pub type PCC_PORTE = crate::Reg<u32, _PCC_PORTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_PORTE;
#[doc = "`read()` method returns [pcc_porte::R](pcc_porte::R) reader structure"]
impl crate::Readable for PCC_PORTE {}
#[doc = "`write(|w| ..)` method takes [pcc_porte::W](pcc_porte::W) writer structure"]
impl crate::Writable for PCC_PORTE {}
#[doc = "PCC PORTE Register"]
pub mod pcc_porte;
#[doc = "PCC FlexIO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_flexio](pcc_flexio) module"]
pub type PCC_FLEXIO = crate::Reg<u32, _PCC_FLEXIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_FLEXIO;
#[doc = "`read()` method returns [pcc_flexio::R](pcc_flexio::R) reader structure"]
impl crate::Readable for PCC_FLEXIO {}
#[doc = "`write(|w| ..)` method takes [pcc_flexio::W](pcc_flexio::W) writer structure"]
impl crate::Writable for PCC_FLEXIO {}
#[doc = "PCC FlexIO Register"]
pub mod pcc_flexio;
#[doc = "PCC EWM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_ewm](pcc_ewm) module"]
pub type PCC_EWM = crate::Reg<u32, _PCC_EWM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_EWM;
#[doc = "`read()` method returns [pcc_ewm::R](pcc_ewm::R) reader structure"]
impl crate::Readable for PCC_EWM {}
#[doc = "`write(|w| ..)` method takes [pcc_ewm::W](pcc_ewm::W) writer structure"]
impl crate::Writable for PCC_EWM {}
#[doc = "PCC EWM Register"]
pub mod pcc_ewm;
#[doc = "PCC LPI2C0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpi2c0](pcc_lpi2c0) module"]
pub type PCC_LPI2C0 = crate::Reg<u32, _PCC_LPI2C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPI2C0;
#[doc = "`read()` method returns [pcc_lpi2c0::R](pcc_lpi2c0::R) reader structure"]
impl crate::Readable for PCC_LPI2C0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpi2c0::W](pcc_lpi2c0::W) writer structure"]
impl crate::Writable for PCC_LPI2C0 {}
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC LPUART0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpuart0](pcc_lpuart0) module"]
pub type PCC_LPUART0 = crate::Reg<u32, _PCC_LPUART0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART0;
#[doc = "`read()` method returns [pcc_lpuart0::R](pcc_lpuart0::R) reader structure"]
impl crate::Readable for PCC_LPUART0 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart0::W](pcc_lpuart0::W) writer structure"]
impl crate::Writable for PCC_LPUART0 {}
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC LPUART1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpuart1](pcc_lpuart1) module"]
pub type PCC_LPUART1 = crate::Reg<u32, _PCC_LPUART1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART1;
#[doc = "`read()` method returns [pcc_lpuart1::R](pcc_lpuart1::R) reader structure"]
impl crate::Readable for PCC_LPUART1 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart1::W](pcc_lpuart1::W) writer structure"]
impl crate::Writable for PCC_LPUART1 {}
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC LPUART2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_lpuart2](pcc_lpuart2) module"]
pub type PCC_LPUART2 = crate::Reg<u32, _PCC_LPUART2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_LPUART2;
#[doc = "`read()` method returns [pcc_lpuart2::R](pcc_lpuart2::R) reader structure"]
impl crate::Readable for PCC_LPUART2 {}
#[doc = "`write(|w| ..)` method takes [pcc_lpuart2::W](pcc_lpuart2::W) writer structure"]
impl crate::Writable for PCC_LPUART2 {}
#[doc = "PCC LPUART2 Register"]
pub mod pcc_lpuart2;
#[doc = "PCC CMP0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcc_cmp0](pcc_cmp0) module"]
pub type PCC_CMP0 = crate::Reg<u32, _PCC_CMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCC_CMP0;
#[doc = "`read()` method returns [pcc_cmp0::R](pcc_cmp0::R) reader structure"]
impl crate::Readable for PCC_CMP0 {}
#[doc = "`write(|w| ..)` method takes [pcc_cmp0::W](pcc_cmp0::W) writer structure"]
impl crate::Writable for PCC_CMP0 {}
#[doc = "PCC CMP0 Register"]
pub mod pcc_cmp0;
