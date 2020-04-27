#[doc = "Reader of register G"]
pub type R = crate::R<u32, super::G>;
#[doc = "Writer for register G"]
pub type W = crate::W<u32, super::G>;
#[doc = "Register G `reset()`'s with value 0x02f0"]
impl crate::ResetValue for super::G {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02f0
    }
}
#[doc = "Reader of field `G`"]
pub type G_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `G`"]
pub struct G_W<'a> {
    w: &'a mut W,
}
impl<'a> G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Gain error adjustment factor for the overall conversion"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain error adjustment factor for the overall conversion"]
    #[inline(always)]
    pub fn g(&mut self) -> G_W {
        G_W { w: self }
    }
}
