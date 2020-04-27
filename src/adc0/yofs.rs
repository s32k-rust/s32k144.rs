#[doc = "Reader of register YOFS"]
pub type R = crate::R<u32, super::YOFS>;
#[doc = "Writer for register YOFS"]
pub type W = crate::W<u32, super::YOFS>;
#[doc = "Register YOFS `reset()`'s with value 0x37"]
impl crate::ResetValue for super::YOFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x37
    }
}
#[doc = "Reader of field `YOFS`"]
pub type YOFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YOFS`"]
pub struct YOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> YOFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Y offset error correction value"]
    #[inline(always)]
    pub fn yofs(&self) -> YOFS_R {
        YOFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y offset error correction value"]
    #[inline(always)]
    pub fn yofs(&mut self) -> YOFS_W {
        YOFS_W { w: self }
    }
}
