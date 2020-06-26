#[doc = "Reader of register LPOTRIM"]
pub type R = crate::R<u8, super::LPOTRIM>;
#[doc = "Writer for register LPOTRIM"]
pub type W = crate::W<u8, super::LPOTRIM>;
#[doc = "Register LPOTRIM `reset()`'s with value 0"]
impl crate::ResetValue for super::LPOTRIM {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPOTRIM`"]
pub type LPOTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPOTRIM`"]
pub struct LPOTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - LPO trimming bits"]
    #[inline(always)]
    pub fn lpotrim(&self) -> LPOTRIM_R {
        LPOTRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LPO trimming bits"]
    #[inline(always)]
    pub fn lpotrim(&mut self) -> LPOTRIM_W {
        LPOTRIM_W { w: self }
    }
}
