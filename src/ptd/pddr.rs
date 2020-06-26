#[doc = "Reader of register PDDR"]
pub type R = crate::R<u32, super::PDDR>;
#[doc = "Writer for register PDDR"]
pub type W = crate::W<u32, super::PDDR>;
#[doc = "Register PDDR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDD`"]
pub type PDD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PDD`"]
pub struct PDD_W<'a> {
    w: &'a mut W,
}
impl<'a> PDD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&self) -> PDD_R {
        PDD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&mut self) -> PDD_W {
        PDD_W { w: self }
    }
}
