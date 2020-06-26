#[doc = "Reader of register CLP2"]
pub type R = crate::R<u32, super::CLP2>;
#[doc = "Writer for register CLP2"]
pub type W = crate::W<u32, super::CLP2>;
#[doc = "Register CLP2 `reset()`'s with value 0xb8"]
impl crate::ResetValue for super::CLP2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb8
    }
}
#[doc = "Reader of field `CLP2`"]
pub type CLP2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLP2`"]
pub struct CLP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&self) -> CLP2_R {
        CLP2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&mut self) -> CLP2_W {
        CLP2_W { w: self }
    }
}
