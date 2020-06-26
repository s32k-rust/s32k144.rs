#[doc = "Reader of register FCCOB3"]
pub type R = crate::R<u8, super::FCCOB3>;
#[doc = "Writer for register FCCOB3"]
pub type W = crate::W<u8, super::FCCOB3>;
#[doc = "Register FCCOB3 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCCOB3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCOBn`"]
pub type CCOBN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCOBn`"]
pub struct CCOBN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCOBN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CCOBn"]
    #[inline(always)]
    pub fn ccobn(&self) -> CCOBN_R {
        CCOBN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CCOBn"]
    #[inline(always)]
    pub fn ccobn(&mut self) -> CCOBN_W {
        CCOBN_W { w: self }
    }
}
