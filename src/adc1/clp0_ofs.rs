#[doc = "Reader of register CLP0_OFS"]
pub type R = crate::R<u32, super::CLP0_OFS>;
#[doc = "Writer for register CLP0_OFS"]
pub type W = crate::W<u32, super::CLP0_OFS>;
#[doc = "Register CLP0_OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLP0_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLP0_OFS`"]
pub type CLP0_OFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLP0_OFS`"]
pub struct CLP0_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP0_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLP0 Offset"]
    #[inline(always)]
    pub fn clp0_ofs(&self) -> CLP0_OFS_R {
        CLP0_OFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLP0 Offset"]
    #[inline(always)]
    pub fn clp0_ofs(&mut self) -> CLP0_OFS_W {
        CLP0_OFS_W { w: self }
    }
}
