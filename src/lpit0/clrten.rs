#[doc = "Reader of register CLRTEN"]
pub type R = crate::R<u32, super::CLRTEN>;
#[doc = "Writer for register CLRTEN"]
pub type W = crate::W<u32, super::CLRTEN>;
#[doc = "Register CLRTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Timer 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_0_AW {
    #[doc = "0: No action"]
    _0 = 0,
    #[doc = "1: Clear T_EN bit for Timer Channel 0"]
    _1 = 1,
}
impl From<CLR_T_EN_0_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_T_EN_0`"]
pub struct CLR_T_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_0_AW::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_0_AW::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Clear Timer 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_1_AW {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Clear T_EN bit for Timer Channel 1"]
    _1 = 1,
}
impl From<CLR_T_EN_1_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_T_EN_1`"]
pub struct CLR_T_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_1_AW::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_1_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear Timer 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_2_AW {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Clear T_EN bit for Timer Channel 2"]
    _1 = 1,
}
impl From<CLR_T_EN_2_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_T_EN_2`"]
pub struct CLR_T_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_2_AW::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_2_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear Timer 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_T_EN_3_AW {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Clear T_EN bit for Timer Channel 3"]
    _1 = 1,
}
impl From<CLR_T_EN_3_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_T_EN_3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_T_EN_3`"]
pub struct CLR_T_EN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_T_EN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_T_EN_3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_3_AW::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 3"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_3_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bit 0 - Clear Timer 0 Enable"]
    #[inline(always)]
    pub fn clr_t_en_0(&mut self) -> CLR_T_EN_0_W {
        CLR_T_EN_0_W { w: self }
    }
    #[doc = "Bit 1 - Clear Timer 1 Enable"]
    #[inline(always)]
    pub fn clr_t_en_1(&mut self) -> CLR_T_EN_1_W {
        CLR_T_EN_1_W { w: self }
    }
    #[doc = "Bit 2 - Clear Timer 2 Enable"]
    #[inline(always)]
    pub fn clr_t_en_2(&mut self) -> CLR_T_EN_2_W {
        CLR_T_EN_2_W { w: self }
    }
    #[doc = "Bit 3 - Clear Timer 3 Enable"]
    #[inline(always)]
    pub fn clr_t_en_3(&mut self) -> CLR_T_EN_3_W {
        CLR_T_EN_3_W { w: self }
    }
}
