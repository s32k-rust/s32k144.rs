#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Register 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC Status and Control Register 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC Status and Control Register 1"]
    pub sc1c: SC1,
    #[doc = "0x0c - ADC Status and Control Register 1"]
    pub sc1d: SC1,
    #[doc = "0x10 - ADC Status and Control Register 1"]
    pub sc1e: SC1,
    #[doc = "0x14 - ADC Status and Control Register 1"]
    pub sc1f: SC1,
    #[doc = "0x18 - ADC Status and Control Register 1"]
    pub sc1g: SC1,
    #[doc = "0x1c - ADC Status and Control Register 1"]
    pub sc1h: SC1,
    #[doc = "0x20 - ADC Status and Control Register 1"]
    pub sc1i: SC1,
    #[doc = "0x24 - ADC Status and Control Register 1"]
    pub sc1j: SC1,
    #[doc = "0x28 - ADC Status and Control Register 1"]
    pub sc1k: SC1,
    #[doc = "0x2c - ADC Status and Control Register 1"]
    pub sc1l: SC1,
    #[doc = "0x30 - ADC Status and Control Register 1"]
    pub sc1m: SC1,
    #[doc = "0x34 - ADC Status and Control Register 1"]
    pub sc1n: SC1,
    #[doc = "0x38 - ADC Status and Control Register 1"]
    pub sc1o: SC1,
    #[doc = "0x3c - ADC Status and Control Register 1"]
    pub sc1p: SC1,
    #[doc = "0x40 - ADC Configuration Register 1"]
    pub cfg1: CFG1,
    #[doc = "0x44 - ADC Configuration Register 2"]
    pub cfg2: CFG2,
    #[doc = "0x48 - ADC Data Result Registers"]
    pub ra: R,
    #[doc = "0x4c - ADC Data Result Registers"]
    pub rb: R,
    #[doc = "0x50 - ADC Data Result Registers"]
    pub rc: R,
    #[doc = "0x54 - ADC Data Result Registers"]
    pub rd: R,
    #[doc = "0x58 - ADC Data Result Registers"]
    pub re: R,
    #[doc = "0x5c - ADC Data Result Registers"]
    pub rf: R,
    #[doc = "0x60 - ADC Data Result Registers"]
    pub rg: R,
    #[doc = "0x64 - ADC Data Result Registers"]
    pub rh: R,
    #[doc = "0x68 - ADC Data Result Registers"]
    pub ri: R,
    #[doc = "0x6c - ADC Data Result Registers"]
    pub rj: R,
    #[doc = "0x70 - ADC Data Result Registers"]
    pub rk: R,
    #[doc = "0x74 - ADC Data Result Registers"]
    pub rl: R,
    #[doc = "0x78 - ADC Data Result Registers"]
    pub rm: R,
    #[doc = "0x7c - ADC Data Result Registers"]
    pub rn: R,
    #[doc = "0x80 - ADC Data Result Registers"]
    pub ro: R,
    #[doc = "0x84 - ADC Data Result Registers"]
    pub rp: R,
    #[doc = "0x88 - Compare Value Registers"]
    pub cv1: CV,
    #[doc = "0x8c - Compare Value Registers"]
    pub cv2: CV,
    #[doc = "0x90 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x94 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x98 - BASE Offset Register"]
    pub base_ofs: BASE_OFS,
    #[doc = "0x9c - ADC Offset Correction Register"]
    pub ofs: OFS,
    #[doc = "0xa0 - USER Offset Correction Register"]
    pub usr_ofs: USR_OFS,
    #[doc = "0xa4 - ADC X Offset Correction Register"]
    pub xofs: XOFS,
    #[doc = "0xa8 - ADC Y Offset Correction Register"]
    pub yofs: YOFS,
    #[doc = "0xac - ADC Gain Register"]
    pub g: G,
    #[doc = "0xb0 - ADC User Gain Register"]
    pub ug: UG,
    #[doc = "0xb4 - ADC General Calibration Value Register S"]
    pub clps: CLPS,
    #[doc = "0xb8 - ADC Plus-Side General Calibration Value Register 3"]
    pub clp3: CLP3,
    #[doc = "0xbc - ADC Plus-Side General Calibration Value Register 2"]
    pub clp2: CLP2,
    #[doc = "0xc0 - ADC Plus-Side General Calibration Value Register 1"]
    pub clp1: CLP1,
    #[doc = "0xc4 - ADC Plus-Side General Calibration Value Register 0"]
    pub clp0: CLP0,
    #[doc = "0xc8 - ADC Plus-Side General Calibration Value Register X"]
    pub clpx: CLPX,
    #[doc = "0xcc - ADC Plus-Side General Calibration Value Register 9"]
    pub clp9: CLP9,
    #[doc = "0xd0 - ADC General Calibration Offset Value Register S"]
    pub clps_ofs: CLPS_OFS,
    #[doc = "0xd4 - ADC Plus-Side General Calibration Offset Value Register 3"]
    pub clp3_ofs: CLP3_OFS,
    #[doc = "0xd8 - ADC Plus-Side General Calibration Offset Value Register 2"]
    pub clp2_ofs: CLP2_OFS,
    #[doc = "0xdc - ADC Plus-Side General Calibration Offset Value Register 1"]
    pub clp1_ofs: CLP1_OFS,
    #[doc = "0xe0 - ADC Plus-Side General Calibration Offset Value Register 0"]
    pub clp0_ofs: CLP0_OFS,
    #[doc = "0xe4 - ADC Plus-Side General Calibration Offset Value Register X"]
    pub clpx_ofs: CLPX_OFS,
    #[doc = "0xe8 - ADC Plus-Side General Calibration Offset Value Register 9"]
    pub clp9_ofs: CLP9_OFS,
}
#[doc = "ADC Status and Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](sc1) module"]
pub type SC1 = crate::Reg<u32, _SC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC1;
#[doc = "`read()` method returns [sc1::R](sc1::R) reader structure"]
impl crate::Readable for SC1 {}
#[doc = "`write(|w| ..)` method takes [sc1::W](sc1::W) writer structure"]
impl crate::Writable for SC1 {}
#[doc = "ADC Status and Control Register 1"]
pub mod sc1;
#[doc = "ADC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "ADC Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "ADC Data Result Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](r) module"]
pub type R = crate::Reg<u32, _R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R;
#[doc = "`read()` method returns [r::R](r::R) reader structure"]
impl crate::Readable for R {}
#[doc = "ADC Data Result Registers"]
pub mod r;
#[doc = "Compare Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](sc2) module"]
pub type SC2 = crate::Reg<u32, _SC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC2;
#[doc = "`read()` method returns [sc2::R](sc2::R) reader structure"]
impl crate::Readable for SC2 {}
#[doc = "`write(|w| ..)` method takes [sc2::W](sc2::W) writer structure"]
impl crate::Writable for SC2 {}
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "Status and Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](sc3) module"]
pub type SC3 = crate::Reg<u32, _SC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC3;
#[doc = "`read()` method returns [sc3::R](sc3::R) reader structure"]
impl crate::Readable for SC3 {}
#[doc = "`write(|w| ..)` method takes [sc3::W](sc3::W) writer structure"]
impl crate::Writable for SC3 {}
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "BASE Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_ofs](base_ofs) module"]
pub type BASE_OFS = crate::Reg<u32, _BASE_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BASE_OFS;
#[doc = "`read()` method returns [base_ofs::R](base_ofs::R) reader structure"]
impl crate::Readable for BASE_OFS {}
#[doc = "`write(|w| ..)` method takes [base_ofs::W](base_ofs::W) writer structure"]
impl crate::Writable for BASE_OFS {}
#[doc = "BASE Offset Register"]
pub mod base_ofs;
#[doc = "ADC Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](ofs) module"]
pub type OFS = crate::Reg<u32, _OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFS;
#[doc = "`read()` method returns [ofs::R](ofs::R) reader structure"]
impl crate::Readable for OFS {}
#[doc = "`write(|w| ..)` method takes [ofs::W](ofs::W) writer structure"]
impl crate::Writable for OFS {}
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "USER Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usr_ofs](usr_ofs) module"]
pub type USR_OFS = crate::Reg<u32, _USR_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USR_OFS;
#[doc = "`read()` method returns [usr_ofs::R](usr_ofs::R) reader structure"]
impl crate::Readable for USR_OFS {}
#[doc = "`write(|w| ..)` method takes [usr_ofs::W](usr_ofs::W) writer structure"]
impl crate::Writable for USR_OFS {}
#[doc = "USER Offset Correction Register"]
pub mod usr_ofs;
#[doc = "ADC X Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xofs](xofs) module"]
pub type XOFS = crate::Reg<u32, _XOFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOFS;
#[doc = "`read()` method returns [xofs::R](xofs::R) reader structure"]
impl crate::Readable for XOFS {}
#[doc = "`write(|w| ..)` method takes [xofs::W](xofs::W) writer structure"]
impl crate::Writable for XOFS {}
#[doc = "ADC X Offset Correction Register"]
pub mod xofs;
#[doc = "ADC Y Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [yofs](yofs) module"]
pub type YOFS = crate::Reg<u32, _YOFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YOFS;
#[doc = "`read()` method returns [yofs::R](yofs::R) reader structure"]
impl crate::Readable for YOFS {}
#[doc = "`write(|w| ..)` method takes [yofs::W](yofs::W) writer structure"]
impl crate::Writable for YOFS {}
#[doc = "ADC Y Offset Correction Register"]
pub mod yofs;
#[doc = "ADC Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g](g) module"]
pub type G = crate::Reg<u32, _G>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _G;
#[doc = "`read()` method returns [g::R](g::R) reader structure"]
impl crate::Readable for G {}
#[doc = "`write(|w| ..)` method takes [g::W](g::W) writer structure"]
impl crate::Writable for G {}
#[doc = "ADC Gain Register"]
pub mod g;
#[doc = "ADC User Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ug](ug) module"]
pub type UG = crate::Reg<u32, _UG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UG;
#[doc = "`read()` method returns [ug::R](ug::R) reader structure"]
impl crate::Readable for UG {}
#[doc = "`write(|w| ..)` method takes [ug::W](ug::W) writer structure"]
impl crate::Writable for UG {}
#[doc = "ADC User Gain Register"]
pub mod ug;
#[doc = "ADC General Calibration Value Register S\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps](clps) module"]
pub type CLPS = crate::Reg<u32, _CLPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPS;
#[doc = "`read()` method returns [clps::R](clps::R) reader structure"]
impl crate::Readable for CLPS {}
#[doc = "`write(|w| ..)` method takes [clps::W](clps::W) writer structure"]
impl crate::Writable for CLPS {}
#[doc = "ADC General Calibration Value Register S"]
pub mod clps;
#[doc = "ADC Plus-Side General Calibration Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp3](clp3) module"]
pub type CLP3 = crate::Reg<u32, _CLP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP3;
#[doc = "`read()` method returns [clp3::R](clp3::R) reader structure"]
impl crate::Readable for CLP3 {}
#[doc = "`write(|w| ..)` method takes [clp3::W](clp3::W) writer structure"]
impl crate::Writable for CLP3 {}
#[doc = "ADC Plus-Side General Calibration Value Register 3"]
pub mod clp3;
#[doc = "ADC Plus-Side General Calibration Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2](clp2) module"]
pub type CLP2 = crate::Reg<u32, _CLP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP2;
#[doc = "`read()` method returns [clp2::R](clp2::R) reader structure"]
impl crate::Readable for CLP2 {}
#[doc = "`write(|w| ..)` method takes [clp2::W](clp2::W) writer structure"]
impl crate::Writable for CLP2 {}
#[doc = "ADC Plus-Side General Calibration Value Register 2"]
pub mod clp2;
#[doc = "ADC Plus-Side General Calibration Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1](clp1) module"]
pub type CLP1 = crate::Reg<u32, _CLP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP1;
#[doc = "`read()` method returns [clp1::R](clp1::R) reader structure"]
impl crate::Readable for CLP1 {}
#[doc = "`write(|w| ..)` method takes [clp1::W](clp1::W) writer structure"]
impl crate::Writable for CLP1 {}
#[doc = "ADC Plus-Side General Calibration Value Register 1"]
pub mod clp1;
#[doc = "ADC Plus-Side General Calibration Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp0](clp0) module"]
pub type CLP0 = crate::Reg<u32, _CLP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP0;
#[doc = "`read()` method returns [clp0::R](clp0::R) reader structure"]
impl crate::Readable for CLP0 {}
#[doc = "`write(|w| ..)` method takes [clp0::W](clp0::W) writer structure"]
impl crate::Writable for CLP0 {}
#[doc = "ADC Plus-Side General Calibration Value Register 0"]
pub mod clp0;
#[doc = "ADC Plus-Side General Calibration Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpx](clpx) module"]
pub type CLPX = crate::Reg<u32, _CLPX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPX;
#[doc = "`read()` method returns [clpx::R](clpx::R) reader structure"]
impl crate::Readable for CLPX {}
#[doc = "`write(|w| ..)` method takes [clpx::W](clpx::W) writer structure"]
impl crate::Writable for CLPX {}
#[doc = "ADC Plus-Side General Calibration Value Register X"]
pub mod clpx;
#[doc = "ADC Plus-Side General Calibration Value Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp9](clp9) module"]
pub type CLP9 = crate::Reg<u32, _CLP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP9;
#[doc = "`read()` method returns [clp9::R](clp9::R) reader structure"]
impl crate::Readable for CLP9 {}
#[doc = "`write(|w| ..)` method takes [clp9::W](clp9::W) writer structure"]
impl crate::Writable for CLP9 {}
#[doc = "ADC Plus-Side General Calibration Value Register 9"]
pub mod clp9;
#[doc = "ADC General Calibration Offset Value Register S\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps_ofs](clps_ofs) module"]
pub type CLPS_OFS = crate::Reg<u32, _CLPS_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPS_OFS;
#[doc = "`read()` method returns [clps_ofs::R](clps_ofs::R) reader structure"]
impl crate::Readable for CLPS_OFS {}
#[doc = "`write(|w| ..)` method takes [clps_ofs::W](clps_ofs::W) writer structure"]
impl crate::Writable for CLPS_OFS {}
#[doc = "ADC General Calibration Offset Value Register S"]
pub mod clps_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp3_ofs](clp3_ofs) module"]
pub type CLP3_OFS = crate::Reg<u32, _CLP3_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP3_OFS;
#[doc = "`read()` method returns [clp3_ofs::R](clp3_ofs::R) reader structure"]
impl crate::Readable for CLP3_OFS {}
#[doc = "`write(|w| ..)` method takes [clp3_ofs::W](clp3_ofs::W) writer structure"]
impl crate::Writable for CLP3_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 3"]
pub mod clp3_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2_ofs](clp2_ofs) module"]
pub type CLP2_OFS = crate::Reg<u32, _CLP2_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP2_OFS;
#[doc = "`read()` method returns [clp2_ofs::R](clp2_ofs::R) reader structure"]
impl crate::Readable for CLP2_OFS {}
#[doc = "`write(|w| ..)` method takes [clp2_ofs::W](clp2_ofs::W) writer structure"]
impl crate::Writable for CLP2_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2"]
pub mod clp2_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1_ofs](clp1_ofs) module"]
pub type CLP1_OFS = crate::Reg<u32, _CLP1_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP1_OFS;
#[doc = "`read()` method returns [clp1_ofs::R](clp1_ofs::R) reader structure"]
impl crate::Readable for CLP1_OFS {}
#[doc = "`write(|w| ..)` method takes [clp1_ofs::W](clp1_ofs::W) writer structure"]
impl crate::Writable for CLP1_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1"]
pub mod clp1_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp0_ofs](clp0_ofs) module"]
pub type CLP0_OFS = crate::Reg<u32, _CLP0_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP0_OFS;
#[doc = "`read()` method returns [clp0_ofs::R](clp0_ofs::R) reader structure"]
impl crate::Readable for CLP0_OFS {}
#[doc = "`write(|w| ..)` method takes [clp0_ofs::W](clp0_ofs::W) writer structure"]
impl crate::Writable for CLP0_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 0"]
pub mod clp0_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpx_ofs](clpx_ofs) module"]
pub type CLPX_OFS = crate::Reg<u32, _CLPX_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPX_OFS;
#[doc = "`read()` method returns [clpx_ofs::R](clpx_ofs::R) reader structure"]
impl crate::Readable for CLPX_OFS {}
#[doc = "`write(|w| ..)` method takes [clpx_ofs::W](clpx_ofs::W) writer structure"]
impl crate::Writable for CLPX_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register X"]
pub mod clpx_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp9_ofs](clp9_ofs) module"]
pub type CLP9_OFS = crate::Reg<u32, _CLP9_OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP9_OFS;
#[doc = "`read()` method returns [clp9_ofs::R](clp9_ofs::R) reader structure"]
impl crate::Readable for CLP9_OFS {}
#[doc = "`write(|w| ..)` method takes [clp9_ofs::W](clp9_ofs::W) writer structure"]
impl crate::Writable for CLP9_OFS {}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 9"]
pub mod clp9_ofs;
