#[doc = "Reader of register CLPS_OFS"]
pub type R = crate::R<u32, super::CLPS_OFS>;
#[doc = "Writer for register CLPS_OFS"]
pub type W = crate::W<u32, super::CLPS_OFS>;
#[doc = "Register CLPS_OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLPS_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLPS_OFS`"]
pub type CLPS_OFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLPS_OFS`"]
pub struct CLPS_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPS_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLPS Offset"]
    #[inline(always)]
    pub fn clps_ofs(&self) -> CLPS_OFS_R {
        CLPS_OFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLPS Offset"]
    #[inline(always)]
    pub fn clps_ofs(&mut self) -> CLPS_OFS_W {
        CLPS_OFS_W { w: self }
    }
}
