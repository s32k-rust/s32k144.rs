#[doc = "Reader of register PAIR0DEADTIME"]
pub type R = crate::R<u32, super::PAIR0DEADTIME>;
#[doc = "Writer for register PAIR0DEADTIME"]
pub type W = crate::W<u32, super::PAIR0DEADTIME>;
#[doc = "Register PAIR0DEADTIME `reset()`'s with value 0"]
impl crate::ResetValue for super::PAIR0DEADTIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTVAL`"]
pub type DTVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTVAL`"]
pub struct DTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Deadtime Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPS_A {
    #[doc = "0: Divide the FTM input clock by 1."]
    _0X = 0,
    #[doc = "2: Divide the FTM input clock by 4."]
    _10 = 2,
    #[doc = "3: Divide the FTM input clock by 16."]
    _11 = 3,
}
impl From<DTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTPS`"]
pub type DTPS_R = crate::R<u8, DTPS_A>;
impl DTPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTPS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTPS_A::_0X),
            2 => Val(DTPS_A::_10),
            3 => Val(DTPS_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X`"]
    #[inline(always)]
    pub fn is_0x(&self) -> bool {
        *self == DTPS_A::_0X
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DTPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DTPS_A::_11
    }
}
#[doc = "Write proxy for field `DTPS`"]
pub struct DTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide the FTM input clock by 1."]
    #[inline(always)]
    pub fn _0x(self) -> &'a mut W {
        self.variant(DTPS_A::_0X)
    }
    #[doc = "Divide the FTM input clock by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTPS_A::_10)
    }
    #[doc = "Divide the FTM input clock by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DTPS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DTVALEX`"]
pub type DTVALEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTVALEX`"]
pub struct DTVALEX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTVALEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&self) -> DTVAL_R {
        DTVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&self) -> DTPS_R {
        DTPS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline(always)]
    pub fn dtvalex(&self) -> DTVALEX_R {
        DTVALEX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline(always)]
    pub fn dtval(&mut self) -> DTVAL_W {
        DTVAL_W { w: self }
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline(always)]
    pub fn dtps(&mut self) -> DTPS_W {
        DTPS_W { w: self }
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline(always)]
    pub fn dtvalex(&mut self) -> DTVALEX_W {
        DTVALEX_W { w: self }
    }
}
