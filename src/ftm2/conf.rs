#[doc = "Reader of register CONF"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LDFQ`"]
pub type LDFQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDFQ`"]
pub struct LDFQ_W<'a> {
    w: &'a mut W,
}
impl<'a> LDFQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `BDMMODE`"]
pub type BDMMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDMMODE`"]
pub struct BDMMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `GTBEEN`"]
pub type GTBEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTBEEN`"]
pub struct GTBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEEN_W<'a> {
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
#[doc = "Reader of field `GTBEOUT`"]
pub type GTBEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTBEOUT`"]
pub struct GTBEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GTBEOUT_W<'a> {
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
#[doc = "Initialization trigger on Reload Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITRIGR_A {
    #[doc = "0: Initialization trigger is generated on counter wrap events."]
    _0 = 0,
    #[doc = "1: Initialization trigger is generated when a reload point is reached."]
    _1 = 1,
}
impl From<ITRIGR_A> for bool {
    #[inline(always)]
    fn from(variant: ITRIGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ITRIGR`"]
pub type ITRIGR_R = crate::R<bool, ITRIGR_A>;
impl ITRIGR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITRIGR_A {
        match self.bits {
            false => ITRIGR_A::_0,
            true => ITRIGR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITRIGR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITRIGR_A::_1
    }
}
#[doc = "Write proxy for field `ITRIGR`"]
pub struct ITRIGR_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIGR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ITRIGR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Initialization trigger is generated on counter wrap events."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITRIGR_A::_0)
    }
    #[doc = "Initialization trigger is generated when a reload point is reached."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITRIGR_A::_1)
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
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline(always)]
    pub fn ldfq(&self) -> LDFQ_R {
        LDFQ_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&self) -> BDMMODE_R {
        BDMMODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&self) -> GTBEOUT_R {
        GTBEOUT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline(always)]
    pub fn itrigr(&self) -> ITRIGR_R {
        ITRIGR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline(always)]
    pub fn ldfq(&mut self) -> LDFQ_W {
        LDFQ_W { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn bdmmode(&mut self) -> BDMMODE_W {
        BDMMODE_W { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&mut self) -> GTBEEN_W {
        GTBEEN_W { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&mut self) -> GTBEOUT_W {
        GTBEOUT_W { w: self }
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline(always)]
    pub fn itrigr(&mut self) -> ITRIGR_W {
        ITRIGR_W { w: self }
    }
}
