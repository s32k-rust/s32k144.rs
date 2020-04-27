#[doc = "Reader of register TVAL0"]
pub type R = crate::R<u32, super::TVAL0>;
#[doc = "Writer for register TVAL0"]
pub type W = crate::W<u32, super::TVAL0>;
#[doc = "Register TVAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TVAL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TMR_VAL_A {
    #[doc = "0: Invalid load value in compare modes."]
    _0 = 0,
    #[doc = "1: Invalid load value in compare modes."]
    _1 = 1,
}
impl From<TMR_VAL_A> for u32 {
    #[inline(always)]
    fn from(variant: TMR_VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMR_VAL`"]
pub type TMR_VAL_R = crate::R<u32, TMR_VAL_A>;
impl TMR_VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TMR_VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMR_VAL_A::_0),
            1 => Val(TMR_VAL_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMR_VAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMR_VAL_A::_1
    }
}
#[doc = "Write proxy for field `TMR_VAL`"]
pub struct TMR_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Invalid load value in compare modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMR_VAL_A::_0)
    }
    #[doc = "Invalid load value in compare modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMR_VAL_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&self) -> TMR_VAL_R {
        TMR_VAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Value"]
    #[inline(always)]
    pub fn tmr_val(&mut self) -> TMR_VAL_W {
        TMR_VAL_W { w: self }
    }
}
