#[doc = "Reader of register MSR"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Writer for register MSR"]
pub type W = crate::W<u32, super::MSR>;
#[doc = "Register MSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF0_A {
    #[doc = "0: Timer has not timed out"]
    _0 = 0,
    #[doc = "1: Timeout has occurred"]
    _1 = 1,
}
impl From<TIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF0`"]
pub type TIF0_R = crate::R<bool, TIF0_A>;
impl TIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF0_A {
        match self.bits {
            false => TIF0_A::_0,
            true => TIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF0_A::_1
    }
}
#[doc = "Write proxy for field `TIF0`"]
pub struct TIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF0_A::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF0_A::_1)
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
#[doc = "Channel 1 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF1_A {
    #[doc = "0: Timer has not timed out"]
    _0 = 0,
    #[doc = "1: Timeout has occurred"]
    _1 = 1,
}
impl From<TIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF1`"]
pub type TIF1_R = crate::R<bool, TIF1_A>;
impl TIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF1_A {
        match self.bits {
            false => TIF1_A::_0,
            true => TIF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF1_A::_1
    }
}
#[doc = "Write proxy for field `TIF1`"]
pub struct TIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF1_A::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF1_A::_1)
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
#[doc = "Channel 2 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF2_A {
    #[doc = "0: Timer has not timed out"]
    _0 = 0,
    #[doc = "1: Timeout has occurred"]
    _1 = 1,
}
impl From<TIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF2`"]
pub type TIF2_R = crate::R<bool, TIF2_A>;
impl TIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF2_A {
        match self.bits {
            false => TIF2_A::_0,
            true => TIF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF2_A::_1
    }
}
#[doc = "Write proxy for field `TIF2`"]
pub struct TIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF2_A::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF2_A::_1)
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
#[doc = "Channel 3 Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF3_A {
    #[doc = "0: Timer has not timed out"]
    _0 = 0,
    #[doc = "1: Timeout has occurred"]
    _1 = 1,
}
impl From<TIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIF3`"]
pub type TIF3_R = crate::R<bool, TIF3_A>;
impl TIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF3_A {
        match self.bits {
            false => TIF3_A::_0,
            true => TIF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF3_A::_1
    }
}
#[doc = "Write proxy for field `TIF3`"]
pub struct TIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIF3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF3_A::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF3_A::_1)
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif0(&self) -> TIF0_R {
        TIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif1(&self) -> TIF1_R {
        TIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif2(&self) -> TIF2_R {
        TIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif3(&self) -> TIF3_R {
        TIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif0(&mut self) -> TIF0_W {
        TIF0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif1(&mut self) -> TIF1_W {
        TIF1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif2(&mut self) -> TIF2_W {
        TIF2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif3(&mut self) -> TIF3_W {
        TIF3_W { w: self }
    }
}
