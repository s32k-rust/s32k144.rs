#[doc = "Reader of register FLTPOL"]
pub type R = crate::R<u32, super::FLTPOL>;
#[doc = "Writer for register FLTPOL"]
pub type W = crate::W<u32, super::FLTPOL>;
#[doc = "Register FLTPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTPOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Input 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT0POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT0POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT0POL`"]
pub type FLT0POL_R = crate::R<bool, FLT0POL_A>;
impl FLT0POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT0POL_A {
        match self.bits {
            false => FLT0POL_A::_0,
            true => FLT0POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT0POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT0POL_A::_1
    }
}
#[doc = "Write proxy for field `FLT0POL`"]
pub struct FLT0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT0POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT0POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT0POL_A::_1)
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
#[doc = "Fault Input 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT1POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT1POL`"]
pub type FLT1POL_R = crate::R<bool, FLT1POL_A>;
impl FLT1POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT1POL_A {
        match self.bits {
            false => FLT1POL_A::_0,
            true => FLT1POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT1POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT1POL_A::_1
    }
}
#[doc = "Write proxy for field `FLT1POL`"]
pub struct FLT1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT1POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT1POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT1POL_A::_1)
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
#[doc = "Fault Input 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT2POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT2POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT2POL`"]
pub type FLT2POL_R = crate::R<bool, FLT2POL_A>;
impl FLT2POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT2POL_A {
        match self.bits {
            false => FLT2POL_A::_0,
            true => FLT2POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT2POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT2POL_A::_1
    }
}
#[doc = "Write proxy for field `FLT2POL`"]
pub struct FLT2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT2POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT2POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT2POL_A::_1)
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
#[doc = "Fault Input 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT3POL_A {
    #[doc = "0: The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    _0 = 0,
    #[doc = "1: The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    _1 = 1,
}
impl From<FLT3POL_A> for bool {
    #[inline(always)]
    fn from(variant: FLT3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT3POL`"]
pub type FLT3POL_R = crate::R<bool, FLT3POL_A>;
impl FLT3POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT3POL_A {
        match self.bits {
            false => FLT3POL_A::_0,
            true => FLT3POL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT3POL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT3POL_A::_1
    }
}
#[doc = "Write proxy for field `FLT3POL`"]
pub struct FLT3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT3POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT3POL_A::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT3POL_A::_1)
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
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&self) -> FLT0POL_R {
        FLT0POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&self) -> FLT1POL_R {
        FLT1POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&self) -> FLT2POL_R {
        FLT2POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&self) -> FLT3POL_R {
        FLT3POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline(always)]
    pub fn flt0pol(&mut self) -> FLT0POL_W {
        FLT0POL_W { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline(always)]
    pub fn flt1pol(&mut self) -> FLT1POL_W {
        FLT1POL_W { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline(always)]
    pub fn flt2pol(&mut self) -> FLT2POL_W {
        FLT2POL_W { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline(always)]
    pub fn flt3pol(&mut self) -> FLT3POL_W {
        FLT3POL_W { w: self }
    }
}
