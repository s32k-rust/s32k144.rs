#[doc = "Reader of register MOD"]
pub type R = crate::R<u32, super::MOD>;
#[doc = "Writer for register MOD"]
pub type W = crate::W<u32, super::MOD>;
#[doc = "Register MOD `reset()`'s with value 0"]
impl crate::ResetValue for super::MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MOD`"]
pub type MOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MOD`"]
pub struct MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MOD"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W { w: self }
    }
}
