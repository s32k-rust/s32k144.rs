#[doc = "Reader of register REGSC"]
pub type R = crate::R<u8, super::REGSC>;
#[doc = "Writer for register REGSC"]
pub type W = crate::W<u8, super::REGSC>;
#[doc = "Register REGSC `reset()`'s with value 0x04"]
impl crate::ResetValue for super::REGSC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Bias Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASEN_A {
    #[doc = "0: Biasing disabled, core logic can run in full performance"]
    _0 = 0,
    #[doc = "1: Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    _1 = 1,
}
impl From<BIASEN_A> for bool {
    #[inline(always)]
    fn from(variant: BIASEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BIASEN`"]
pub type BIASEN_R = crate::R<bool, BIASEN_A>;
impl BIASEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIASEN_A {
        match self.bits {
            false => BIASEN_A::_0,
            true => BIASEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BIASEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BIASEN_A::_1
    }
}
#[doc = "Write proxy for field `BIASEN`"]
pub struct BIASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIASEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Biasing disabled, core logic can run in full performance"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIASEN_A::_0)
    }
    #[doc = "Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIASEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Clock Bias Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKBIASDIS_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    _1 = 1,
}
impl From<CLKBIASDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CLKBIASDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKBIASDIS`"]
pub type CLKBIASDIS_R = crate::R<bool, CLKBIASDIS_A>;
impl CLKBIASDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKBIASDIS_A {
        match self.bits {
            false => CLKBIASDIS_A::_0,
            true => CLKBIASDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKBIASDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKBIASDIS_A::_1
    }
}
#[doc = "Write proxy for field `CLKBIASDIS`"]
pub struct CLKBIASDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKBIASDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKBIASDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKBIASDIS_A::_0)
    }
    #[doc = "In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKBIASDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Regulator in Full Performance Mode Status Bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGFPM_A {
    #[doc = "0: Regulator is in low power mode or transition to/from"]
    _0 = 0,
    #[doc = "1: Regulator is in full performance mode"]
    _1 = 1,
}
impl From<REGFPM_A> for bool {
    #[inline(always)]
    fn from(variant: REGFPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGFPM`"]
pub type REGFPM_R = crate::R<bool, REGFPM_A>;
impl REGFPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGFPM_A {
        match self.bits {
            false => REGFPM_A::_0,
            true => REGFPM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REGFPM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REGFPM_A::_1
    }
}
#[doc = "LPO Status Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSTAT_A {
    #[doc = "0: Low power oscillator in low phase"]
    _0 = 0,
    #[doc = "1: Low power oscillator in high phase"]
    _1 = 1,
}
impl From<LPOSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSTAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPOSTAT`"]
pub type LPOSTAT_R = crate::R<bool, LPOSTAT_A>;
impl LPOSTAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSTAT_A {
        match self.bits {
            false => LPOSTAT_A::_0,
            true => LPOSTAT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPOSTAT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPOSTAT_A::_1
    }
}
#[doc = "LPO Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPODIS_A {
    #[doc = "0: Low power oscillator enabled"]
    _0 = 0,
    #[doc = "1: Low power oscillator disabled"]
    _1 = 1,
}
impl From<LPODIS_A> for bool {
    #[inline(always)]
    fn from(variant: LPODIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPODIS`"]
pub type LPODIS_R = crate::R<bool, LPODIS_A>;
impl LPODIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPODIS_A {
        match self.bits {
            false => LPODIS_A::_0,
            true => LPODIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPODIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPODIS_A::_1
    }
}
#[doc = "Write proxy for field `LPODIS`"]
pub struct LPODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPODIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPODIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low power oscillator enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPODIS_A::_0)
    }
    #[doc = "Low power oscillator disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPODIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline(always)]
    pub fn biasen(&self) -> BIASEN_R {
        BIASEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline(always)]
    pub fn clkbiasdis(&self) -> CLKBIASDIS_R {
        CLKBIASDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Regulator in Full Performance Mode Status Bit"]
    #[inline(always)]
    pub fn regfpm(&self) -> REGFPM_R {
        REGFPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPO Status Bit"]
    #[inline(always)]
    pub fn lpostat(&self) -> LPOSTAT_R {
        LPOSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline(always)]
    pub fn lpodis(&self) -> LPODIS_R {
        LPODIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline(always)]
    pub fn biasen(&mut self) -> BIASEN_W {
        BIASEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline(always)]
    pub fn clkbiasdis(&mut self) -> CLKBIASDIS_W {
        CLKBIASDIS_W { w: self }
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline(always)]
    pub fn lpodis(&mut self) -> LPODIS_W {
        LPODIS_W { w: self }
    }
}
