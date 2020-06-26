#[doc = "Reader of register SWOCTRL"]
pub type R = crate::R<u32, super::SWOCTRL>;
#[doc = "Writer for register SWOCTRL"]
pub type W = crate::W<u32, super::SWOCTRL>;
#[doc = "Register SWOCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SWOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH0OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0OC`"]
pub type CH0OC_R = crate::R<bool, CH0OC_A>;
impl CH0OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OC_A {
        match self.bits {
            false => CH0OC_A::_0,
            true => CH0OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OC_A::_1
    }
}
#[doc = "Write proxy for field `CH0OC`"]
pub struct CH0OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OC_A::_1)
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
#[doc = "Channel 1 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH1OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1OC`"]
pub type CH1OC_R = crate::R<bool, CH1OC_A>;
impl CH1OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OC_A {
        match self.bits {
            false => CH1OC_A::_0,
            true => CH1OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OC_A::_1
    }
}
#[doc = "Write proxy for field `CH1OC`"]
pub struct CH1OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OC_A::_1)
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
#[doc = "Channel 2 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH2OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2OC`"]
pub type CH2OC_R = crate::R<bool, CH2OC_A>;
impl CH2OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OC_A {
        match self.bits {
            false => CH2OC_A::_0,
            true => CH2OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OC_A::_1
    }
}
#[doc = "Write proxy for field `CH2OC`"]
pub struct CH2OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OC_A::_1)
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
#[doc = "Channel 3 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH3OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3OC`"]
pub type CH3OC_R = crate::R<bool, CH3OC_A>;
impl CH3OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OC_A {
        match self.bits {
            false => CH3OC_A::_0,
            true => CH3OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OC_A::_1
    }
}
#[doc = "Write proxy for field `CH3OC`"]
pub struct CH3OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OC_A::_1)
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
#[doc = "Channel 4 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH4OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4OC`"]
pub type CH4OC_R = crate::R<bool, CH4OC_A>;
impl CH4OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OC_A {
        match self.bits {
            false => CH4OC_A::_0,
            true => CH4OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OC_A::_1
    }
}
#[doc = "Write proxy for field `CH4OC`"]
pub struct CH4OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OC_A::_1)
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
#[doc = "Channel 5 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH5OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5OC`"]
pub type CH5OC_R = crate::R<bool, CH5OC_A>;
impl CH5OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OC_A {
        match self.bits {
            false => CH5OC_A::_0,
            true => CH5OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OC_A::_1
    }
}
#[doc = "Write proxy for field `CH5OC`"]
pub struct CH5OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OC_A::_1)
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
#[doc = "Channel 6 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH6OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6OC`"]
pub type CH6OC_R = crate::R<bool, CH6OC_A>;
impl CH6OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OC_A {
        match self.bits {
            false => CH6OC_A::_0,
            true => CH6OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OC_A::_1
    }
}
#[doc = "Write proxy for field `CH6OC`"]
pub struct CH6OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OC_A::_1)
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
#[doc = "Channel 7 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    _0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    _1 = 1,
}
impl From<CH7OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7OC`"]
pub type CH7OC_R = crate::R<bool, CH7OC_A>;
impl CH7OC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OC_A {
        match self.bits {
            false => CH7OC_A::_0,
            true => CH7OC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OC_A::_1
    }
}
#[doc = "Write proxy for field `CH7OC`"]
pub struct CH7OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OC_A::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OC_A::_1)
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
#[doc = "Channel 0 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH0OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0OCV`"]
pub type CH0OCV_R = crate::R<bool, CH0OCV_A>;
impl CH0OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OCV_A {
        match self.bits {
            false => CH0OCV_A::_0,
            true => CH0OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH0OCV`"]
pub struct CH0OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Channel 1 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH1OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1OCV`"]
pub type CH1OCV_R = crate::R<bool, CH1OCV_A>;
impl CH1OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OCV_A {
        match self.bits {
            false => CH1OCV_A::_0,
            true => CH1OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH1OCV`"]
pub struct CH1OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Channel 2 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH2OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2OCV`"]
pub type CH2OCV_R = crate::R<bool, CH2OCV_A>;
impl CH2OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OCV_A {
        match self.bits {
            false => CH2OCV_A::_0,
            true => CH2OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH2OCV`"]
pub struct CH2OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Channel 3 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH3OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3OCV`"]
pub type CH3OCV_R = crate::R<bool, CH3OCV_A>;
impl CH3OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OCV_A {
        match self.bits {
            false => CH3OCV_A::_0,
            true => CH3OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH3OCV`"]
pub struct CH3OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Channel 4 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH4OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4OCV`"]
pub type CH4OCV_R = crate::R<bool, CH4OCV_A>;
impl CH4OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OCV_A {
        match self.bits {
            false => CH4OCV_A::_0,
            true => CH4OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH4OCV`"]
pub struct CH4OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Channel 5 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH5OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5OCV`"]
pub type CH5OCV_R = crate::R<bool, CH5OCV_A>;
impl CH5OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OCV_A {
        match self.bits {
            false => CH5OCV_A::_0,
            true => CH5OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH5OCV`"]
pub struct CH5OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Channel 6 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH6OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6OCV`"]
pub type CH6OCV_R = crate::R<bool, CH6OCV_A>;
impl CH6OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OCV_A {
        match self.bits {
            false => CH6OCV_A::_0,
            true => CH6OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH6OCV`"]
pub struct CH6OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Channel 7 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    _0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    _1 = 1,
}
impl From<CH7OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7OCV`"]
pub type CH7OCV_R = crate::R<bool, CH7OCV_A>;
impl CH7OCV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OCV_A {
        match self.bits {
            false => CH7OCV_A::_0,
            true => CH7OCV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OCV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OCV_A::_1
    }
}
#[doc = "Write proxy for field `CH7OCV`"]
pub struct CH7OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OCV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OCV_A::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OCV_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&self) -> CH0OC_R {
        CH0OC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&self) -> CH1OC_R {
        CH1OC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&self) -> CH2OC_R {
        CH2OC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&self) -> CH3OC_R {
        CH3OC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&self) -> CH4OC_R {
        CH4OC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&self) -> CH5OC_R {
        CH5OC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&self) -> CH6OC_R {
        CH6OC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&self) -> CH7OC_R {
        CH7OC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&self) -> CH0OCV_R {
        CH0OCV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&self) -> CH1OCV_R {
        CH1OCV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&self) -> CH2OCV_R {
        CH2OCV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&self) -> CH3OCV_R {
        CH3OCV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&self) -> CH4OCV_R {
        CH4OCV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&self) -> CH5OCV_R {
        CH5OCV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&self) -> CH6OCV_R {
        CH6OCV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&self) -> CH7OCV_R {
        CH7OCV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&mut self) -> CH0OC_W {
        CH0OC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&mut self) -> CH1OC_W {
        CH1OC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&mut self) -> CH2OC_W {
        CH2OC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&mut self) -> CH3OC_W {
        CH3OC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&mut self) -> CH4OC_W {
        CH4OC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&mut self) -> CH5OC_W {
        CH5OC_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&mut self) -> CH6OC_W {
        CH6OC_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&mut self) -> CH7OC_W {
        CH7OC_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&mut self) -> CH0OCV_W {
        CH0OCV_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&mut self) -> CH1OCV_W {
        CH1OCV_W { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&mut self) -> CH2OCV_W {
        CH2OCV_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&mut self) -> CH3OCV_W {
        CH3OCV_W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&mut self) -> CH4OCV_W {
        CH4OCV_W { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&mut self) -> CH5OCV_W {
        CH5OCV_W { w: self }
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&mut self) -> CH6OCV_W {
        CH6OCV_W { w: self }
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&mut self) -> CH7OCV_W {
        CH7OCV_W { w: self }
    }
}
