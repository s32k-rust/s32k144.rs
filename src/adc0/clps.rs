#[doc = "Reader of register CLPS"]
pub type R = crate::R<u32, super::CLPS>;
#[doc = "Writer for register CLPS"]
pub type W = crate::W<u32, super::CLPS>;
#[doc = "Register CLPS `reset()`'s with value 0x2e"]
impl crate::ResetValue for super::CLPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2e
    }
}
#[doc = "Reader of field `CLPS`"]
pub type CLPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLPS`"]
pub struct CLPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPS_W<'a> {
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
    pub fn clps(&self) -> CLPS_R {
        CLPS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clps(&mut self) -> CLPS_W {
        CLPS_W { w: self }
    }
}
