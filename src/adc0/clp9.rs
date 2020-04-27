#[doc = "Reader of register CLP9"]
pub type R = crate::R<u32, super::CLP9>;
#[doc = "Writer for register CLP9"]
pub type W = crate::W<u32, super::CLP9>;
#[doc = "Register CLP9 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLP9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLP9`"]
pub type CLP9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLP9`"]
pub struct CLP9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP9_W<'a> {
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
    pub fn clp9(&self) -> CLP9_R {
        CLP9_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clp9(&mut self) -> CLP9_W {
        CLP9_W { w: self }
    }
}
