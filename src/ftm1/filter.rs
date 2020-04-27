#[doc = "Reader of register FILTER"]
pub type R = crate::R<u32, super::FILTER>;
#[doc = "Writer for register FILTER"]
pub type W = crate::W<u32, super::FILTER>;
#[doc = "Register FILTER `reset()`'s with value 0"]
impl crate::ResetValue for super::FILTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0FVAL`"]
pub type CH0FVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0FVAL`"]
pub struct CH0FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CH1FVAL`"]
pub type CH1FVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1FVAL`"]
pub struct CH1FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH2FVAL`"]
pub type CH2FVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2FVAL`"]
pub struct CH2FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH3FVAL`"]
pub type CH3FVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3FVAL`"]
pub struct CH3FVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3FVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&self) -> CH0FVAL_R {
        CH0FVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&self) -> CH1FVAL_R {
        CH1FVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&self) -> CH2FVAL_R {
        CH2FVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&self) -> CH3FVAL_R {
        CH3FVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&mut self) -> CH0FVAL_W {
        CH0FVAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&mut self) -> CH1FVAL_W {
        CH1FVAL_W { w: self }
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&mut self) -> CH2FVAL_W {
        CH2FVAL_W { w: self }
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&mut self) -> CH3FVAL_W {
        CH3FVAL_W { w: self }
    }
}
