#[doc = "Reader of register MIER"]
pub type R = crate::R<u32, super::MIER>;
#[doc = "Writer for register MIER"]
pub type W = crate::W<u32, super::MIER>;
#[doc = "Register MIER `reset()`'s with value 0"]
impl crate::ResetValue for super::MIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE0_A {
    #[doc = "0: Interrupt generation is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation is enabled"]
    _1 = 1,
}
impl From<TIE0_A> for bool {
    #[inline(always)]
    fn from(variant: TIE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE0`"]
pub type TIE0_R = crate::R<bool, TIE0_A>;
impl TIE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE0_A {
        match self.bits {
            false => TIE0_A::_0,
            true => TIE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE0_A::_1
    }
}
#[doc = "Write proxy for field `TIE0`"]
pub struct TIE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE0_A::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE0_A::_1)
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
#[doc = "Channel 1 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE1_A {
    #[doc = "0: Interrupt generation is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation is enabled"]
    _1 = 1,
}
impl From<TIE1_A> for bool {
    #[inline(always)]
    fn from(variant: TIE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE1`"]
pub type TIE1_R = crate::R<bool, TIE1_A>;
impl TIE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE1_A {
        match self.bits {
            false => TIE1_A::_0,
            true => TIE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE1_A::_1
    }
}
#[doc = "Write proxy for field `TIE1`"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE1_A::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Channel 2 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE2_A {
    #[doc = "0: Interrupt generation is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation is enabled"]
    _1 = 1,
}
impl From<TIE2_A> for bool {
    #[inline(always)]
    fn from(variant: TIE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE2`"]
pub type TIE2_R = crate::R<bool, TIE2_A>;
impl TIE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE2_A {
        match self.bits {
            false => TIE2_A::_0,
            true => TIE2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE2_A::_1
    }
}
#[doc = "Write proxy for field `TIE2`"]
pub struct TIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE2_A::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE2_A::_1)
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
#[doc = "Channel 3 Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE3_A {
    #[doc = "0: Interrupt generation is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt generation is enabled"]
    _1 = 1,
}
impl From<TIE3_A> for bool {
    #[inline(always)]
    fn from(variant: TIE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIE3`"]
pub type TIE3_R = crate::R<bool, TIE3_A>;
impl TIE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE3_A {
        match self.bits {
            false => TIE3_A::_0,
            true => TIE3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE3_A::_1
    }
}
#[doc = "Write proxy for field `TIE3`"]
pub struct TIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE3_A::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE3_A::_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie0(&mut self) -> TIE0_W {
        TIE0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie2(&mut self) -> TIE2_W {
        TIE2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie3(&mut self) -> TIE3_W {
        TIE3_W { w: self }
    }
}
