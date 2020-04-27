#[doc = "Reader of register LVDSC1"]
pub type R = crate::R<u8, super::LVDSC1>;
#[doc = "Writer for register LVDSC1"]
pub type W = crate::W<u8, super::LVDSC1>;
#[doc = "Register LVDSC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LVDSC1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low Voltage Detect Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRE_A {
    #[doc = "0: No system resets on low voltage detect events."]
    _0 = 0,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDRE`"]
pub type LVDRE_R = crate::R<bool, LVDRE_A>;
impl LVDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, LVDRE_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(LVDRE_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDRE_A::_0
    }
}
#[doc = "Write proxy for field `LVDRE`"]
pub struct LVDRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No system resets on low voltage detect events."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Low Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVDF = 1"]
    _1 = 1,
}
impl From<LVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDIE`"]
pub type LVDIE_R = crate::R<bool, LVDIE_A>;
impl LVDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDIE_A {
        match self.bits {
            false => LVDIE_A::_0,
            true => LVDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDIE_A::_1
    }
}
#[doc = "Write proxy for field `LVDIE`"]
pub struct LVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LVDACK`"]
pub struct LVDACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Low Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDF_A {
    #[doc = "0: Low-voltage event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage event detected"]
    _1 = 1,
}
impl From<LVDF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LVDF`"]
pub type LVDF_R = crate::R<bool, LVDF_A>;
impl LVDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDF_A {
        match self.bits {
            false => LVDF_A::_0,
            true => LVDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDF_A::_1
    }
}
impl R {
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&self) -> LVDIE_R {
        LVDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low Voltage Detect Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&mut self) -> LVDRE_W {
        LVDRE_W { w: self }
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&mut self) -> LVDIE_W {
        LVDIE_W { w: self }
    }
    #[doc = "Bit 6 - Low Voltage Detect Acknowledge"]
    #[inline(always)]
    pub fn lvdack(&mut self) -> LVDACK_W {
        LVDACK_W { w: self }
    }
}
