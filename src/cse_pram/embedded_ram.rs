#[doc = "Reader of register EmbeddedRAM%s"]
pub type R = crate::R<u32, super::EMBEDDEDRAM>;
#[doc = "Writer for register EmbeddedRAM%s"]
pub type W = crate::W<u32, super::EMBEDDEDRAM>;
#[doc = "Register EmbeddedRAM%s `reset()`'s with value 0"]
impl crate::ResetValue for super::EMBEDDEDRAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYTE_3`"]
pub type BYTE_3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_3`"]
pub struct BYTE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BYTE_2`"]
pub type BYTE_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_2`"]
pub struct BYTE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYTE_1`"]
pub type BYTE_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_1`"]
pub struct BYTE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BYTE_0`"]
pub type BYTE_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTE_0`"]
pub struct BYTE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BYTE_3 stores the fourth 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_3(&self) -> BYTE_3_R {
        BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BYTE_2 stores the third 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_2(&self) -> BYTE_2_R {
        BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BYTE_1 stores the second 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_1(&self) -> BYTE_1_R {
        BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - BYTE_0 stores the first 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_0(&self) -> BYTE_0_R {
        BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BYTE_3 stores the fourth 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_3(&mut self) -> BYTE_3_W {
        BYTE_3_W { w: self }
    }
    #[doc = "Bits 8:15 - BYTE_2 stores the third 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_2(&mut self) -> BYTE_2_W {
        BYTE_2_W { w: self }
    }
    #[doc = "Bits 16:23 - BYTE_1 stores the second 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_1(&mut self) -> BYTE_1_W {
        BYTE_1_W { w: self }
    }
    #[doc = "Bits 24:31 - BYTE_0 stores the first 8 bits of the 32 bit CRC."]
    #[inline(always)]
    pub fn byte_0(&mut self) -> BYTE_0_W {
        BYTE_0_W { w: self }
    }
}
