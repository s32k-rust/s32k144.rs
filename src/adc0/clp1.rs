#[doc = "Reader of register CLP1"]
pub type R = crate::R<u32, super::CLP1>;
#[doc = "Writer for register CLP1"]
pub type W = crate::W<u32, super::CLP1>;
#[doc = "Register CLP1 `reset()`'s with value 0x5c"]
impl crate::ResetValue for super::CLP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5c
    }
}
#[doc = "Reader of field `CLP1`"]
pub type CLP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLP1`"]
pub struct CLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clp1(&self) -> CLP1_R {
        CLP1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clp1(&mut self) -> CLP1_W {
        CLP1_W { w: self }
    }
}
