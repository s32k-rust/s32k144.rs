#[doc = "Writer for register GPCHR"]
pub type W = crate::W<u32, super::GPCHR>;
#[doc = "Register GPCHR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPWD`"]
pub struct GPWD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum GPWE_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE_AW> for u16 {
    #[inline(always)]
    fn from(variant: GPWE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `GPWE`"]
pub struct GPWE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPWE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE_AW::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    pub fn gpwd(&mut self) -> GPWD_W {
        GPWD_W { w: self }
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    pub fn gpwe(&mut self) -> GPWE_W {
        GPWE_W { w: self }
    }
}
