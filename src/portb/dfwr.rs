#[doc = "Reader of register DFWR"]
pub type R = crate::R<u32, super::DFWR>;
#[doc = "Writer for register DFWR"]
pub type W = crate::W<u32, super::DFWR>;
#[doc = "Register DFWR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILT`"]
pub type FILT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FILT`"]
pub struct FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W { w: self }
    }
}
