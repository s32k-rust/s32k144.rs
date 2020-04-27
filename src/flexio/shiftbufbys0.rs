#[doc = "Reader of register SHIFTBUFBYS0"]
pub type R = crate::R<u32, super::SHIFTBUFBYS0>;
#[doc = "Writer for register SHIFTBUFBYS0"]
pub type W = crate::W<u32, super::SHIFTBUFBYS0>;
#[doc = "Register SHIFTBUFBYS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTBUFBYS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFTBUFBYS`"]
pub type SHIFTBUFBYS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SHIFTBUFBYS`"]
pub struct SHIFTBUFBYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTBUFBYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbys(&self) -> SHIFTBUFBYS_R {
        SHIFTBUFBYS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbys(&mut self) -> SHIFTBUFBYS_W {
        SHIFTBUFBYS_W { w: self }
    }
}
