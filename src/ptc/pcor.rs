#[doc = "Writer for register PCOR"]
pub type W = crate::W<u32, super::PCOR>;
#[doc = "Register PCOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PTCO`"]
pub struct PTCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco(&mut self) -> PTCO_W {
        PTCO_W { w: self }
    }
}
