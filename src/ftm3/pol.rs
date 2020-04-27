#[doc = "Reader of register POL"]
pub type R = crate::R<u32, super::POL>;
#[doc = "Writer for register POL"]
pub type W = crate::W<u32, super::POL>;
#[doc = "Register POL `reset()`'s with value 0"]
impl crate::ResetValue for super::POL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL0`"]
pub type POL0_R = crate::R<bool, POL0_A>;
impl POL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL0_A {
        match self.bits {
            false => POL0_A::_0,
            true => POL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL0_A::_1
    }
}
#[doc = "Write proxy for field `POL0`"]
pub struct POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL0_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL0_A::_1)
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
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL1`"]
pub type POL1_R = crate::R<bool, POL1_A>;
impl POL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::_0,
            true => POL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL1_A::_1
    }
}
#[doc = "Write proxy for field `POL1`"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL1_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL1_A::_1)
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
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL2`"]
pub type POL2_R = crate::R<bool, POL2_A>;
impl POL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::_0,
            true => POL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL2_A::_1
    }
}
#[doc = "Write proxy for field `POL2`"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL2_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL2_A::_1)
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
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL3_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL3_A> for bool {
    #[inline(always)]
    fn from(variant: POL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL3`"]
pub type POL3_R = crate::R<bool, POL3_A>;
impl POL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL3_A {
        match self.bits {
            false => POL3_A::_0,
            true => POL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL3_A::_1
    }
}
#[doc = "Write proxy for field `POL3`"]
pub struct POL3_W<'a> {
    w: &'a mut W,
}
impl<'a> POL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL3_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL3_A::_1)
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
#[doc = "Channel 4 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL4_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL4_A> for bool {
    #[inline(always)]
    fn from(variant: POL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL4`"]
pub type POL4_R = crate::R<bool, POL4_A>;
impl POL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL4_A {
        match self.bits {
            false => POL4_A::_0,
            true => POL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL4_A::_1
    }
}
#[doc = "Write proxy for field `POL4`"]
pub struct POL4_W<'a> {
    w: &'a mut W,
}
impl<'a> POL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL4_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL4_A::_1)
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
#[doc = "Channel 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL5_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL5_A> for bool {
    #[inline(always)]
    fn from(variant: POL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL5`"]
pub type POL5_R = crate::R<bool, POL5_A>;
impl POL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL5_A {
        match self.bits {
            false => POL5_A::_0,
            true => POL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL5_A::_1
    }
}
#[doc = "Write proxy for field `POL5`"]
pub struct POL5_W<'a> {
    w: &'a mut W,
}
impl<'a> POL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL5_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL5_A::_1)
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
#[doc = "Channel 6 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL6_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL6_A> for bool {
    #[inline(always)]
    fn from(variant: POL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL6`"]
pub type POL6_R = crate::R<bool, POL6_A>;
impl POL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL6_A {
        match self.bits {
            false => POL6_A::_0,
            true => POL6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL6_A::_1
    }
}
#[doc = "Write proxy for field `POL6`"]
pub struct POL6_W<'a> {
    w: &'a mut W,
}
impl<'a> POL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL6_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL6_A::_1)
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
#[doc = "Channel 7 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL7_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL7_A> for bool {
    #[inline(always)]
    fn from(variant: POL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POL7`"]
pub type POL7_R = crate::R<bool, POL7_A>;
impl POL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL7_A {
        match self.bits {
            false => POL7_A::_0,
            true => POL7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL7_A::_1
    }
}
#[doc = "Write proxy for field `POL7`"]
pub struct POL7_W<'a> {
    w: &'a mut W,
}
impl<'a> POL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL7_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL7_A::_1)
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
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&self) -> POL4_R {
        POL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&self) -> POL5_R {
        POL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    pub fn pol6(&self) -> POL6_R {
        POL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    pub fn pol7(&self) -> POL7_R {
        POL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W {
        POL0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&mut self) -> POL3_W {
        POL3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&mut self) -> POL4_W {
        POL4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&mut self) -> POL5_W {
        POL5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    pub fn pol6(&mut self) -> POL6_W {
        POL6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    pub fn pol7(&mut self) -> POL7_W {
        POL7_W { w: self }
    }
}
