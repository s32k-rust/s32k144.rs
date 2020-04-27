#[doc = "Writer for register PSOR"]
pub type W = crate::W<u32, super::PSOR>;
#[doc = "Register PSOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PTSO`"]
pub struct PTSO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTSO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&mut self) -> PTSO_W {
        PTSO_W { w: self }
    }
}
