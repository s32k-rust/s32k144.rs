#[doc = "Reader of register SIRCCFG"]
pub type R = crate::R<u32, super::SIRCCFG>;
#[doc = "Writer for register SIRCCFG"]
pub type W = crate::W<u32, super::SIRCCFG>;
#[doc = "Register SIRCCFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SIRCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Frequency Range\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_A {
    #[doc = "0: Slow IRC low range clock (2 MHz)"]
    _0 = 0,
    #[doc = "1: Slow IRC high range clock (8 MHz )"]
    _1 = 1,
}
impl From<RANGE_A> for bool {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RANGE`"]
pub type RANGE_R = crate::R<bool, RANGE_A>;
impl RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGE_A {
        match self.bits {
            false => RANGE_A::_0,
            true => RANGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RANGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RANGE_A::_1
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
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Slow IRC low range clock (2 MHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RANGE_A::_0)
    }
    #[doc = "Slow IRC high range clock (8 MHz )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RANGE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Frequency Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
}
