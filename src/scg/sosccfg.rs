#[doc = "Reader of register SOSCCFG"]
pub type R = crate::R<u32, super::SOSCCFG>;
#[doc = "Writer for register SOSCCFG"]
pub type W = crate::W<u32, super::SOSCCFG>;
#[doc = "Register SOSCCFG `reset()`'s with value 0x10"]
impl crate::ResetValue for super::SOSCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFS_A {
    #[doc = "0: External reference clock selected"]
    _0 = 0,
    #[doc = "1: Internal crystal oscillator of OSC selected."]
    _1 = 1,
}
impl From<EREFS_A> for bool {
    #[inline(always)]
    fn from(variant: EREFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EREFS`"]
pub type EREFS_R = crate::R<bool, EREFS_A>;
impl EREFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFS_A {
        match self.bits {
            false => EREFS_A::_0,
            true => EREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EREFS_A::_1
    }
}
#[doc = "Write proxy for field `EREFS`"]
pub struct EREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> EREFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EREFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External reference clock selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS_A::_0)
    }
    #[doc = "Internal crystal oscillator of OSC selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS_A::_1)
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
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGO_A {
    #[doc = "0: Configure crystal oscillator for low-gain operation"]
    _0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation"]
    _1 = 1,
}
impl From<HGO_A> for bool {
    #[inline(always)]
    fn from(variant: HGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HGO`"]
pub type HGO_R = crate::R<bool, HGO_A>;
impl HGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO_A {
        match self.bits {
            false => HGO_A::_0,
            true => HGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HGO_A::_1
    }
}
#[doc = "Write proxy for field `HGO`"]
pub struct HGO_W<'a> {
    w: &'a mut W,
}
impl<'a> HGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HGO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configure crystal oscillator for low-gain operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO_A::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "System OSC Range Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "1: Low frequency range selected for the crystal oscillator"]
    _01 = 1,
    #[doc = "2: Medium frequency range selected for the crytstal oscillator"]
    _10 = 2,
    #[doc = "3: High frequency range selected for the crystal oscillator"]
    _11 = 3,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RANGE`"]
pub type RANGE_R = crate::R<u8, RANGE_A>;
impl RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(RANGE_A::_01),
            2 => Val(RANGE_A::_10),
            3 => Val(RANGE_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RANGE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RANGE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RANGE_A::_11
    }
}
#[doc = "Write proxy for field `RANGE`"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low frequency range selected for the crystal oscillator"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE_A::_01)
    }
    #[doc = "Medium frequency range selected for the crytstal oscillator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RANGE_A::_10)
    }
    #[doc = "High frequency range selected for the crystal oscillator"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RANGE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&self) -> EREFS_R {
        EREFS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&self) -> HGO_R {
        HGO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&mut self) -> EREFS_W {
        EREFS_W { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&mut self) -> HGO_W {
        HGO_W { w: self }
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
}
