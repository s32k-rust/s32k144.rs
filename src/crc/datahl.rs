#[doc = "Reader of register DATAHL"]
pub type R = crate::R<u8, super::DATAHL>;
#[doc = "Writer for register DATAHL"]
pub type W = crate::W<u8, super::DATAHL>;
#[doc = "Register DATAHL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::DATAHL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DATAHL`"]
pub type DATAHL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAHL`"]
pub struct DATAHL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAHL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahl(&self) -> DATAHL_R {
        DATAHL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datahl(&mut self) -> DATAHL_W {
        DATAHL_W { w: self }
    }
}
