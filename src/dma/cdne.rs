#[doc = "Writer for register CDNE"]
pub type W = crate::W<u8, super::CDNE>;
#[doc = "Register CDNE `reset()`'s with value 0"]
impl crate::ResetValue for super::CDNE {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CDNE`"]
pub struct CDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Clears All DONE Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CADN_AW {
    #[doc = "0: Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    _0 = 0,
    #[doc = "1: Clears all bits in TCDn_CSR\\[DONE\\]"]
    _1 = 1,
}
impl From<CADN_AW> for bool {
    #[inline(always)]
    fn from(variant: CADN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CADN`"]
pub struct CADN_W<'a> {
    w: &'a mut W,
}
impl<'a> CADN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CADN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CADN_AW::_0)
    }
    #[doc = "Clears all bits in TCDn_CSR\\[DONE\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CADN_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NOP`"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear DONE Bit"]
    #[inline(always)]
    pub fn cdne(&mut self) -> CDNE_W {
        CDNE_W { w: self }
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    pub fn cadn(&mut self) -> CADN_W {
        CADN_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
}
