#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ERM Configuration Register 0"]
    pub cr0: CR0,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - ERM Status Register 0"]
    pub sr0: SR0,
    _reserved2: [u8; 236usize],
    #[doc = "0x100 - ERM Memory n Error Address Register"]
    pub ear0: EAR0,
    _reserved3: [u8; 12usize],
    #[doc = "0x110 - ERM Memory n Error Address Register"]
    pub ear1: EAR1,
}
#[doc = "ERM Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u32, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "ERM Configuration Register 0"]
pub mod cr0;
#[doc = "ERM Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr0](sr0) module"]
pub type SR0 = crate::Reg<u32, _SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR0;
#[doc = "`read()` method returns [sr0::R](sr0::R) reader structure"]
impl crate::Readable for SR0 {}
#[doc = "`write(|w| ..)` method takes [sr0::W](sr0::W) writer structure"]
impl crate::Writable for SR0 {}
#[doc = "ERM Status Register 0"]
pub mod sr0;
#[doc = "ERM Memory n Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ear0](ear0) module"]
pub type EAR0 = crate::Reg<u32, _EAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EAR0;
#[doc = "`read()` method returns [ear0::R](ear0::R) reader structure"]
impl crate::Readable for EAR0 {}
#[doc = "ERM Memory n Error Address Register"]
pub mod ear0;
#[doc = "ERM Memory n Error Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ear1](ear1) module"]
pub type EAR1 = crate::Reg<u32, _EAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EAR1;
#[doc = "`read()` method returns [ear1::R](ear1::R) reader structure"]
impl crate::Readable for EAR1 {}
#[doc = "ERM Memory n Error Address Register"]
pub mod ear1;
