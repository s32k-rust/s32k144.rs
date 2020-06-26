#[doc = "Reader of register CLPX_OFS"]
pub type R = crate::R<u32, super::CLPX_OFS>;
#[doc = "Writer for register CLPX_OFS"]
pub type W = crate::W<u32, super::CLPX_OFS>;
#[doc = "Register CLPX_OFS `reset()`'s with value 0x0440"]
impl crate::ResetValue for super::CLPX_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0440
    }
}
#[doc = "Reader of field `CLPX_OFS`"]
pub type CLPX_OFS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLPX_OFS`"]
pub struct CLPX_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPX_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - CLPX Offset"]
    #[inline(always)]
    pub fn clpx_ofs(&self) -> CLPX_OFS_R {
        CLPX_OFS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - CLPX Offset"]
    #[inline(always)]
    pub fn clpx_ofs(&mut self) -> CLPX_OFS_W {
        CLPX_OFS_W { w: self }
    }
}
