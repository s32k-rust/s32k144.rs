#[doc = "Reader of register PWMLOAD"]
pub type R = crate::R<u32, super::PWMLOAD>;
#[doc = "Writer for register PWMLOAD"]
pub type W = crate::W<u32, super::PWMLOAD>;
#[doc = "Register PWMLOAD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMLOAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0SEL`"]
pub type CH0SEL_R = crate::R<bool, CH0SEL_A>;
impl CH0SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0SEL_A {
        match self.bits {
            false => CH0SEL_A::_0,
            true => CH0SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH0SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH0SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH0SEL`"]
pub struct CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0SEL_A::_1)
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
#[doc = "Channel 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1SEL`"]
pub type CH1SEL_R = crate::R<bool, CH1SEL_A>;
impl CH1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1SEL_A {
        match self.bits {
            false => CH1SEL_A::_0,
            true => CH1SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH1SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH1SEL`"]
pub struct CH1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1SEL_A::_1)
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
#[doc = "Channel 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2SEL`"]
pub type CH2SEL_R = crate::R<bool, CH2SEL_A>;
impl CH2SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2SEL_A {
        match self.bits {
            false => CH2SEL_A::_0,
            true => CH2SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH2SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH2SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH2SEL`"]
pub struct CH2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2SEL_A::_1)
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
#[doc = "Channel 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH3SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3SEL`"]
pub type CH3SEL_R = crate::R<bool, CH3SEL_A>;
impl CH3SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3SEL_A {
        match self.bits {
            false => CH3SEL_A::_0,
            true => CH3SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH3SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH3SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH3SEL`"]
pub struct CH3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3SEL_A::_1)
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
#[doc = "Channel 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH4SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4SEL`"]
pub type CH4SEL_R = crate::R<bool, CH4SEL_A>;
impl CH4SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4SEL_A {
        match self.bits {
            false => CH4SEL_A::_0,
            true => CH4SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH4SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH4SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH4SEL`"]
pub struct CH4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4SEL_A::_1)
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
#[doc = "Channel 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH5SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5SEL`"]
pub type CH5SEL_R = crate::R<bool, CH5SEL_A>;
impl CH5SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5SEL_A {
        match self.bits {
            false => CH5SEL_A::_0,
            true => CH5SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH5SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH5SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH5SEL`"]
pub struct CH5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5SEL_A::_1)
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
#[doc = "Channel 6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH6SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6SEL`"]
pub type CH6SEL_R = crate::R<bool, CH6SEL_A>;
impl CH6SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6SEL_A {
        match self.bits {
            false => CH6SEL_A::_0,
            true => CH6SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH6SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH6SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH6SEL`"]
pub struct CH6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6SEL_A::_1)
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
#[doc = "Channel 7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    _1 = 1,
}
impl From<CH7SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7SEL`"]
pub type CH7SEL_R = crate::R<bool, CH7SEL_A>;
impl CH7SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7SEL_A {
        match self.bits {
            false => CH7SEL_A::_0,
            true => CH7SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CH7SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CH7SEL_A::_1
    }
}
#[doc = "Write proxy for field `CH7SEL`"]
pub struct CH7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7SEL_A::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7SEL_A::_1)
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
#[doc = "Half Cycle Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCSEL_A {
    #[doc = "0: Half cycle reload is disabled and it is not considered as a reload opportunity."]
    _0 = 0,
    #[doc = "1: Half cycle reload is enabled and it is considered as a reload opportunity."]
    _1 = 1,
}
impl From<HCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCSEL`"]
pub type HCSEL_R = crate::R<bool, HCSEL_A>;
impl HCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCSEL_A {
        match self.bits {
            false => HCSEL_A::_0,
            true => HCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCSEL_A::_1
    }
}
#[doc = "Write proxy for field `HCSEL`"]
pub struct HCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Half cycle reload is disabled and it is not considered as a reload opportunity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCSEL_A::_0)
    }
    #[doc = "Half cycle reload is enabled and it is considered as a reload opportunity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCSEL_A::_1)
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
#[doc = "Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOK_A {
    #[doc = "0: Loading updated values is disabled."]
    _0 = 0,
    #[doc = "1: Loading updated values is enabled."]
    _1 = 1,
}
impl From<LDOK_A> for bool {
    #[inline(always)]
    fn from(variant: LDOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDOK`"]
pub type LDOK_R = crate::R<bool, LDOK_A>;
impl LDOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOK_A {
        match self.bits {
            false => LDOK_A::_0,
            true => LDOK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDOK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDOK_A::_1
    }
}
#[doc = "Write proxy for field `LDOK`"]
pub struct LDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Loading updated values is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDOK_A::_0)
    }
    #[doc = "Loading updated values is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDOK_A::_1)
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
#[doc = "Global Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLEN_A {
    #[doc = "0: Global Load Ok disabled."]
    _0 = 0,
    #[doc = "1: Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    _1 = 1,
}
impl From<GLEN_A> for bool {
    #[inline(always)]
    fn from(variant: GLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GLEN`"]
pub type GLEN_R = crate::R<bool, GLEN_A>;
impl GLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLEN_A {
        match self.bits {
            false => GLEN_A::_0,
            true => GLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GLEN_A::_1
    }
}
#[doc = "Write proxy for field `GLEN`"]
pub struct GLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Global Load Ok disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GLEN_A::_0)
    }
    #[doc = "Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GLEN_A::_1)
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
#[doc = "Global Load OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLDOK_AW {
    #[doc = "0: No action."]
    _0 = 0,
    #[doc = "1: LDOK bit is set."]
    _1 = 1,
}
impl From<GLDOK_AW> for bool {
    #[inline(always)]
    fn from(variant: GLDOK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `GLDOK`"]
pub struct GLDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> GLDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLDOK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GLDOK_AW::_0)
    }
    #[doc = "LDOK bit is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GLDOK_AW::_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline(always)]
    pub fn hcsel(&self) -> HCSEL_R {
        HCSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline(always)]
    pub fn glen(&self) -> GLEN_R {
        GLEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> CH0SEL_W {
        CH0SEL_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> CH1SEL_W {
        CH1SEL_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> CH2SEL_W {
        CH2SEL_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> CH3SEL_W {
        CH3SEL_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> CH4SEL_W {
        CH4SEL_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> CH5SEL_W {
        CH5SEL_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> CH6SEL_W {
        CH6SEL_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&mut self) -> CH7SEL_W {
        CH7SEL_W { w: self }
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline(always)]
    pub fn hcsel(&mut self) -> HCSEL_W {
        HCSEL_W { w: self }
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&mut self) -> LDOK_W {
        LDOK_W { w: self }
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline(always)]
    pub fn glen(&mut self) -> GLEN_W {
        GLEN_W { w: self }
    }
    #[doc = "Bit 11 - Global Load OK"]
    #[inline(always)]
    pub fn gldok(&mut self) -> GLDOK_W {
        GLDOK_W { w: self }
    }
}
