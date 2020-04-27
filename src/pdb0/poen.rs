#[doc = "Reader of register POEN"]
pub type R = crate::R<u32, super::POEN>;
#[doc = "Writer for register POEN"]
pub type W = crate::W<u32, super::POEN>;
#[doc = "Register POEN `reset()`'s with value 0"]
impl crate::ResetValue for super::POEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POEN_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN_A> for u8 {
    #[inline(always)]
    fn from(variant: POEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `POEN`"]
pub type POEN_R = crate::R<u8, POEN_A>;
impl POEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, POEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(POEN_A::_0),
            1 => Val(POEN_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN_A::_1
    }
}
#[doc = "Write proxy for field `POEN`"]
pub struct POEN_W<'a> {
    w: &'a mut W,
}
impl<'a> POEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&mut self) -> POEN_W {
        POEN_W { w: self }
    }
}
