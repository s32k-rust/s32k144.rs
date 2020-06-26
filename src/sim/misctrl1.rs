#[doc = "Reader of register MISCTRL1"]
pub type R = crate::R<u32, super::MISCTRL1>;
#[doc = "Writer for register MISCTRL1"]
pub type W = crate::W<u32, super::MISCTRL1>;
#[doc = "Register MISCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MISCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW_TRG`"]
pub type SW_TRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW_TRG`"]
pub struct SW_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TRG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
    #[inline(always)]
    pub fn sw_trg(&self) -> SW_TRG_R {
        SW_TRG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger to TRGMUX. Writing to this bit generates software trigger to peripherals through TRGMUX (Refer to Figure: Trigger interconnectivity)."]
    #[inline(always)]
    pub fn sw_trg(&mut self) -> SW_TRG_W {
        SW_TRG_W { w: self }
    }
}
