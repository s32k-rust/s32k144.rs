#[doc = "Reader of register DATAH"]
pub type R = crate::R<u16, super::DATAH>;
#[doc = "Writer for register DATAH"]
pub type W = crate::W<u16, super::DATAH>;
#[doc = "Register DATAH `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::DATAH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DATAH`"]
pub type DATAH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATAH`"]
pub struct DATAH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn datah(&self) -> DATAH_R {
        DATAH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn datah(&mut self) -> DATAH_W {
        DATAH_W { w: self }
    }
}
