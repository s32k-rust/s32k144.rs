#[doc = "Reader of register CTRL2_PN"]
pub type R = crate::R<u32, super::CTRL2_PN>;
#[doc = "Writer for register CTRL2_PN"]
pub type W = crate::W<u32, super::CTRL2_PN>;
#[doc = "Register CTRL2_PN `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2_PN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCHTO`"]
pub type MATCHTO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCHTO`"]
pub struct MATCHTO_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timeout for No Message Matching the Filtering Criteria"]
    #[inline(always)]
    pub fn matchto(&self) -> MATCHTO_R {
        MATCHTO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout for No Message Matching the Filtering Criteria"]
    #[inline(always)]
    pub fn matchto(&mut self) -> MATCHTO_W {
        MATCHTO_W { w: self }
    }
}
