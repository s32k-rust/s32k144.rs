#[doc = "Reader of register HCR"]
pub type R = crate::R<u32, super::HCR>;
#[doc = "Writer for register HCR"]
pub type W = crate::W<u32, super::HCR>;
#[doc = "Register HCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCVAL`"]
pub type HCVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HCVAL`"]
pub struct HCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Half Cycle Value"]
    #[inline(always)]
    pub fn hcval(&self) -> HCVAL_R {
        HCVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half Cycle Value"]
    #[inline(always)]
    pub fn hcval(&mut self) -> HCVAL_W {
        HCVAL_W { w: self }
    }
}
