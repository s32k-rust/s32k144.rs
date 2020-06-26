#[doc = "Reader of register COMBINE"]
pub type R = crate::R<u32, super::COMBINE>;
#[doc = "Writer for register COMBINE"]
pub type W = crate::W<u32, super::COMBINE>;
#[doc = "Register COMBINE `reset()`'s with value 0"]
impl crate::ResetValue for super::COMBINE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMBINE0`"]
pub type COMBINE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMBINE0`"]
pub struct COMBINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE0_W<'a> {
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
#[doc = "Complement Of Channel (n) For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP0_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP0_A> for bool {
    #[inline(always)]
    fn from(variant: COMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP0`"]
pub type COMP0_R = crate::R<bool, COMP0_A>;
impl COMP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP0_A {
        match self.bits {
            false => COMP0_A::_0,
            true => COMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP0_A::_1
    }
}
#[doc = "Write proxy for field `COMP0`"]
pub struct COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP0_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP0_A::_1)
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
#[doc = "Reader of field `DECAPEN0`"]
pub type DECAPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECAPEN0`"]
pub struct DECAPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN0_W<'a> {
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
#[doc = "Dual Edge Capture Mode Captures For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP0_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP0_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECAP0`"]
pub type DECAP0_R = crate::R<bool, DECAP0_A>;
impl DECAP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP0_A {
        match self.bits {
            false => DECAP0_A::_0,
            true => DECAP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP0_A::_1
    }
}
#[doc = "Write proxy for field `DECAP0`"]
pub struct DECAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP0_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP0_A::_1)
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
#[doc = "Deadtime Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN0_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN0`"]
pub type DTEN0_R = crate::R<bool, DTEN0_A>;
impl DTEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::_0,
            true => DTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN0_A::_1
    }
}
#[doc = "Write proxy for field `DTEN0`"]
pub struct DTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN0_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN0_A::_1)
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
#[doc = "Synchronization Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN0_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCEN0`"]
pub type SYNCEN0_R = crate::R<bool, SYNCEN0_A>;
impl SYNCEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN0_A {
        match self.bits {
            false => SYNCEN0_A::_0,
            true => SYNCEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN0_A::_1
    }
}
#[doc = "Write proxy for field `SYNCEN0`"]
pub struct SYNCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_1)
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
#[doc = "Fault Control Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN0_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTEN0`"]
pub type FAULTEN0_R = crate::R<bool, FAULTEN0_A>;
impl FAULTEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN0_A {
        match self.bits {
            false => FAULTEN0_A::_0,
            true => FAULTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN0_A::_1
    }
}
#[doc = "Write proxy for field `FAULTEN0`"]
pub struct FAULTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_1)
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
#[doc = "Reader of field `MCOMBINE0`"]
pub type MCOMBINE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCOMBINE0`"]
pub struct MCOMBINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE0_W<'a> {
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
#[doc = "Reader of field `COMBINE1`"]
pub type COMBINE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMBINE1`"]
pub struct COMBINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE1_W<'a> {
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
#[doc = "Complement Of Channel (n) For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP1`"]
pub type COMP1_R = crate::R<bool, COMP1_A>;
impl COMP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1_A {
        match self.bits {
            false => COMP1_A::_0,
            true => COMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP1_A::_1
    }
}
#[doc = "Write proxy for field `COMP1`"]
pub struct COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP1_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP1_A::_1)
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
#[doc = "Reader of field `DECAPEN1`"]
pub type DECAPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECAPEN1`"]
pub struct DECAPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN1_W<'a> {
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
#[doc = "Dual Edge Capture Mode Captures For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP1_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP1_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECAP1`"]
pub type DECAP1_R = crate::R<bool, DECAP1_A>;
impl DECAP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP1_A {
        match self.bits {
            false => DECAP1_A::_0,
            true => DECAP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP1_A::_1
    }
}
#[doc = "Write proxy for field `DECAP1`"]
pub struct DECAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP1_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP1_A::_1)
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
#[doc = "Deadtime Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN1_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN1`"]
pub type DTEN1_R = crate::R<bool, DTEN1_A>;
impl DTEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN1_A {
        match self.bits {
            false => DTEN1_A::_0,
            true => DTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN1_A::_1
    }
}
#[doc = "Write proxy for field `DTEN1`"]
pub struct DTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN1_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN1_A::_1)
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
#[doc = "Synchronization Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN1_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCEN1`"]
pub type SYNCEN1_R = crate::R<bool, SYNCEN1_A>;
impl SYNCEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN1_A {
        match self.bits {
            false => SYNCEN1_A::_0,
            true => SYNCEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN1_A::_1
    }
}
#[doc = "Write proxy for field `SYNCEN1`"]
pub struct SYNCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_1)
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
#[doc = "Fault Control Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN1_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTEN1`"]
pub type FAULTEN1_R = crate::R<bool, FAULTEN1_A>;
impl FAULTEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN1_A {
        match self.bits {
            false => FAULTEN1_A::_0,
            true => FAULTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN1_A::_1
    }
}
#[doc = "Write proxy for field `FAULTEN1`"]
pub struct FAULTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_1)
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
#[doc = "Reader of field `MCOMBINE1`"]
pub type MCOMBINE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCOMBINE1`"]
pub struct MCOMBINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE1_W<'a> {
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
#[doc = "Reader of field `COMBINE2`"]
pub type COMBINE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMBINE2`"]
pub struct COMBINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Complement Of Channel (n) For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP2_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP2`"]
pub type COMP2_R = crate::R<bool, COMP2_A>;
impl COMP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2_A {
        match self.bits {
            false => COMP2_A::_0,
            true => COMP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP2_A::_1
    }
}
#[doc = "Write proxy for field `COMP2`"]
pub struct COMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP2_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DECAPEN2`"]
pub type DECAPEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECAPEN2`"]
pub struct DECAPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Dual Edge Capture Mode Captures For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP2_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP2_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECAP2`"]
pub type DECAP2_R = crate::R<bool, DECAP2_A>;
impl DECAP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP2_A {
        match self.bits {
            false => DECAP2_A::_0,
            true => DECAP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP2_A::_1
    }
}
#[doc = "Write proxy for field `DECAP2`"]
pub struct DECAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP2_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Deadtime Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN2_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN2`"]
pub type DTEN2_R = crate::R<bool, DTEN2_A>;
impl DTEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN2_A {
        match self.bits {
            false => DTEN2_A::_0,
            true => DTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN2_A::_1
    }
}
#[doc = "Write proxy for field `DTEN2`"]
pub struct DTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN2_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Synchronization Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN2_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCEN2`"]
pub type SYNCEN2_R = crate::R<bool, SYNCEN2_A>;
impl SYNCEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN2_A {
        match self.bits {
            false => SYNCEN2_A::_0,
            true => SYNCEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN2_A::_1
    }
}
#[doc = "Write proxy for field `SYNCEN2`"]
pub struct SYNCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Fault Control Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN2_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTEN2`"]
pub type FAULTEN2_R = crate::R<bool, FAULTEN2_A>;
impl FAULTEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN2_A {
        match self.bits {
            false => FAULTEN2_A::_0,
            true => FAULTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN2_A::_1
    }
}
#[doc = "Write proxy for field `FAULTEN2`"]
pub struct FAULTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `MCOMBINE2`"]
pub type MCOMBINE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCOMBINE2`"]
pub struct MCOMBINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `COMBINE3`"]
pub type COMBINE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMBINE3`"]
pub struct COMBINE3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Complement Of Channel (n) for n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP3_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP3_A> for bool {
    #[inline(always)]
    fn from(variant: COMP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMP3`"]
pub type COMP3_R = crate::R<bool, COMP3_A>;
impl COMP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP3_A {
        match self.bits {
            false => COMP3_A::_0,
            true => COMP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COMP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COMP3_A::_1
    }
}
#[doc = "Write proxy for field `COMP3`"]
pub struct COMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP3_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DECAPEN3`"]
pub type DECAPEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECAPEN3`"]
pub struct DECAPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Dual Edge Capture Mode Captures For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP3_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP3_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DECAP3`"]
pub type DECAP3_R = crate::R<bool, DECAP3_A>;
impl DECAP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP3_A {
        match self.bits {
            false => DECAP3_A::_0,
            true => DECAP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DECAP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DECAP3_A::_1
    }
}
#[doc = "Write proxy for field `DECAP3`"]
pub struct DECAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP3_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Deadtime Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN3_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN3`"]
pub type DTEN3_R = crate::R<bool, DTEN3_A>;
impl DTEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN3_A {
        match self.bits {
            false => DTEN3_A::_0,
            true => DTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEN3_A::_1
    }
}
#[doc = "Write proxy for field `DTEN3`"]
pub struct DTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN3_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Synchronization Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN3_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCEN3`"]
pub type SYNCEN3_R = crate::R<bool, SYNCEN3_A>;
impl SYNCEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN3_A {
        match self.bits {
            false => SYNCEN3_A::_0,
            true => SYNCEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN3_A::_1
    }
}
#[doc = "Write proxy for field `SYNCEN3`"]
pub struct SYNCEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Fault Control Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN3_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTEN3`"]
pub type FAULTEN3_R = crate::R<bool, FAULTEN3_A>;
impl FAULTEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN3_A {
        match self.bits {
            false => FAULTEN3_A::_0,
            true => FAULTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN3_A::_1
    }
}
#[doc = "Write proxy for field `FAULTEN3`"]
pub struct FAULTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MCOMBINE3`"]
pub type MCOMBINE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCOMBINE3`"]
pub struct MCOMBINE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    pub fn combine0(&self) -> COMBINE0_R {
        COMBINE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    pub fn decapen0(&self) -> DECAPEN0_R {
        DECAPEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    pub fn decap0(&self) -> DECAP0_R {
        DECAP0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    pub fn syncen0(&self) -> SYNCEN0_R {
        SYNCEN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    pub fn faulten0(&self) -> FAULTEN0_R {
        FAULTEN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline(always)]
    pub fn mcombine0(&self) -> MCOMBINE0_R {
        MCOMBINE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    pub fn combine1(&self) -> COMBINE1_R {
        COMBINE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    pub fn decapen1(&self) -> DECAPEN1_R {
        DECAPEN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    pub fn decap1(&self) -> DECAP1_R {
        DECAP1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    pub fn syncen1(&self) -> SYNCEN1_R {
        SYNCEN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    pub fn faulten1(&self) -> FAULTEN1_R {
        FAULTEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline(always)]
    pub fn mcombine1(&self) -> MCOMBINE1_R {
        MCOMBINE1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    pub fn combine2(&self) -> COMBINE2_R {
        COMBINE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    pub fn decapen2(&self) -> DECAPEN2_R {
        DECAPEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    pub fn decap2(&self) -> DECAP2_R {
        DECAP2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    pub fn dten2(&self) -> DTEN2_R {
        DTEN2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    pub fn syncen2(&self) -> SYNCEN2_R {
        SYNCEN2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    pub fn faulten2(&self) -> FAULTEN2_R {
        FAULTEN2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline(always)]
    pub fn mcombine2(&self) -> MCOMBINE2_R {
        MCOMBINE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    pub fn combine3(&self) -> COMBINE3_R {
        COMBINE3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    pub fn comp3(&self) -> COMP3_R {
        COMP3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    pub fn decapen3(&self) -> DECAPEN3_R {
        DECAPEN3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    pub fn decap3(&self) -> DECAP3_R {
        DECAP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    pub fn dten3(&self) -> DTEN3_R {
        DTEN3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    pub fn syncen3(&self) -> SYNCEN3_R {
        SYNCEN3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    pub fn faulten3(&self) -> FAULTEN3_R {
        FAULTEN3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline(always)]
    pub fn mcombine3(&self) -> MCOMBINE3_R {
        MCOMBINE3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    pub fn combine0(&mut self) -> COMBINE0_W {
        COMBINE0_W { w: self }
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W { w: self }
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    pub fn decapen0(&mut self) -> DECAPEN0_W {
        DECAPEN0_W { w: self }
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    pub fn decap0(&mut self) -> DECAP0_W {
        DECAP0_W { w: self }
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    pub fn dten0(&mut self) -> DTEN0_W {
        DTEN0_W { w: self }
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    pub fn syncen0(&mut self) -> SYNCEN0_W {
        SYNCEN0_W { w: self }
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    pub fn faulten0(&mut self) -> FAULTEN0_W {
        FAULTEN0_W { w: self }
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline(always)]
    pub fn mcombine0(&mut self) -> MCOMBINE0_W {
        MCOMBINE0_W { w: self }
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    pub fn combine1(&mut self) -> COMBINE1_W {
        COMBINE1_W { w: self }
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W { w: self }
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    pub fn decapen1(&mut self) -> DECAPEN1_W {
        DECAPEN1_W { w: self }
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    pub fn decap1(&mut self) -> DECAP1_W {
        DECAP1_W { w: self }
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    pub fn dten1(&mut self) -> DTEN1_W {
        DTEN1_W { w: self }
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    pub fn syncen1(&mut self) -> SYNCEN1_W {
        SYNCEN1_W { w: self }
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    pub fn faulten1(&mut self) -> FAULTEN1_W {
        FAULTEN1_W { w: self }
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline(always)]
    pub fn mcombine1(&mut self) -> MCOMBINE1_W {
        MCOMBINE1_W { w: self }
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    pub fn combine2(&mut self) -> COMBINE2_W {
        COMBINE2_W { w: self }
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    pub fn comp2(&mut self) -> COMP2_W {
        COMP2_W { w: self }
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    pub fn decapen2(&mut self) -> DECAPEN2_W {
        DECAPEN2_W { w: self }
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    pub fn decap2(&mut self) -> DECAP2_W {
        DECAP2_W { w: self }
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    pub fn dten2(&mut self) -> DTEN2_W {
        DTEN2_W { w: self }
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    pub fn syncen2(&mut self) -> SYNCEN2_W {
        SYNCEN2_W { w: self }
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    pub fn faulten2(&mut self) -> FAULTEN2_W {
        FAULTEN2_W { w: self }
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline(always)]
    pub fn mcombine2(&mut self) -> MCOMBINE2_W {
        MCOMBINE2_W { w: self }
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    pub fn combine3(&mut self) -> COMBINE3_W {
        COMBINE3_W { w: self }
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    pub fn comp3(&mut self) -> COMP3_W {
        COMP3_W { w: self }
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    pub fn decapen3(&mut self) -> DECAPEN3_W {
        DECAPEN3_W { w: self }
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    pub fn decap3(&mut self) -> DECAP3_W {
        DECAP3_W { w: self }
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    pub fn dten3(&mut self) -> DTEN3_W {
        DTEN3_W { w: self }
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    pub fn syncen3(&mut self) -> SYNCEN3_W {
        SYNCEN3_W { w: self }
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    pub fn faulten3(&mut self) -> FAULTEN3_W {
        FAULTEN3_W { w: self }
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline(always)]
    pub fn mcombine3(&mut self) -> MCOMBINE3_W {
        MCOMBINE3_W { w: self }
    }
}
