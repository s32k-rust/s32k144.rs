#[doc = "Reader of register TCD5_CSR"]
pub type R = crate::R<u16, super::TCD5_CSR>;
#[doc = "Writer for register TCD5_CSR"]
pub type W = crate::W<u16, super::TCD5_CSR>;
#[doc = "Register TCD5_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCD5_CSR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: The channel is not explicitly started."]
    _0 = 0,
    #[doc = "1: The channel is explicitly started via a software initiated service request."]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel is not explicitly started."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_A::_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Enable an interrupt when major iteration count completes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMAJOR_A {
    #[doc = "0: The end-of-major loop interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The end-of-major loop interrupt is enabled."]
    _1 = 1,
}
impl From<INTMAJOR_A> for bool {
    #[inline(always)]
    fn from(variant: INTMAJOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTMAJOR`"]
pub type INTMAJOR_R = crate::R<bool, INTMAJOR_A>;
impl INTMAJOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMAJOR_A {
        match self.bits {
            false => INTMAJOR_A::_0,
            true => INTMAJOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTMAJOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTMAJOR_A::_1
    }
}
#[doc = "Write proxy for field `INTMAJOR`"]
pub struct INTMAJOR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTMAJOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTMAJOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The end-of-major loop interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTMAJOR_A::_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTMAJOR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTHALF_A {
    #[doc = "0: The half-point interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The half-point interrupt is enabled."]
    _1 = 1,
}
impl From<INTHALF_A> for bool {
    #[inline(always)]
    fn from(variant: INTHALF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTHALF`"]
pub type INTHALF_R = crate::R<bool, INTHALF_A>;
impl INTHALF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTHALF_A {
        match self.bits {
            false => INTHALF_A::_0,
            true => INTHALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTHALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTHALF_A::_1
    }
}
#[doc = "Write proxy for field `INTHALF`"]
pub struct INTHALF_W<'a> {
    w: &'a mut W,
}
impl<'a> INTHALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTHALF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The half-point interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTHALF_A::_0)
    }
    #[doc = "The half-point interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTHALF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `DREQ`"]
pub type DREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DREQ`"]
pub struct DREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESG_A {
    #[doc = "0: The current channel's TCD is normal format."]
    _0 = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    _1 = 1,
}
impl From<ESG_A> for bool {
    #[inline(always)]
    fn from(variant: ESG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESG`"]
pub type ESG_R = crate::R<bool, ESG_A>;
impl ESG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESG_A {
        match self.bits {
            false => ESG_A::_0,
            true => ESG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESG_A::_1
    }
}
#[doc = "Write proxy for field `ESG`"]
pub struct ESG_W<'a> {
    w: &'a mut W,
}
impl<'a> ESG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The current channel's TCD is normal format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESG_A::_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJORELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled."]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled."]
    _1 = 1,
}
impl From<MAJORELINK_A> for bool {
    #[inline(always)]
    fn from(variant: MAJORELINK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAJORELINK`"]
pub type MAJORELINK_R = crate::R<bool, MAJORELINK_A>;
impl MAJORELINK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAJORELINK_A {
        match self.bits {
            false => MAJORELINK_A::_0,
            true => MAJORELINK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAJORELINK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAJORELINK_A::_1
    }
}
#[doc = "Write proxy for field `MAJORELINK`"]
pub struct MAJORELINK_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJORELINK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAJORELINK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The channel-to-channel linking is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAJORELINK_A::_0)
    }
    #[doc = "The channel-to-channel linking is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAJORELINK_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ACTIVE`"]
pub type ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACTIVE`"]
pub struct ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `MAJORLINKCH`"]
pub type MAJORLINKCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAJORLINKCH`"]
pub struct MAJORLINKCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MAJORLINKCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u16) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BWC_A {
    #[doc = "0: No eDMA engine stalls."]
    _0 = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each R/W."]
    _10 = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each R/W."]
    _11 = 3,
}
impl From<BWC_A> for u8 {
    #[inline(always)]
    fn from(variant: BWC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BWC`"]
pub type BWC_R = crate::R<u8, BWC_A>;
impl BWC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BWC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BWC_A::_0),
            2 => Val(BWC_A::_10),
            3 => Val(BWC_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWC_A::_0
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BWC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BWC_A::_11
    }
}
#[doc = "Write proxy for field `BWC`"]
pub struct BWC_W<'a> {
    w: &'a mut W,
}
impl<'a> BWC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No eDMA engine stalls."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWC_A::_0)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BWC_A::_10)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BWC_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u16) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub fn intmajor(&self) -> INTMAJOR_R {
        INTMAJOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> INTHALF_R {
        INTHALF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> ESG_R {
        ESG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MAJORELINK_R {
        MAJORELINK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MAJORLINKCH_R {
        MAJORLINKCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BWC_R {
        BWC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub fn intmajor(&mut self) -> INTMAJOR_W {
        INTMAJOR_W { w: self }
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&mut self) -> INTHALF_W {
        INTHALF_W { w: self }
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W {
        DREQ_W { w: self }
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&mut self) -> ESG_W {
        ESG_W { w: self }
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&mut self) -> MAJORELINK_W {
        MAJORELINK_W { w: self }
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W {
        ACTIVE_W { w: self }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&mut self) -> MAJORLINKCH_W {
        MAJORLINKCH_W { w: self }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&mut self) -> BWC_W {
        BWC_W { w: self }
    }
}
