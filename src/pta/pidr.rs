#[doc = "Reader of register PIDR"]
pub type R = crate::R<u32, super::PIDR>;
#[doc = "Writer for register PIDR"]
pub type W = crate::W<u32, super::PIDR>;
#[doc = "Register PIDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PIDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
