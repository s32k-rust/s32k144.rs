#[doc = "Reader of register DLY1"]
pub type R = crate::R<u16, super::DLY1>;
#[doc = "Writer for register DLY1"]
pub type W = crate::W<u16, super::DLY1>;
#[doc = "Register DLY1 `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::DLY1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `DLY1`"]
pub type DLY1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DLY1`"]
pub struct DLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DLY1"]
    #[inline(always)]
    pub fn dly1(&self) -> DLY1_R {
        DLY1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DLY1"]
    #[inline(always)]
    pub fn dly1(&mut self) -> DLY1_W {
        DLY1_W { w: self }
    }
}
