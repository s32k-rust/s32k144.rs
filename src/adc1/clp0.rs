#[doc = "Reader of register CLP0"]
pub type R = crate::R<u32, super::CLP0>;
#[doc = "Writer for register CLP0"]
pub type W = crate::W<u32, super::CLP0>;
#[doc = "Register CLP0 `reset()`'s with value 0x2e"]
impl crate::ResetValue for super::CLP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2e
    }
}
#[doc = "Reader of field `CLP0`"]
pub type CLP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLP0`"]
pub struct CLP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clp0(&self) -> CLP0_R {
        CLP0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clp0(&mut self) -> CLP0_W {
        CLP0_W { w: self }
    }
}
