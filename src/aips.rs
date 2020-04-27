#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Privilege Register A"]
    pub mpra: MPRA,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - Peripheral Access Control Register"]
    pub pacra: PACRA,
    #[doc = "0x24 - Peripheral Access Control Register"]
    pub pacrb: PACRB,
    #[doc = "0x28 - Peripheral Access Control Register"]
    pub pacrc: PACRC,
    #[doc = "0x2c - Peripheral Access Control Register"]
    pub pacrd: PACRD,
    _reserved5: [u8; 16usize],
    #[doc = "0x40 - Off-Platform Peripheral Access Control Register"]
    pub opacra: OPACRA,
    #[doc = "0x44 - Off-Platform Peripheral Access Control Register"]
    pub opacrb: OPACRB,
    #[doc = "0x48 - Off-Platform Peripheral Access Control Register"]
    pub opacrc: OPACRC,
    #[doc = "0x4c - Off-Platform Peripheral Access Control Register"]
    pub opacrd: OPACRD,
    #[doc = "0x50 - Off-Platform Peripheral Access Control Register"]
    pub opacre: OPACRE,
    #[doc = "0x54 - Off-Platform Peripheral Access Control Register"]
    pub opacrf: OPACRF,
    #[doc = "0x58 - Off-Platform Peripheral Access Control Register"]
    pub opacrg: OPACRG,
    #[doc = "0x5c - Off-Platform Peripheral Access Control Register"]
    pub opacrh: OPACRH,
    #[doc = "0x60 - Off-Platform Peripheral Access Control Register"]
    pub opacri: OPACRI,
    #[doc = "0x64 - Off-Platform Peripheral Access Control Register"]
    pub opacrj: OPACRJ,
    #[doc = "0x68 - Off-Platform Peripheral Access Control Register"]
    pub opacrk: OPACRK,
    #[doc = "0x6c - Off-Platform Peripheral Access Control Register"]
    pub opacrl: OPACRL,
}
#[doc = "Master Privilege Register A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpra](mpra) module"]
pub type MPRA = crate::Reg<u32, _MPRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPRA;
#[doc = "`read()` method returns [mpra::R](mpra::R) reader structure"]
impl crate::Readable for MPRA {}
#[doc = "`write(|w| ..)` method takes [mpra::W](mpra::W) writer structure"]
impl crate::Writable for MPRA {}
#[doc = "Master Privilege Register A"]
pub mod mpra;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacra](pacra) module"]
pub type PACRA = crate::Reg<u32, _PACRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRA;
#[doc = "`read()` method returns [pacra::R](pacra::R) reader structure"]
impl crate::Readable for PACRA {}
#[doc = "`write(|w| ..)` method takes [pacra::W](pacra::W) writer structure"]
impl crate::Writable for PACRA {}
#[doc = "Peripheral Access Control Register"]
pub mod pacra;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrb](pacrb) module"]
pub type PACRB = crate::Reg<u32, _PACRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRB;
#[doc = "`read()` method returns [pacrb::R](pacrb::R) reader structure"]
impl crate::Readable for PACRB {}
#[doc = "`write(|w| ..)` method takes [pacrb::W](pacrb::W) writer structure"]
impl crate::Writable for PACRB {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrb;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrc](pacrc) module"]
pub type PACRC = crate::Reg<u32, _PACRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRC;
#[doc = "`read()` method returns [pacrc::R](pacrc::R) reader structure"]
impl crate::Readable for PACRC {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrc;
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrd](pacrd) module"]
pub type PACRD = crate::Reg<u32, _PACRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACRD;
#[doc = "`read()` method returns [pacrd::R](pacrd::R) reader structure"]
impl crate::Readable for PACRD {}
#[doc = "`write(|w| ..)` method takes [pacrd::W](pacrd::W) writer structure"]
impl crate::Writable for PACRD {}
#[doc = "Peripheral Access Control Register"]
pub mod pacrd;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacra](opacra) module"]
pub type OPACRA = crate::Reg<u32, _OPACRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRA;
#[doc = "`read()` method returns [opacra::R](opacra::R) reader structure"]
impl crate::Readable for OPACRA {}
#[doc = "`write(|w| ..)` method takes [opacra::W](opacra::W) writer structure"]
impl crate::Writable for OPACRA {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacra;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrb](opacrb) module"]
pub type OPACRB = crate::Reg<u32, _OPACRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRB;
#[doc = "`read()` method returns [opacrb::R](opacrb::R) reader structure"]
impl crate::Readable for OPACRB {}
#[doc = "`write(|w| ..)` method takes [opacrb::W](opacrb::W) writer structure"]
impl crate::Writable for OPACRB {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrb;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrc](opacrc) module"]
pub type OPACRC = crate::Reg<u32, _OPACRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRC;
#[doc = "`read()` method returns [opacrc::R](opacrc::R) reader structure"]
impl crate::Readable for OPACRC {}
#[doc = "`write(|w| ..)` method takes [opacrc::W](opacrc::W) writer structure"]
impl crate::Writable for OPACRC {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrc;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrd](opacrd) module"]
pub type OPACRD = crate::Reg<u32, _OPACRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRD;
#[doc = "`read()` method returns [opacrd::R](opacrd::R) reader structure"]
impl crate::Readable for OPACRD {}
#[doc = "`write(|w| ..)` method takes [opacrd::W](opacrd::W) writer structure"]
impl crate::Writable for OPACRD {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrd;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacre](opacre) module"]
pub type OPACRE = crate::Reg<u32, _OPACRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRE;
#[doc = "`read()` method returns [opacre::R](opacre::R) reader structure"]
impl crate::Readable for OPACRE {}
#[doc = "`write(|w| ..)` method takes [opacre::W](opacre::W) writer structure"]
impl crate::Writable for OPACRE {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacre;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrf](opacrf) module"]
pub type OPACRF = crate::Reg<u32, _OPACRF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRF;
#[doc = "`read()` method returns [opacrf::R](opacrf::R) reader structure"]
impl crate::Readable for OPACRF {}
#[doc = "`write(|w| ..)` method takes [opacrf::W](opacrf::W) writer structure"]
impl crate::Writable for OPACRF {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrf;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrg](opacrg) module"]
pub type OPACRG = crate::Reg<u32, _OPACRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRG;
#[doc = "`read()` method returns [opacrg::R](opacrg::R) reader structure"]
impl crate::Readable for OPACRG {}
#[doc = "`write(|w| ..)` method takes [opacrg::W](opacrg::W) writer structure"]
impl crate::Writable for OPACRG {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrg;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrh](opacrh) module"]
pub type OPACRH = crate::Reg<u32, _OPACRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRH;
#[doc = "`read()` method returns [opacrh::R](opacrh::R) reader structure"]
impl crate::Readable for OPACRH {}
#[doc = "`write(|w| ..)` method takes [opacrh::W](opacrh::W) writer structure"]
impl crate::Writable for OPACRH {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrh;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacri](opacri) module"]
pub type OPACRI = crate::Reg<u32, _OPACRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRI;
#[doc = "`read()` method returns [opacri::R](opacri::R) reader structure"]
impl crate::Readable for OPACRI {}
#[doc = "`write(|w| ..)` method takes [opacri::W](opacri::W) writer structure"]
impl crate::Writable for OPACRI {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacri;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrj](opacrj) module"]
pub type OPACRJ = crate::Reg<u32, _OPACRJ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRJ;
#[doc = "`read()` method returns [opacrj::R](opacrj::R) reader structure"]
impl crate::Readable for OPACRJ {}
#[doc = "`write(|w| ..)` method takes [opacrj::W](opacrj::W) writer structure"]
impl crate::Writable for OPACRJ {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrj;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrk](opacrk) module"]
pub type OPACRK = crate::Reg<u32, _OPACRK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRK;
#[doc = "`read()` method returns [opacrk::R](opacrk::R) reader structure"]
impl crate::Readable for OPACRK {}
#[doc = "`write(|w| ..)` method takes [opacrk::W](opacrk::W) writer structure"]
impl crate::Writable for OPACRK {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrk;
#[doc = "Off-Platform Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacrl](opacrl) module"]
pub type OPACRL = crate::Reg<u32, _OPACRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACRL;
#[doc = "`read()` method returns [opacrl::R](opacrl::R) reader structure"]
impl crate::Readable for OPACRL {}
#[doc = "`write(|w| ..)` method takes [opacrl::W](opacrl::W) writer structure"]
impl crate::Writable for OPACRL {}
#[doc = "Off-Platform Peripheral Access Control Register"]
pub mod opacrl;
