#[doc = "Reader of register CLKDIV4"]
pub type R = crate::R<u32, super::CLKDIV4>;
#[doc = "Writer for register CLKDIV4"]
pub type W = crate::W<u32, super::CLKDIV4>;
#[doc = "Register CLKDIV4 `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::CLKDIV4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `TRACEFRAC`"]
pub type TRACEFRAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACEFRAC`"]
pub struct TRACEFRAC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEFRAC_W<'a> {
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
#[doc = "Reader of field `TRACEDIV`"]
pub type TRACEDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACEDIV`"]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Debug Trace Divider control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACEDIVEN_A {
    #[doc = "0: Debug trace divider disabled"]
    _0 = 0,
    #[doc = "1: Debug trace divider enabled"]
    _1 = 1,
}
impl From<TRACEDIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACEDIVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRACEDIVEN`"]
pub type TRACEDIVEN_R = crate::R<bool, TRACEDIVEN_A>;
impl TRACEDIVEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEDIVEN_A {
        match self.bits {
            false => TRACEDIVEN_A::_0,
            true => TRACEDIVEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACEDIVEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACEDIVEN_A::_1
    }
}
#[doc = "Write proxy for field `TRACEDIVEN`"]
pub struct TRACEDIVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEDIVEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debug trace divider disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACEDIVEN_A::_0)
    }
    #[doc = "Debug trace divider enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACEDIVEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
    #[inline(always)]
    pub fn tracefrac(&self) -> TRACEFRAC_R {
        TRACEFRAC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Debug Trace Divider control"]
    #[inline(always)]
    pub fn tracediven(&self) -> TRACEDIVEN_R {
        TRACEDIVEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trace Clock Divider fraction To configure TRACEDIV and TRACEFRAC, you must first clear TRACEDIVEN to disable the trace clock divide function."]
    #[inline(always)]
    pub fn tracefrac(&mut self) -> TRACEFRAC_W {
        TRACEFRAC_W { w: self }
    }
    #[doc = "Bits 1:3 - Trace Clock Divider value To configure TRACEDIV, you must first disable TRACEDIVEN, then enable it after setting TRACEDIV."]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
    #[doc = "Bit 28 - Debug Trace Divider control"]
    #[inline(always)]
    pub fn tracediven(&mut self) -> TRACEDIVEN_W {
        TRACEDIVEN_W { w: self }
    }
}
