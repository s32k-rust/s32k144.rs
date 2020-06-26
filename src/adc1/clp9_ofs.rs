#[doc = "Reader of register CLP9_OFS"]
pub type R = crate::R<u32, super::CLP9_OFS>;
#[doc = "Writer for register CLP9_OFS"]
pub type W = crate::W<u32, super::CLP9_OFS>;
#[doc = "Register CLP9_OFS `reset()`'s with value 0x0240"]
impl crate::ResetValue for super::CLP9_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0240
    }
}
#[doc = "Reader of field `CLP9_OFS`"]
pub type CLP9_OFS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CLP9_OFS`"]
pub struct CLP9_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP9_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - CLP9 Offset"]
    #[inline(always)]
    pub fn clp9_ofs(&self) -> CLP9_OFS_R {
        CLP9_OFS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - CLP9 Offset"]
    #[inline(always)]
    pub fn clp9_ofs(&mut self) -> CLP9_OFS_W {
        CLP9_OFS_W { w: self }
    }
}
