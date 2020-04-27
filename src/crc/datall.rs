#[doc = "Reader of register DATALL"]
pub type R = crate::R<u8, super::DATALL>;
#[doc = "Writer for register DATALL"]
pub type W = crate::W<u8, super::DATALL>;
#[doc = "Register DATALL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::DATALL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `DATALL`"]
pub type DATALL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATALL`"]
pub struct DATALL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline(always)]
    pub fn datall(&self) -> DATALL_R {
        DATALL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline(always)]
    pub fn datall(&mut self) -> DATALL_W {
        DATALL_W { w: self }
    }
}
