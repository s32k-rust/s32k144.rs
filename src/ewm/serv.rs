#[doc = "Writer for register SERV"]
pub type W = crate::W<u8, super::SERV>;
#[doc = "Register SERV `reset()`'s with value 0"]
impl crate::ResetValue for super::SERV {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SERVICE`"]
pub struct SERVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SERVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - SERVICE"]
    #[inline(always)]
    pub fn service(&mut self) -> SERVICE_W {
        SERVICE_W { w: self }
    }
}
