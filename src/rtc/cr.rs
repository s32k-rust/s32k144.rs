#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR_A {
    #[doc = "0: No effect."]
    _0 = 0,
}
impl From<SWR_A> for bool {
    #[inline(always)]
    fn from(variant: SWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWR`"]
pub type SWR_R = crate::R<bool, SWR_A>;
impl SWR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SWR_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(SWR_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWR_A::_0
    }
}
#[doc = "Write proxy for field `SWR`"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWR_A::_0)
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
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUP_A {
    #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
    _0 = 0,
    #[doc = "1: Non-supervisor mode write accesses are supported."]
    _1 = 1,
}
impl From<SUP_A> for bool {
    #[inline(always)]
    fn from(variant: SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SUP`"]
pub type SUP_R = crate::R<bool, SUP_A>;
impl SUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUP_A {
        match self.bits {
            false => SUP_A::_0,
            true => SUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUP_A::_1
    }
}
#[doc = "Write proxy for field `SUP`"]
pub struct SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUP_A::_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUP_A::_1)
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
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UM_A {
    #[doc = "0: Registers cannot be written when locked."]
    _0 = 0,
    #[doc = "1: Registers can be written when locked under limited conditions."]
    _1 = 1,
}
impl From<UM_A> for bool {
    #[inline(always)]
    fn from(variant: UM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UM`"]
pub type UM_R = crate::R<bool, UM_A>;
impl UM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UM_A {
        match self.bits {
            false => UM_A::_0,
            true => UM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UM_A::_1
    }
}
#[doc = "Write proxy for field `UM`"]
pub struct UM_W<'a> {
    w: &'a mut W,
}
impl<'a> UM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UM_A::_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UM_A::_1)
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
#[doc = "Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPS_A {
    #[doc = "0: The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
    _0 = 0,
    #[doc = "1: The RTC 32kHz crystal clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    _1 = 1,
}
impl From<CPS_A> for bool {
    #[inline(always)]
    fn from(variant: CPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPS`"]
pub type CPS_R = crate::R<bool, CPS_A>;
impl CPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPS_A {
        match self.bits {
            false => CPS_A::_0,
            true => CPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPS_A::_1
    }
}
#[doc = "Write proxy for field `CPS`"]
pub struct CPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPS_A::_0)
    }
    #[doc = "The RTC 32kHz crystal clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPS_A::_1)
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
#[doc = "LPO Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOS_A {
    #[doc = "0: RTC prescaler increments using 32kHz crystal."]
    _0 = 0,
    #[doc = "1: RTC prescaler increments using 1kHz LPO, bits \\[4:0\\]
of the prescaler are ignored."]
    _1 = 1,
}
impl From<LPOS_A> for bool {
    #[inline(always)]
    fn from(variant: LPOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPOS`"]
pub type LPOS_R = crate::R<bool, LPOS_A>;
impl LPOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOS_A {
        match self.bits {
            false => LPOS_A::_0,
            true => LPOS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPOS_A::_1
    }
}
#[doc = "Write proxy for field `LPOS`"]
pub struct LPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC prescaler increments using 32kHz crystal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPOS_A::_0)
    }
    #[doc = "RTC prescaler increments using 1kHz LPO, bits \\[4:0\\]
of the prescaler are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPOS_A::_1)
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
#[doc = "Clock Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPE_A {
    #[doc = "0: Disable RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: Enable RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<CPE_A> for bool {
    #[inline(always)]
    fn from(variant: CPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPE`"]
pub type CPE_R = crate::R<bool, CPE_A>;
impl CPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPE_A {
        match self.bits {
            false => CPE_A::_0,
            true => CPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPE_A::_1
    }
}
#[doc = "Write proxy for field `CPE`"]
pub struct CPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPE_A::_0)
    }
    #[doc = "Enable RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&self) -> SUP_R {
        SUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&self) -> UM_R {
        UM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline(always)]
    pub fn cps(&self) -> CPS_R {
        CPS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline(always)]
    pub fn lpos(&self) -> LPOS_R {
        LPOS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&mut self) -> SUP_W {
        SUP_W { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&mut self) -> UM_W {
        UM_W { w: self }
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline(always)]
    pub fn cps(&mut self) -> CPS_W {
        CPS_W { w: self }
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline(always)]
    pub fn lpos(&mut self) -> LPOS_W {
        LPOS_W { w: self }
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline(always)]
    pub fn cpe(&mut self) -> CPE_W {
        CPE_W { w: self }
    }
}
