#[doc = "Reader of register CFG2"]
pub type R = crate::R<u32, super::CFG2>;
#[doc = "Writer for register CFG2"]
pub type W = crate::W<u32, super::CFG2>;
#[doc = "Register CFG2 `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `SMPLTS`"]
pub type SMPLTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPLTS`"]
pub struct SMPLTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPLTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sample Time Select"]
    #[inline(always)]
    pub fn smplts(&self) -> SMPLTS_R {
        SMPLTS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sample Time Select"]
    #[inline(always)]
    pub fn smplts(&mut self) -> SMPLTS_W {
        SMPLTS_W { w: self }
    }
}
