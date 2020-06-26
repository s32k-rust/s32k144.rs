#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `LL`"]
pub type LL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LL`"]
pub struct LL_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LU`"]
pub type LU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LU`"]
pub struct LU_W<'a> {
    w: &'a mut W,
}
impl<'a> LU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HL`"]
pub type HL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HL`"]
pub struct HL_W<'a> {
    w: &'a mut W,
}
impl<'a> HL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HU`"]
pub type HU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HU`"]
pub struct HU_W<'a> {
    w: &'a mut W,
}
impl<'a> HU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    pub fn ll(&self) -> LL_R {
        LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    pub fn lu(&self) -> LU_R {
        LU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    pub fn hl(&self) -> HL_R {
        HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC Low Lower Byte"]
    #[inline(always)]
    pub fn ll(&mut self) -> LL_W {
        LL_W { w: self }
    }
    #[doc = "Bits 8:15 - CRC Low Upper Byte"]
    #[inline(always)]
    pub fn lu(&mut self) -> LU_W {
        LU_W { w: self }
    }
    #[doc = "Bits 16:23 - CRC High Lower Byte"]
    #[inline(always)]
    pub fn hl(&mut self) -> HL_W {
        HL_W { w: self }
    }
    #[doc = "Bits 24:31 - CRC High Upper Byte"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W {
        HU_W { w: self }
    }
}
