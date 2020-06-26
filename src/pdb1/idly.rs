#[doc = "Reader of register IDLY"]
pub type R = crate::R<u32, super::IDLY>;
#[doc = "Writer for register IDLY"]
pub type W = crate::W<u32, super::IDLY>;
#[doc = "Register IDLY `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::IDLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `IDLY`"]
pub type IDLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IDLY`"]
pub struct IDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&self) -> IDLY_R {
        IDLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&mut self) -> IDLY_W {
        IDLY_W { w: self }
    }
}
