#[doc = "Reader of register TPR"]
pub type R = crate::R<u32, super::TPR>;
#[doc = "Writer for register TPR"]
pub type W = crate::W<u32, super::TPR>;
#[doc = "Register TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPR`"]
pub type TPR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TPR`"]
pub struct TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time Prescaler Register"]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Prescaler Register"]
    #[inline(always)]
    pub fn tpr(&mut self) -> TPR_W {
        TPR_W { w: self }
    }
}
