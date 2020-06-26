#[doc = "Reader of register FEPROT"]
pub type R = crate::R<u8, super::FEPROT>;
#[doc = "Writer for register FEPROT"]
pub type W = crate::W<u8, super::FEPROT>;
#[doc = "Register FEPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEPROT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPROT`"]
pub type EPROT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPROT`"]
pub struct EPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> EPROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&mut self) -> EPROT_W {
        EPROT_W { w: self }
    }
}
