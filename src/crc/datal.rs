#[doc = "Reader of register DATAL"]
pub type R = crate::R<u16, super::DATAL>;
#[doc = "Writer for register DATAL"]
pub type W = crate::W<u16, super::DATAL>;
#[doc = "Register DATAL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::DATAL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DATAL`"]
pub type DATAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATAL`"]
pub struct DATAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn datal(&self) -> DATAL_R {
        DATAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn datal(&mut self) -> DATAL_W {
        DATAL_W { w: self }
    }
}
