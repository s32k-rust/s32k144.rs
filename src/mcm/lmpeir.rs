#[doc = "Reader of register LMPEIR"]
pub type R = crate::R<u32, super::LMPEIR>;
#[doc = "Writer for register LMPEIR"]
pub type W = crate::W<u32, super::LMPEIR>;
#[doc = "Register LMPEIR `reset()`'s with value 0"]
impl crate::ResetValue for super::LMPEIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENC`"]
pub type ENC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ENC`"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `E1B`"]
pub type E1B_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `E1B`"]
pub struct E1B_W<'a> {
    w: &'a mut W,
}
impl<'a> E1B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Parity or ECC Error Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEELOC_A {
    #[doc = "0: Non-correctable ECC event from SRAM_L"]
    _00 = 0,
    #[doc = "1: Non-correctable ECC event from SRAM_U"]
    _01 = 1,
}
impl From<PEELOC_A> for u8 {
    #[inline(always)]
    fn from(variant: PEELOC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEELOC`"]
pub type PEELOC_R = crate::R<u8, PEELOC_A>;
impl PEELOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PEELOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PEELOC_A::_00),
            1 => Val(PEELOC_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PEELOC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PEELOC_A::_01
    }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - ENCn = ECC Noncorrectable Error n"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - E1Bn = ECC 1-bit Error n"]
    #[inline(always)]
    pub fn e1b(&self) -> E1B_R {
        E1B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Cache Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Parity or ECC Error Location"]
    #[inline(always)]
    pub fn peeloc(&self) -> PEELOC_R {
        PEELOC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Valid Bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ENCn = ECC Noncorrectable Error n"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bits 8:15 - E1Bn = ECC 1-bit Error n"]
    #[inline(always)]
    pub fn e1b(&mut self) -> E1B_W {
        E1B_W { w: self }
    }
    #[doc = "Bits 16:23 - Cache Parity Error"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
}
