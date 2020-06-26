#[doc = "Reader of register DATAHU"]
pub type R = crate::R<u8, super::DATAHU>;
#[doc = "Writer for register DATAHU"]
pub type W = crate::W<u8, super::DATAHU>;
#[doc = "Register DATAHU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::DATAHU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DATAHU`"]
pub type DATAHU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAHU`"]
pub struct DATAHU_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahu(&self) -> DATAHU_R {
        DATAHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahu(&mut self) -> DATAHU_W {
        DATAHU_W { w: self }
    }
}
