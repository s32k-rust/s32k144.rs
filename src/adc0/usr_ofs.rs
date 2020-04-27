#[doc = "Reader of register USR_OFS"]
pub type R = crate::R<u32, super::USR_OFS>;
#[doc = "Writer for register USR_OFS"]
pub type W = crate::W<u32, super::USR_OFS>;
#[doc = "Register USR_OFS `reset()`'s with value 0"]
impl crate::ResetValue for super::USR_OFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USR_OFS`"]
pub type USR_OFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USR_OFS`"]
pub struct USR_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> USR_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - USER Offset Error Correction Value"]
    #[inline(always)]
    pub fn usr_ofs(&self) -> USR_OFS_R {
        USR_OFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - USER Offset Error Correction Value"]
    #[inline(always)]
    pub fn usr_ofs(&mut self) -> USR_OFS_W {
        USR_OFS_W { w: self }
    }
}
