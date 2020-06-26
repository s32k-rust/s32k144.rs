#[doc = "Reader of register TCD%s_CITER_ELINKNO"]
pub type R = crate::R<u16, super::TCD_CITER_ELINKNO>;
#[doc = "Writer for register TCD%s_CITER_ELINKNO"]
pub type W = crate::W<u16, super::TCD_CITER_ELINKNO>;
#[doc = "Register TCD%s_CITER_ELINKNO `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD_CITER_ELINKNO {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CITER`"]
pub type CITER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CITER`"]
pub struct CITER_W<'a> {
    w: &'a mut W,
}
impl<'a> CITER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u16) & 0x7fff);
        self.w
    }
}
#[doc = "Enable channel-to-channel linking on minor-loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled"]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    _1 = 1,
}
impl From<ELINK_A> for bool {
    #[inline(always)]
    fn from(variant: ELINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELINK`"]
pub type ELINK_R = crate::R<bool, ELINK_A>;
impl ELINK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELINK_A {
        match self.bits {
            false => ELINK_A::_0,
            true => ELINK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELINK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELINK_A::_1
    }
}
#[doc = "Write proxy for field `ELINK`"]
pub struct ELINK_W<'a> {
    w: &'a mut W,
}
impl<'a> ELINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ELINK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELINK_A::_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELINK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&self) -> CITER_R {
        CITER_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ELINK_R {
        ELINK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&mut self) -> CITER_W {
        CITER_W { w: self }
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&mut self) -> ELINK_W {
        ELINK_W { w: self }
    }
}
