#[doc = "Reader of register CLP2_OFS"]
pub type R = crate::R<u32, super::CLP2_OFS>;
#[doc = "Writer for register CLP2_OFS"]
pub type W = crate::W<u32, super::CLP2_OFS>;
#[doc = "Register CLP2_OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::CLP2_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLP2_OFS`"]
pub type CLP2_OFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLP2_OFS`"]
pub struct CLP2_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP2_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLP2 Offset"]
    #[inline(always)]
    pub fn clp2_ofs(&self) -> CLP2_OFS_R {
        CLP2_OFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLP2 Offset"]
    #[inline(always)]
    pub fn clp2_ofs(&mut self) -> CLP2_OFS_W {
        CLP2_OFS_W { w: self }
    }
}
