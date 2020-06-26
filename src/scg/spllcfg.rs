#[doc = "Reader of register SPLLCFG"]
pub type R = crate::R<u32, super::SPLLCFG>;
#[doc = "Writer for register SPLLCFG"]
pub type W = crate::W<u32, super::SPLLCFG>;
#[doc = "Register SPLLCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SPLLCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PREDIV`"]
pub type PREDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PREDIV`"]
pub struct PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `MULT`"]
pub type MULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MULT`"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - PLL Reference Clock Divider"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - System PLL Multiplier"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - PLL Reference Clock Divider"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W {
        PREDIV_W { w: self }
    }
    #[doc = "Bits 16:20 - System PLL Multiplier"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
}
