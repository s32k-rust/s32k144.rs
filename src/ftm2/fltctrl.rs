#[doc = "Reader of register FLTCTRL"]
pub type R = crate::R<u32, super::FLTCTRL>;
#[doc = "Writer for register FLTCTRL"]
pub type W = crate::W<u32, super::FLTCTRL>;
#[doc = "Register FLTCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Input 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT0EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT0EN`"]
pub type FAULT0EN_R = crate::R<bool, FAULT0EN_A>;
impl FAULT0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT0EN_A {
        match self.bits {
            false => FAULT0EN_A::_0,
            true => FAULT0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT0EN_A::_1
    }
}
#[doc = "Write proxy for field `FAULT0EN`"]
pub struct FAULT0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_1)
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
#[doc = "Fault Input 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT1EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT1EN`"]
pub type FAULT1EN_R = crate::R<bool, FAULT1EN_A>;
impl FAULT1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT1EN_A {
        match self.bits {
            false => FAULT1EN_A::_0,
            true => FAULT1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT1EN_A::_1
    }
}
#[doc = "Write proxy for field `FAULT1EN`"]
pub struct FAULT1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_1)
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
#[doc = "Fault Input 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT2EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT2EN`"]
pub type FAULT2EN_R = crate::R<bool, FAULT2EN_A>;
impl FAULT2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT2EN_A {
        match self.bits {
            false => FAULT2EN_A::_0,
            true => FAULT2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT2EN_A::_1
    }
}
#[doc = "Write proxy for field `FAULT2EN`"]
pub struct FAULT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_1)
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
#[doc = "Fault Input 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT3EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULT3EN`"]
pub type FAULT3EN_R = crate::R<bool, FAULT3EN_A>;
impl FAULT3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT3EN_A {
        match self.bits {
            false => FAULT3EN_A::_0,
            true => FAULT3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULT3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULT3EN_A::_1
    }
}
#[doc = "Write proxy for field `FAULT3EN`"]
pub struct FAULT3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_1)
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
#[doc = "Fault Input 0 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR0EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FFLTR0EN`"]
pub type FFLTR0EN_R = crate::R<bool, FFLTR0EN_A>;
impl FFLTR0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR0EN_A {
        match self.bits {
            false => FFLTR0EN_A::_0,
            true => FFLTR0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR0EN_A::_1
    }
}
#[doc = "Write proxy for field `FFLTR0EN`"]
pub struct FFLTR0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_1)
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
#[doc = "Fault Input 1 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR1EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FFLTR1EN`"]
pub type FFLTR1EN_R = crate::R<bool, FFLTR1EN_A>;
impl FFLTR1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR1EN_A {
        match self.bits {
            false => FFLTR1EN_A::_0,
            true => FFLTR1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR1EN_A::_1
    }
}
#[doc = "Write proxy for field `FFLTR1EN`"]
pub struct FFLTR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_1)
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
#[doc = "Fault Input 2 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR2EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FFLTR2EN`"]
pub type FFLTR2EN_R = crate::R<bool, FFLTR2EN_A>;
impl FFLTR2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR2EN_A {
        match self.bits {
            false => FFLTR2EN_A::_0,
            true => FFLTR2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR2EN_A::_1
    }
}
#[doc = "Write proxy for field `FFLTR2EN`"]
pub struct FFLTR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_1)
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
#[doc = "Fault Input 3 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR3EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FFLTR3EN`"]
pub type FFLTR3EN_R = crate::R<bool, FFLTR3EN_A>;
impl FFLTR3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR3EN_A {
        match self.bits {
            false => FFLTR3EN_A::_0,
            true => FFLTR3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FFLTR3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FFLTR3EN_A::_1
    }
}
#[doc = "Write proxy for field `FFLTR3EN`"]
pub struct FFLTR3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_1)
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
#[doc = "Reader of field `FFVAL`"]
pub type FFVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FFVAL`"]
pub struct FFVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Fault output state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSTATE_A {
    #[doc = "0: FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    _0 = 0,
    #[doc = "1: FTM outputs will be tri-stated when fault event is ongoing"]
    _1 = 1,
}
impl From<FSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: FSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSTATE`"]
pub type FSTATE_R = crate::R<bool, FSTATE_A>;
impl FSTATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSTATE_A {
        match self.bits {
            false => FSTATE_A::_0,
            true => FSTATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSTATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSTATE_A::_1
    }
}
#[doc = "Write proxy for field `FSTATE`"]
pub struct FSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSTATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSTATE_A::_0)
    }
    #[doc = "FTM outputs will be tri-stated when fault event is ongoing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSTATE_A::_1)
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
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&self) -> FAULT0EN_R {
        FAULT0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&self) -> FAULT1EN_R {
        FAULT1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&self) -> FAULT2EN_R {
        FAULT2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&self) -> FAULT3EN_R {
        FAULT3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&self) -> FFLTR0EN_R {
        FFLTR0EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&self) -> FFLTR1EN_R {
        FFLTR1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&self) -> FFLTR2EN_R {
        FFLTR2EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&self) -> FFLTR3EN_R {
        FFLTR3EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&self) -> FFVAL_R {
        FFVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline(always)]
    pub fn fstate(&self) -> FSTATE_R {
        FSTATE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&mut self) -> FAULT0EN_W {
        FAULT0EN_W { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&mut self) -> FAULT1EN_W {
        FAULT1EN_W { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&mut self) -> FAULT2EN_W {
        FAULT2EN_W { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&mut self) -> FAULT3EN_W {
        FAULT3EN_W { w: self }
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&mut self) -> FFLTR0EN_W {
        FFLTR0EN_W { w: self }
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&mut self) -> FFLTR1EN_W {
        FFLTR1EN_W { w: self }
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&mut self) -> FFLTR2EN_W {
        FFLTR2EN_W { w: self }
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&mut self) -> FFLTR3EN_W {
        FFLTR3EN_W { w: self }
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&mut self) -> FFVAL_W {
        FFVAL_W { w: self }
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline(always)]
    pub fn fstate(&mut self) -> FSTATE_W {
        FSTATE_W { w: self }
    }
}
