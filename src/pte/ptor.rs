#[doc = "Writer for register PTOR"]
pub type W = crate::W<u32, super::PTOR>;
#[doc = "Register PTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PTTO`"]
pub struct PTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto(&mut self) -> PTTO_W {
        PTTO_W { w: self }
    }
}
