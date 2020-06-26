#[doc = "Reader of register ISCR"]
pub type R = crate::R<u32, super::ISCR>;
#[doc = "Writer for register ISCR"]
pub type W = crate::W<u32, super::ISCR>;
#[doc = "Register ISCR `reset()`'s with value 0x0002_0000"]
impl crate::ResetValue for super::ISCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0000
    }
}
#[doc = "FPU Invalid Operation Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIOC_A> for bool {
    #[inline(always)]
    fn from(variant: FIOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIOC`"]
pub type FIOC_R = crate::R<bool, FIOC_A>;
impl FIOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOC_A {
        match self.bits {
            false => FIOC_A::_0,
            true => FIOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIOC_A::_1
    }
}
#[doc = "FPU Divide-by-Zero Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FDZC_A> for bool {
    #[inline(always)]
    fn from(variant: FDZC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDZC`"]
pub type FDZC_R = crate::R<bool, FDZC_A>;
impl FDZC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZC_A {
        match self.bits {
            false => FDZC_A::_0,
            true => FDZC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDZC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDZC_A::_1
    }
}
#[doc = "FPU Overflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FOFC_A> for bool {
    #[inline(always)]
    fn from(variant: FOFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FOFC`"]
pub type FOFC_R = crate::R<bool, FOFC_A>;
impl FOFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFC_A {
        match self.bits {
            false => FOFC_A::_0,
            true => FOFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FOFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FOFC_A::_1
    }
}
#[doc = "FPU Underflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FUFC_A> for bool {
    #[inline(always)]
    fn from(variant: FUFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FUFC`"]
pub type FUFC_R = crate::R<bool, FUFC_A>;
impl FUFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFC_A {
        match self.bits {
            false => FUFC_A::_0,
            true => FUFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUFC_A::_1
    }
}
#[doc = "FPU Inexact Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIXC_A> for bool {
    #[inline(always)]
    fn from(variant: FIXC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIXC`"]
pub type FIXC_R = crate::R<bool, FIXC_A>;
impl FIXC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXC_A {
        match self.bits {
            false => FIXC_A::_0,
            true => FIXC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXC_A::_1
    }
}
#[doc = "FPU Input Denormal Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIDC_A> for bool {
    #[inline(always)]
    fn from(variant: FIDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIDC`"]
pub type FIDC_R = crate::R<bool, FIDC_A>;
impl FIDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDC_A {
        match self.bits {
            false => FIDC_A::_0,
            true => FIDC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIDC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIDC_A::_1
    }
}
#[doc = "FPU Invalid Operation Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIOCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIOCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIOCE`"]
pub type FIOCE_R = crate::R<bool, FIOCE_A>;
impl FIOCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOCE_A {
        match self.bits {
            false => FIOCE_A::_0,
            true => FIOCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIOCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIOCE_A::_1
    }
}
#[doc = "Write proxy for field `FIOCE`"]
pub struct FIOCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIOCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIOCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIOCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIOCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "FPU Divide-by-Zero Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FDZCE_A> for bool {
    #[inline(always)]
    fn from(variant: FDZCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDZCE`"]
pub type FDZCE_R = crate::R<bool, FDZCE_A>;
impl FDZCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZCE_A {
        match self.bits {
            false => FDZCE_A::_0,
            true => FDZCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDZCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDZCE_A::_1
    }
}
#[doc = "Write proxy for field `FDZCE`"]
pub struct FDZCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDZCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDZCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDZCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDZCE_A::_1)
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
#[doc = "FPU Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FOFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FOFCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FOFCE`"]
pub type FOFCE_R = crate::R<bool, FOFCE_A>;
impl FOFCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFCE_A {
        match self.bits {
            false => FOFCE_A::_0,
            true => FOFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FOFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FOFCE_A::_1
    }
}
#[doc = "Write proxy for field `FOFCE`"]
pub struct FOFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FOFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FOFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FOFCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FOFCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "FPU Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FUFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FUFCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FUFCE`"]
pub type FUFCE_R = crate::R<bool, FUFCE_A>;
impl FUFCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFCE_A {
        match self.bits {
            false => FUFCE_A::_0,
            true => FUFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUFCE_A::_1
    }
}
#[doc = "Write proxy for field `FUFCE`"]
pub struct FUFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUFCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUFCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUFCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUFCE_A::_1)
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
#[doc = "FPU Inexact Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIXCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIXCE`"]
pub type FIXCE_R = crate::R<bool, FIXCE_A>;
impl FIXCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXCE_A {
        match self.bits {
            false => FIXCE_A::_0,
            true => FIXCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXCE_A::_1
    }
}
#[doc = "Write proxy for field `FIXCE`"]
pub struct FIXCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIXCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXCE_A::_1)
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
#[doc = "FPU Input Denormal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIDCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIDCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIDCE`"]
pub type FIDCE_R = crate::R<bool, FIDCE_A>;
impl FIDCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDCE_A {
        match self.bits {
            false => FIDCE_A::_0,
            true => FIDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIDCE_A::_1
    }
}
#[doc = "Write proxy for field `FIDCE`"]
pub struct FIDCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIDCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIDCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIDCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIDCE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - FPU Invalid Operation Interrupt Status"]
    #[inline(always)]
    pub fn fioc(&self) -> FIOC_R {
        FIOC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FPU Divide-by-Zero Interrupt Status"]
    #[inline(always)]
    pub fn fdzc(&self) -> FDZC_R {
        FDZC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FPU Overflow Interrupt Status"]
    #[inline(always)]
    pub fn fofc(&self) -> FOFC_R {
        FOFC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FPU Underflow Interrupt Status"]
    #[inline(always)]
    pub fn fufc(&self) -> FUFC_R {
        FUFC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FPU Inexact Interrupt Status"]
    #[inline(always)]
    pub fn fixc(&self) -> FIXC_R {
        FIXC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FPU Input Denormal Interrupt Status"]
    #[inline(always)]
    pub fn fidc(&self) -> FIDC_R {
        FIDC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    pub fn fioce(&self) -> FIOCE_R {
        FIOCE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub fn fdzce(&self) -> FDZCE_R {
        FDZCE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofce(&self) -> FOFCE_R {
        FOFCE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn fufce(&self) -> FUFCE_R {
        FUFCE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline(always)]
    pub fn fixce(&self) -> FIXCE_R {
        FIXCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    pub fn fidce(&self) -> FIDCE_R {
        FIDCE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    pub fn fioce(&mut self) -> FIOCE_W {
        FIOCE_W { w: self }
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub fn fdzce(&mut self) -> FDZCE_W {
        FDZCE_W { w: self }
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofce(&mut self) -> FOFCE_W {
        FOFCE_W { w: self }
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn fufce(&mut self) -> FUFCE_W {
        FUFCE_W { w: self }
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline(always)]
    pub fn fixce(&mut self) -> FIXCE_W {
        FIXCE_W { w: self }
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    pub fn fidce(&mut self) -> FIDCE_W {
        FIDCE_W { w: self }
    }
}
