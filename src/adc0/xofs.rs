#[doc = "Reader of register XOFS"]
pub type R = crate::R<u32, super::XOFS>;
#[doc = "Writer for register XOFS"]
pub type W = crate::W<u32, super::XOFS>;
#[doc = "Register XOFS `reset()`'s with value 0x30"]
impl crate::ResetValue for super::XOFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Reader of field `XOFS`"]
pub type XOFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOFS`"]
pub struct XOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - X offset error correction value"]
    #[inline(always)]
    pub fn xofs(&self) -> XOFS_R {
        XOFS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - X offset error correction value"]
    #[inline(always)]
    pub fn xofs(&mut self) -> XOFS_W {
        XOFS_W { w: self }
    }
}
