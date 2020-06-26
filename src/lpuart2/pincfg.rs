#[doc = "Reader of register PINCFG"]
pub type R = crate::R<u32, super::PINCFG>;
#[doc = "Writer for register PINCFG"]
pub type W = crate::W<u32, super::PINCFG>;
#[doc = "Register PINCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::PINCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Input trigger is disabled."]
    _00 = 0,
    #[doc = "1: Input trigger is used instead of RXD pin input."]
    _01 = 1,
    #[doc = "2: Input trigger is used instead of CTS_B pin input."]
    _10 = 2,
    #[doc = "3: Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    _11 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, TRGSEL_A>;
impl TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::_00,
            1 => TRGSEL_A::_01,
            2 => TRGSEL_A::_10,
            3 => TRGSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TRGSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TRGSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TRGSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TRGSEL_A::_11
    }
}
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input trigger is disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TRGSEL_A::_00)
    }
    #[doc = "Input trigger is used instead of RXD pin input."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TRGSEL_A::_01)
    }
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSEL_A::_10)
    }
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
}
