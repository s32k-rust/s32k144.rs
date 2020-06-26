#[doc = "Reader of register FIRCCFG"]
pub type R = crate::R<u32, super::FIRCCFG>;
#[doc = "Writer for register FIRCCFG"]
pub type W = crate::W<u32, super::FIRCCFG>;
#[doc = "Register FIRCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FIRCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frequency Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: Fast IRC is trimmed to 48 MHz"]
    _00 = 0,
    #[doc = "1: Fast IRC is trimmed to 52 MHz"]
    _01 = 1,
    #[doc = "2: Fast IRC is trimmed to 56 MHz"]
    _10 = 2,
    #[doc = "3: Fast IRC is trimmed to 60 MHz"]
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
    pub fn variant(&self) -> RANGE_A {
        match self.bits {
            0 => RANGE_A::_00,
            1 => RANGE_A::_01,
            2 => RANGE_A::_10,
            3 => RANGE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RANGE_A::_00
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Fast IRC is trimmed to 48 MHz"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGE_A::_00)
    }
    #[doc = "Fast IRC is trimmed to 52 MHz"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE_A::_01)
    }
    #[doc = "Fast IRC is trimmed to 56 MHz"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RANGE_A::_10)
    }
    #[doc = "Fast IRC is trimmed to 60 MHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RANGE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
}
