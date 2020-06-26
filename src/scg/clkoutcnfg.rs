#[doc = "Reader of register CLKOUTCNFG"]
pub type R = crate::R<u32, super::CLKOUTCNFG>;
#[doc = "Writer for register CLKOUTCNFG"]
pub type W = crate::W<u32, super::CLKOUTCNFG>;
#[doc = "Register CLKOUTCNFG `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::CLKOUTCNFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_0000
    }
}
#[doc = "SCG Clkout Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: SCG SLOW Clock"]
    _0000 = 0,
    #[doc = "1: System OSC (SOSC_CLK)"]
    _0001 = 1,
    #[doc = "2: Slow IRC (SIRC_CLK)"]
    _0010 = 2,
    #[doc = "3: Fast IRC (FIRC_CLK)"]
    _0011 = 3,
    #[doc = "6: System PLL (SPLL_CLK)"]
    _0110 = 6,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL_A::_0000),
            1 => Val(CLKOUTSEL_A::_0001),
            2 => Val(CLKOUTSEL_A::_0010),
            3 => Val(CLKOUTSEL_A::_0011),
            6 => Val(CLKOUTSEL_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CLKOUTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == CLKOUTSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CLKOUTSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == CLKOUTSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CLKOUTSEL_A::_0110
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCG SLOW Clock"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0000)
    }
    #[doc = "System OSC (SOSC_CLK)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0001)
    }
    #[doc = "Slow IRC (SIRC_CLK)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0010)
    }
    #[doc = "Fast IRC (FIRC_CLK)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0011)
    }
    #[doc = "System PLL (SPLL_CLK)"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
}
