#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    pub c0: C0,
    #[doc = "0x04 - CMP Control Register 1"]
    pub c1: C1,
    #[doc = "0x08 - CMP Control Register 2"]
    pub c2: C2,
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](c0) module"]
pub type C0 = crate::Reg<u32, _C0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C0;
#[doc = "`read()` method returns [c0::R](c0::R) reader structure"]
impl crate::Readable for C0 {}
#[doc = "`write(|w| ..)` method takes [c0::W](c0::W) writer structure"]
impl crate::Writable for C0 {}
#[doc = "CMP Control Register 0"]
pub mod c0;
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](c1) module"]
pub type C1 = crate::Reg<u32, _C1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1;
#[doc = "`read()` method returns [c1::R](c1::R) reader structure"]
impl crate::Readable for C1 {}
#[doc = "`write(|w| ..)` method takes [c1::W](c1::W) writer structure"]
impl crate::Writable for C1 {}
#[doc = "CMP Control Register 1"]
pub mod c1;
#[doc = "CMP Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](c2) module"]
pub type C2 = crate::Reg<u32, _C2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2;
#[doc = "`read()` method returns [c2::R](c2::R) reader structure"]
impl crate::Readable for C2 {}
#[doc = "`write(|w| ..)` method takes [c2::W](c2::W) writer structure"]
impl crate::Writable for C2 {}
#[doc = "CMP Control Register 2"]
pub mod c2;
