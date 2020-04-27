#[doc = "Reader of register OUTMASK"]
pub type R = crate::R<u32, super::OUTMASK>;
#[doc = "Writer for register OUTMASK"]
pub type W = crate::W<u32, super::OUTMASK>;
#[doc = "Register OUTMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH0OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0OM`"]
pub type CH0OM_R = crate::R<bool, CH0OM_A>;
impl CH0OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OM_A {
        match self.bits {
            false => CH0OM_A::_0,
            true => CH0OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0OM_A::_1
    }
}
#[doc = "Write proxy for field `CH0OM`"]
pub struct CH0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OM_A::_1)
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
#[doc = "Channel 1 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH1OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1OM`"]
pub type CH1OM_R = crate::R<bool, CH1OM_A>;
impl CH1OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OM_A {
        match self.bits {
            false => CH1OM_A::_0,
            true => CH1OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1OM_A::_1
    }
}
#[doc = "Write proxy for field `CH1OM`"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OM_A::_1)
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
#[doc = "Channel 2 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH2OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2OM`"]
pub type CH2OM_R = crate::R<bool, CH2OM_A>;
impl CH2OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OM_A {
        match self.bits {
            false => CH2OM_A::_0,
            true => CH2OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2OM_A::_1
    }
}
#[doc = "Write proxy for field `CH2OM`"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OM_A::_1)
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
#[doc = "Channel 3 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH3OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3OM`"]
pub type CH3OM_R = crate::R<bool, CH3OM_A>;
impl CH3OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OM_A {
        match self.bits {
            false => CH3OM_A::_0,
            true => CH3OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3OM_A::_1
    }
}
#[doc = "Write proxy for field `CH3OM`"]
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OM_A::_1)
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
#[doc = "Channel 4 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH4OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4OM`"]
pub type CH4OM_R = crate::R<bool, CH4OM_A>;
impl CH4OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OM_A {
        match self.bits {
            false => CH4OM_A::_0,
            true => CH4OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4OM_A::_1
    }
}
#[doc = "Write proxy for field `CH4OM`"]
pub struct CH4OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OM_A::_1)
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
#[doc = "Channel 5 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH5OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5OM`"]
pub type CH5OM_R = crate::R<bool, CH5OM_A>;
impl CH5OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OM_A {
        match self.bits {
            false => CH5OM_A::_0,
            true => CH5OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5OM_A::_1
    }
}
#[doc = "Write proxy for field `CH5OM`"]
pub struct CH5OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OM_A::_1)
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
#[doc = "Channel 6 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH6OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6OM`"]
pub type CH6OM_R = crate::R<bool, CH6OM_A>;
impl CH6OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OM_A {
        match self.bits {
            false => CH6OM_A::_0,
            true => CH6OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6OM_A::_1
    }
}
#[doc = "Write proxy for field `CH6OM`"]
pub struct CH6OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OM_A::_1)
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
#[doc = "Channel 7 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH7OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7OM`"]
pub type CH7OM_R = crate::R<bool, CH7OM_A>;
impl CH7OM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OM_A {
        match self.bits {
            false => CH7OM_A::_0,
            true => CH7OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7OM_A::_1
    }
}
#[doc = "Write proxy for field `CH7OM`"]
pub struct CH7OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OM_A::_1)
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
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&self) -> CH0OM_R {
        CH0OM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&self) -> CH7OM_R {
        CH7OM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&mut self) -> CH0OM_W {
        CH0OM_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W {
        CH4OM_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W {
        CH5OM_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W {
        CH6OM_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&mut self) -> CH7OM_W {
        CH7OM_W { w: self }
    }
}
