#[doc = "Reader of register BASE_OFS"]
pub type R = crate::R<u32, super::BASE_OFS>;
#[doc = "Writer for register BASE_OFS"]
pub type W = crate::W<u32, super::BASE_OFS>;
#[doc = "Register BASE_OFS `reset()`'s with value 0x40"]
impl crate::ResetValue for super::BASE_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `BA_OFS`"]
pub type BA_OFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BA_OFS`"]
pub struct BA_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Base Offset Error Correction Value"]
    #[inline(always)]
    pub fn ba_ofs(&self) -> BA_OFS_R {
        BA_OFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base Offset Error Correction Value"]
    #[inline(always)]
    pub fn ba_ofs(&mut self) -> BA_OFS_W {
        BA_OFS_W { w: self }
    }
}
