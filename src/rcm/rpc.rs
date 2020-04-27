#[doc = "Reader of register RPC"]
pub type R = crate::R<u32, super::RPC>;
#[doc = "Writer for register RPC"]
pub type W = crate::W<u32, super::RPC>;
#[doc = "Register RPC `reset()`'s with value 0"]
impl crate::ResetValue for super::RPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset Pin Filter Select in Run and Wait Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTFLTSRW_A {
    #[doc = "0: All filtering disabled"]
    _00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    _01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    _10 = 2,
}
impl From<RSTFLTSRW_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTFLTSRW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTFLTSRW`"]
pub type RSTFLTSRW_R = crate::R<u8, RSTFLTSRW_A>;
impl RSTFLTSRW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTFLTSRW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSTFLTSRW_A::_00),
            1 => Val(RSTFLTSRW_A::_01),
            2 => Val(RSTFLTSRW_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSTFLTSRW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSTFLTSRW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSTFLTSRW_A::_10
    }
}
#[doc = "Write proxy for field `RSTFLTSRW`"]
pub struct RSTFLTSRW_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSRW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reset Pin Filter Select in Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSS_A {
    #[doc = "0: All filtering disabled"]
    _0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    _1 = 1,
}
impl From<RSTFLTSS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFLTSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTFLTSS`"]
pub type RSTFLTSS_R = crate::R<bool, RSTFLTSS_A>;
impl RSTFLTSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSS_A {
        match self.bits {
            false => RSTFLTSS_A::_0,
            true => RSTFLTSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTFLTSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTFLTSS_A::_1
    }
}
#[doc = "Write proxy for field `RSTFLTSS`"]
pub struct RSTFLTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RSTFLTSEL`"]
pub type RSTFLTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSTFLTSEL`"]
pub struct RSTFLTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RSTFLTSRW_R {
        RSTFLTSRW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RSTFLTSS_R {
        RSTFLTSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    pub fn rstfltsel(&self) -> RSTFLTSEL_R {
        RSTFLTSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&mut self) -> RSTFLTSRW_W {
        RSTFLTSRW_W { w: self }
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&mut self) -> RSTFLTSS_W {
        RSTFLTSS_W { w: self }
    }
    #[doc = "Bits 8:12 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    pub fn rstfltsel(&mut self) -> RSTFLTSEL_W {
        RSTFLTSEL_W { w: self }
    }
}
