#[doc = "Reader of register EICHD0_WORD0"]
pub type R = crate::R<u32, super::EICHD0_WORD0>;
#[doc = "Writer for register EICHD0_WORD0"]
pub type W = crate::W<u32, super::EICHD0_WORD0>;
#[doc = "Register EICHD0_WORD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EICHD0_WORD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHKBIT_MASK`"]
pub type CHKBIT_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHKBIT_MASK`"]
pub struct CHKBIT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKBIT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - Checkbit Mask"]
    #[inline(always)]
    pub fn chkbit_mask(&self) -> CHKBIT_MASK_R {
        CHKBIT_MASK_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - Checkbit Mask"]
    #[inline(always)]
    pub fn chkbit_mask(&mut self) -> CHKBIT_MASK_W {
        CHKBIT_MASK_W { w: self }
    }
}
