#[doc = "Reader of register DFER"]
pub type R = crate::R<u32, super::DFER>;
#[doc = "Writer for register DFER"]
pub type W = crate::W<u32, super::DFER>;
#[doc = "Register DFER `reset()`'s with value 0"]
impl crate::ResetValue for super::DFER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum DFE_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE_A> for u32 {
    #[inline(always)]
    fn from(variant: DFE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DFE`"]
pub type DFE_R = crate::R<u32, DFE_A>;
impl DFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, DFE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DFE_A::_0),
            1 => Val(DFE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE_A::_1
    }
}
#[doc = "Write proxy for field `DFE`"]
pub struct DFE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe(&self) -> DFE_R {
        DFE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe(&mut self) -> DFE_W {
        DFE_W { w: self }
    }
}
