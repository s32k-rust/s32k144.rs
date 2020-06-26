#[doc = "Reader of register UG"]
pub type R = crate::R<u32, super::UG>;
#[doc = "Writer for register UG"]
pub type W = crate::W<u32, super::UG>;
#[doc = "Register UG `reset()`'s with value 0x04"]
impl crate::ResetValue for super::UG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `UG`"]
pub type UG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UG`"]
pub struct UG_W<'a> {
    w: &'a mut W,
}
impl<'a> UG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - User gain error correction value"]
    #[inline(always)]
    pub fn ug(&self) -> UG_R {
        UG_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - User gain error correction value"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W {
        UG_W { w: self }
    }
}
