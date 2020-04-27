#[doc = "Reader of register TCD2_ATTR"]
pub type R = crate::R<u16, super::TCD2_ATTR>;
#[doc = "Writer for register TCD2_ATTR"]
pub type W = crate::W<u16, super::TCD2_ATTR>;
#[doc = "Register TCD2_ATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD2_ATTR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DMOD`"]
pub type DMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMOD`"]
pub struct DMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u16) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 8-bit"]
    _0 = 0,
    #[doc = "1: 16-bit"]
    _1 = 1,
    #[doc = "2: 32-bit"]
    _10 = 2,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIZE`"]
pub type SSIZE_R = crate::R<u8, SSIZE_A>;
impl SSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SSIZE_A::_0),
            1 => Val(SSIZE_A::_1),
            2 => Val(SSIZE_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSIZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSIZE_A::_10
    }
}
#[doc = "Write proxy for field `SSIZE`"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSIZE_A::_0)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSIZE_A::_1)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSIZE_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
#[doc = "Source Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Source address modulo feature is disabled"]
    _0 = 0,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD`"]
pub type SMOD_R = crate::R<u8, SMOD_A>;
impl SMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMOD_A::_0
    }
}
#[doc = "Write proxy for field `SMOD`"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMOD_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u16) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination data transfer size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&mut self) -> DMOD_W {
        DMOD_W { w: self }
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 11:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
}
