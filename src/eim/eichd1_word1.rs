#[doc = "Reader of register EICHD1_WORD1"]
pub type R = crate::R<u32, super::EICHD1_WORD1>;
#[doc = "Writer for register EICHD1_WORD1"]
pub type W = crate::W<u32, super::EICHD1_WORD1>;
#[doc = "Register EICHD1_WORD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EICHD1_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `B0_3DATA_MASK`"]
pub type B0_3DATA_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `B0_3DATA_MASK`"]
pub struct B0_3DATA_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_3DATA_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Mask Bytes 0-3"]
    #[inline(always)]
    pub fn b0_3data_mask(&self) -> B0_3DATA_MASK_R {
        B0_3DATA_MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Mask Bytes 0-3"]
    #[inline(always)]
    pub fn b0_3data_mask(&mut self) -> B0_3DATA_MASK_W {
        B0_3DATA_MASK_W { w: self }
    }
}
