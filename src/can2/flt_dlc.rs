#[doc = "Reader of register FLT_DLC"]
pub type R = crate::R<u32, super::FLT_DLC>;
#[doc = "Writer for register FLT_DLC"]
pub type W = crate::W<u32, super::FLT_DLC>;
#[doc = "Register FLT_DLC `reset()`'s with value 0x08"]
impl crate::ResetValue for super::FLT_DLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `FLT_DLC_HI`"]
pub type FLT_DLC_HI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT_DLC_HI`"]
pub struct FLT_DLC_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_DLC_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FLT_DLC_LO`"]
pub type FLT_DLC_LO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLT_DLC_LO`"]
pub struct FLT_DLC_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_DLC_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Upper Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_hi(&self) -> FLT_DLC_HI_R {
        FLT_DLC_HI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Lower Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_lo(&self) -> FLT_DLC_LO_R {
        FLT_DLC_LO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Upper Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_hi(&mut self) -> FLT_DLC_HI_W {
        FLT_DLC_HI_W { w: self }
    }
    #[doc = "Bits 16:19 - Lower Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_lo(&mut self) -> FLT_DLC_LO_W {
        FLT_DLC_LO_W { w: self }
    }
}
