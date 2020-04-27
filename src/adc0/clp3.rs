#[doc = "Reader of register CLP3"]
pub type R = crate::R<u32, super::CLP3>;
#[doc = "Writer for register CLP3"]
pub type W = crate::W<u32, super::CLP3>;
#[doc = "Register CLP3 `reset()`'s with value 0x0180"]
impl crate::ResetValue for super::CLP3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0180
    }
}
#[doc = "Reader of field `CLP3`"]
pub type CLP3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLP3`"]
pub struct CLP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP3_W<'a> {
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
    pub fn clp3(&self) -> CLP3_R {
        CLP3_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp3(&mut self) -> CLP3_W {
        CLP3_W { w: self }
    }
}
