#[doc = "Reader of register QDCTRL"]
pub type R = crate::R<u32, super::QDCTRL>;
#[doc = "Writer for register QDCTRL"]
pub type W = crate::W<u32, super::QDCTRL>;
#[doc = "Register QDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::QDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Quadrature Decoder Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADEN_A {
    #[doc = "0: Quadrature Decoder mode is disabled."]
    _0 = 0,
    #[doc = "1: Quadrature Decoder mode is enabled."]
    _1 = 1,
}
impl From<QUADEN_A> for bool {
    #[inline(always)]
    fn from(variant: QUADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QUADEN`"]
pub type QUADEN_R = crate::R<bool, QUADEN_A>;
impl QUADEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADEN_A {
        match self.bits {
            false => QUADEN_A::_0,
            true => QUADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADEN_A::_1
    }
}
#[doc = "Write proxy for field `QUADEN`"]
pub struct QUADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Quadrature Decoder mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADEN_A::_0)
    }
    #[doc = "Quadrature Decoder mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADEN_A::_1)
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
#[doc = "Timer Overflow Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIR_A {
    #[doc = "0: TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (CNTIN register) to its maximum value (MOD register)."]
    _0 = 0,
    #[doc = "1: TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (CNTIN register)."]
    _1 = 1,
}
impl From<TOFDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TOFDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOFDIR`"]
pub type TOFDIR_R = crate::R<bool, TOFDIR_A>;
impl TOFDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOFDIR_A {
        match self.bits {
            false => TOFDIR_A::_0,
            true => TOFDIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOFDIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOFDIR_A::_1
    }
}
#[doc = "FTM Counter Direction In Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIR_A {
    #[doc = "0: Counting direction is decreasing (FTM counter decrement)."]
    _0 = 0,
    #[doc = "1: Counting direction is increasing (FTM counter increment)."]
    _1 = 1,
}
impl From<QUADIR_A> for bool {
    #[inline(always)]
    fn from(variant: QUADIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QUADIR`"]
pub type QUADIR_R = crate::R<bool, QUADIR_A>;
impl QUADIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADIR_A {
        match self.bits {
            false => QUADIR_A::_0,
            true => QUADIR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADIR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADIR_A::_1
    }
}
#[doc = "Quadrature Decoder Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODE_A {
    #[doc = "0: Phase A and phase B encoding mode."]
    _0 = 0,
    #[doc = "1: Count and direction encoding mode."]
    _1 = 1,
}
impl From<QUADMODE_A> for bool {
    #[inline(always)]
    fn from(variant: QUADMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QUADMODE`"]
pub type QUADMODE_R = crate::R<bool, QUADMODE_A>;
impl QUADMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUADMODE_A {
        match self.bits {
            false => QUADMODE_A::_0,
            true => QUADMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == QUADMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == QUADMODE_A::_1
    }
}
#[doc = "Write proxy for field `QUADMODE`"]
pub struct QUADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QUADMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase A and phase B encoding mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADMODE_A::_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADMODE_A::_1)
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
#[doc = "Phase B Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBPOL_A {
    #[doc = "0: Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0 = 0,
    #[doc = "1: Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    _1 = 1,
}
impl From<PHBPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHBPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHBPOL`"]
pub type PHBPOL_R = crate::R<bool, PHBPOL_A>;
impl PHBPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBPOL_A {
        match self.bits {
            false => PHBPOL_A::_0,
            true => PHBPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHBPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHBPOL_A::_1
    }
}
#[doc = "Write proxy for field `PHBPOL`"]
pub struct PHBPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHBPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHBPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBPOL_A::_0)
    }
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Phase A Input Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAPOL_A {
    #[doc = "0: Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0 = 0,
    #[doc = "1: Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    _1 = 1,
}
impl From<PHAPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PHAPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHAPOL`"]
pub type PHAPOL_R = crate::R<bool, PHAPOL_A>;
impl PHAPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAPOL_A {
        match self.bits {
            false => PHAPOL_A::_0,
            true => PHAPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHAPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHAPOL_A::_1
    }
}
#[doc = "Write proxy for field `PHAPOL`"]
pub struct PHAPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHAPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHAPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAPOL_A::_0)
    }
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Phase B Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBFLTREN_A {
    #[doc = "0: Phase B input filter is disabled."]
    _0 = 0,
    #[doc = "1: Phase B input filter is enabled."]
    _1 = 1,
}
impl From<PHBFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHBFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHBFLTREN`"]
pub type PHBFLTREN_R = crate::R<bool, PHBFLTREN_A>;
impl PHBFLTREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHBFLTREN_A {
        match self.bits {
            false => PHBFLTREN_A::_0,
            true => PHBFLTREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHBFLTREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHBFLTREN_A::_1
    }
}
#[doc = "Write proxy for field `PHBFLTREN`"]
pub struct PHBFLTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHBFLTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHBFLTREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase B input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::_0)
    }
    #[doc = "Phase B input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBFLTREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Phase A Input Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAFLTREN_A {
    #[doc = "0: Phase A input filter is disabled."]
    _0 = 0,
    #[doc = "1: Phase A input filter is enabled."]
    _1 = 1,
}
impl From<PHAFLTREN_A> for bool {
    #[inline(always)]
    fn from(variant: PHAFLTREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PHAFLTREN`"]
pub type PHAFLTREN_R = crate::R<bool, PHAFLTREN_A>;
impl PHAFLTREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHAFLTREN_A {
        match self.bits {
            false => PHAFLTREN_A::_0,
            true => PHAFLTREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHAFLTREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PHAFLTREN_A::_1
    }
}
#[doc = "Write proxy for field `PHAFLTREN`"]
pub struct PHAFLTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHAFLTREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHAFLTREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Phase A input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::_0)
    }
    #[doc = "Phase A input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAFLTREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    pub fn quaden(&self) -> QUADEN_R {
        QUADEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Overflow Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn tofdir(&self) -> TOFDIR_R {
        TOFDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Direction In Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadir(&self) -> QUADIR_R {
        QUADIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&self) -> QUADMODE_R {
        QUADMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    pub fn phbpol(&self) -> PHBPOL_R {
        PHBPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    pub fn phapol(&self) -> PHAPOL_R {
        PHAPOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    pub fn phbfltren(&self) -> PHBFLTREN_R {
        PHBFLTREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    pub fn phafltren(&self) -> PHAFLTREN_R {
        PHAFLTREN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline(always)]
    pub fn quaden(&mut self) -> QUADEN_W {
        QUADEN_W { w: self }
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline(always)]
    pub fn quadmode(&mut self) -> QUADMODE_W {
        QUADMODE_W { w: self }
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline(always)]
    pub fn phbpol(&mut self) -> PHBPOL_W {
        PHBPOL_W { w: self }
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline(always)]
    pub fn phapol(&mut self) -> PHAPOL_W {
        PHAPOL_W { w: self }
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline(always)]
    pub fn phbfltren(&mut self) -> PHBFLTREN_W {
        PHBFLTREN_W { w: self }
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline(always)]
    pub fn phafltren(&mut self) -> PHAFLTREN_W {
        PHAFLTREN_W { w: self }
    }
}
