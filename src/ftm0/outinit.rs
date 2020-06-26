#[doc = "Reader of register OUTINIT"]
pub type R = crate::R<u32, super::OUTINIT>;
#[doc = "Writer for register OUTINIT"]
pub type W = crate::W<u32, super::OUTINIT>;
#[doc = "Register OUTINIT `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTINIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH0OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0OI`"]
pub type CH0OI_R = crate::R<bool, CH0OI_A>;
impl CH0OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OI_A {
        match self.bits {
            false => CH0OI_A::_0,
            true => CH0OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OI_A::_1
    }
}
#[doc = "Write proxy for field `CH0OI`"]
pub struct CH0OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OI_A::_1)
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
#[doc = "Channel 1 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH1OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1OI`"]
pub type CH1OI_R = crate::R<bool, CH1OI_A>;
impl CH1OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OI_A {
        match self.bits {
            false => CH1OI_A::_0,
            true => CH1OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OI_A::_1
    }
}
#[doc = "Write proxy for field `CH1OI`"]
pub struct CH1OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OI_A::_1)
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
#[doc = "Channel 2 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH2OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2OI`"]
pub type CH2OI_R = crate::R<bool, CH2OI_A>;
impl CH2OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OI_A {
        match self.bits {
            false => CH2OI_A::_0,
            true => CH2OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OI_A::_1
    }
}
#[doc = "Write proxy for field `CH2OI`"]
pub struct CH2OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OI_A::_1)
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
#[doc = "Channel 3 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH3OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3OI`"]
pub type CH3OI_R = crate::R<bool, CH3OI_A>;
impl CH3OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OI_A {
        match self.bits {
            false => CH3OI_A::_0,
            true => CH3OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OI_A::_1
    }
}
#[doc = "Write proxy for field `CH3OI`"]
pub struct CH3OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OI_A::_1)
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
#[doc = "Channel 4 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH4OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4OI`"]
pub type CH4OI_R = crate::R<bool, CH4OI_A>;
impl CH4OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OI_A {
        match self.bits {
            false => CH4OI_A::_0,
            true => CH4OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OI_A::_1
    }
}
#[doc = "Write proxy for field `CH4OI`"]
pub struct CH4OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OI_A::_1)
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
#[doc = "Channel 5 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH5OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5OI`"]
pub type CH5OI_R = crate::R<bool, CH5OI_A>;
impl CH5OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OI_A {
        match self.bits {
            false => CH5OI_A::_0,
            true => CH5OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OI_A::_1
    }
}
#[doc = "Write proxy for field `CH5OI`"]
pub struct CH5OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OI_A::_1)
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
#[doc = "Channel 6 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH6OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6OI`"]
pub type CH6OI_R = crate::R<bool, CH6OI_A>;
impl CH6OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OI_A {
        match self.bits {
            false => CH6OI_A::_0,
            true => CH6OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OI_A::_1
    }
}
#[doc = "Write proxy for field `CH6OI`"]
pub struct CH6OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OI_A::_1)
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
#[doc = "Channel 7 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OI_A {
    #[doc = "0: The initialization value is 0."]
    _0 = 0,
    #[doc = "1: The initialization value is 1."]
    _1 = 1,
}
impl From<CH7OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7OI`"]
pub type CH7OI_R = crate::R<bool, CH7OI_A>;
impl CH7OI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OI_A {
        match self.bits {
            false => CH7OI_A::_0,
            true => CH7OI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OI_A::_1
    }
}
#[doc = "Write proxy for field `CH7OI`"]
pub struct CH7OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OI_A::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OI_A::_1)
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
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&self) -> CH0OI_R {
        CH0OI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&self) -> CH1OI_R {
        CH1OI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&self) -> CH2OI_R {
        CH2OI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&self) -> CH3OI_R {
        CH3OI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&self) -> CH4OI_R {
        CH4OI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&self) -> CH5OI_R {
        CH5OI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&self) -> CH6OI_R {
        CH6OI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&self) -> CH7OI_R {
        CH7OI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&mut self) -> CH0OI_W {
        CH0OI_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&mut self) -> CH1OI_W {
        CH1OI_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&mut self) -> CH2OI_W {
        CH2OI_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&mut self) -> CH3OI_W {
        CH3OI_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&mut self) -> CH4OI_W {
        CH4OI_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&mut self) -> CH5OI_W {
        CH5OI_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&mut self) -> CH6OI_W {
        CH6OI_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&mut self) -> CH7OI_W {
        CH7OI_W { w: self }
    }
}
