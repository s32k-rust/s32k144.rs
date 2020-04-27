#[doc = "Reader of register SC3"]
pub type R = crate::R<u32, super::SC3>;
#[doc = "Writer for register SC3"]
pub type W = crate::W<u32, super::SC3>;
#[doc = "Register SC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: 4 samples averaged."]
    _00 = 0,
    #[doc = "1: 8 samples averaged."]
    _01 = 1,
    #[doc = "2: 16 samples averaged."]
    _10 = 2,
    #[doc = "3: 32 samples averaged."]
    _11 = 3,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AVGS`"]
pub type AVGS_R = crate::R<u8, AVGS_A>;
impl AVGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::_00,
            1 => AVGS_A::_01,
            2 => AVGS_A::_10,
            3 => AVGS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AVGS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AVGS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AVGS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AVGS_A::_11
    }
}
#[doc = "Write proxy for field `AVGS`"]
pub struct AVGS_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4 samples averaged."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AVGS_A::_00)
    }
    #[doc = "8 samples averaged."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AVGS_A::_01)
    }
    #[doc = "16 samples averaged."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AVGS_A::_10)
    }
    #[doc = "32 samples averaged."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AVGS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Hardware Average Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGE_A {
    #[doc = "0: Hardware average function disabled."]
    _0 = 0,
    #[doc = "1: Hardware average function enabled."]
    _1 = 1,
}
impl From<AVGE_A> for bool {
    #[inline(always)]
    fn from(variant: AVGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AVGE`"]
pub type AVGE_R = crate::R<bool, AVGE_A>;
impl AVGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGE_A {
        match self.bits {
            false => AVGE_A::_0,
            true => AVGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVGE_A::_1
    }
}
#[doc = "Write proxy for field `AVGE`"]
pub struct AVGE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware average function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVGE_A::_0)
    }
    #[doc = "Hardware average function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVGE_A::_1)
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
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCO_A {
    #[doc = "0: One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated."]
    _0 = 0,
    #[doc = "1: Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated."]
    _1 = 1,
}
impl From<ADCO_A> for bool {
    #[inline(always)]
    fn from(variant: ADCO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADCO`"]
pub type ADCO_R = crate::R<bool, ADCO_A>;
impl ADCO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCO_A {
        match self.bits {
            false => ADCO_A::_0,
            true => ADCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADCO_A::_1
    }
}
#[doc = "Write proxy for field `ADCO`"]
pub struct ADCO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCO_A::_0)
    }
    #[doc = "Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCO_A::_1)
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
#[doc = "Reader of field `CAL`"]
pub type CAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAL`"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
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
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    pub fn avge(&self) -> AVGE_R {
        AVGE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> ADCO_R {
        ADCO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&mut self) -> AVGS_W {
        AVGS_W { w: self }
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    pub fn avge(&mut self) -> AVGE_W {
        AVGE_W { w: self }
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&mut self) -> ADCO_W {
        ADCO_W { w: self }
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
}
