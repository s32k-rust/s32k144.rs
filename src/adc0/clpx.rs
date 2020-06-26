#[doc = "Reader of register CLPX"]
pub type R = crate::R<u32, super::CLPX>;
#[doc = "Writer for register CLPX"]
pub type W = crate::W<u32, super::CLPX>;
#[doc = "Register CLPX `reset()`'s with value 0"]
impl crate::ResetValue for super::CLPX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLPX`"]
pub type CLPX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLPX`"]
pub struct CLPX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clpx(&self) -> CLPX_R {
        CLPX_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clpx(&mut self) -> CLPX_W {
        CLPX_W { w: self }
    }
}
