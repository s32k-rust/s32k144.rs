#[doc = "Reader of register STOPCTRL"]
pub type R = crate::R<u32, super::STOPCTRL>;
#[doc = "Writer for register STOPCTRL"]
pub type W = crate::W<u32, super::STOPCTRL>;
#[doc = "Register STOPCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::STOPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STOPO_A {
    #[doc = "1: STOP1 - Stop with both system and bus clocks disabled"]
    _01 = 1,
    #[doc = "2: STOP2 - Stop with system clock disabled and bus clock enabled"]
    _10 = 2,
}
impl From<STOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STOPO`"]
pub type STOPO_R = crate::R<u8, STOPO_A>;
impl STOPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STOPO_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(STOPO_A::_01),
            2 => Val(STOPO_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == STOPO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == STOPO_A::_10
    }
}
#[doc = "Write proxy for field `STOPO`"]
pub struct STOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STOP1 - Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(STOPO_A::_01)
    }
    #[doc = "STOP2 - Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(STOPO_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline(always)]
    pub fn stopo(&self) -> STOPO_R {
        STOPO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline(always)]
    pub fn stopo(&mut self) -> STOPO_W {
        STOPO_W { w: self }
    }
}
