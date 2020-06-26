#[doc = "Reader of register ISFR"]
pub type R = crate::R<u32, super::ISFR>;
#[doc = "Writer for register ISFR"]
pub type W = crate::W<u32, super::ISFR>;
#[doc = "Register ISFR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum ISF_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF_A> for u32 {
    #[inline(always)]
    fn from(variant: ISF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISF`"]
pub type ISF_R = crate::R<u32, ISF_A>;
impl ISF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, ISF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ISF_A::_0),
            1 => Val(ISF_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF_A::_1
    }
}
#[doc = "Write proxy for field `ISF`"]
pub struct ISF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF_A::_0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&mut self) -> ISF_W {
        ISF_W { w: self }
    }
}
