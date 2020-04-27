#[doc = "Reader of register OFS"]
pub type R = crate::R<u32, super::OFS>;
#[doc = "Writer for register OFS"]
pub type W = crate::W<u32, super::OFS>;
#[doc = "Register OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFS`"]
pub type OFS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFS`"]
pub struct OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W {
        OFS_W { w: self }
    }
}
