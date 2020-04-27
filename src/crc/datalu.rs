#[doc = "Reader of register DATALU"]
pub type R = crate::R<u8, super::DATALU>;
#[doc = "Writer for register DATALU"]
pub type W = crate::W<u8, super::DATALU>;
#[doc = "Register DATALU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::DATALU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DATALU`"]
pub type DATALU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATALU`"]
pub struct DATALU_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datalu(&self) -> DATALU_R {
        DATALU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datalu(&mut self) -> DATALU_W {
        DATALU_W { w: self }
    }
}
