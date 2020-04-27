#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Minimum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMIN_A {
    #[doc = "0: The minimum loading point is disabled."]
    _0 = 0,
    #[doc = "1: The minimum loading point is enabled."]
    _1 = 1,
}
impl From<CNTMIN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTMIN`"]
pub type CNTMIN_R = crate::R<bool, CNTMIN_A>;
impl CNTMIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMIN_A {
        match self.bits {
            false => CNTMIN_A::_0,
            true => CNTMIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMIN_A::_1
    }
}
#[doc = "Write proxy for field `CNTMIN`"]
pub struct CNTMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The minimum loading point is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMIN_A::_0)
    }
    #[doc = "The minimum loading point is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMIN_A::_1)
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
#[doc = "Maximum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAX_A {
    #[doc = "0: The maximum loading point is disabled."]
    _0 = 0,
    #[doc = "1: The maximum loading point is enabled."]
    _1 = 1,
}
impl From<CNTMAX_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CNTMAX`"]
pub type CNTMAX_R = crate::R<bool, CNTMAX_A>;
impl CNTMAX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX_A {
        match self.bits {
            false => CNTMAX_A::_0,
            true => CNTMAX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMAX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMAX_A::_1
    }
}
#[doc = "Write proxy for field `CNTMAX`"]
pub struct CNTMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The maximum loading point is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAX_A::_0)
    }
    #[doc = "The maximum loading point is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAX_A::_1)
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
#[doc = "FTM Counter Reinitialization by Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REINIT_A {
    #[doc = "0: FTM counter continues to count normally."]
    _0 = 0,
    #[doc = "1: FTM counter is updated with its initial value when the selected trigger is detected."]
    _1 = 1,
}
impl From<REINIT_A> for bool {
    #[inline(always)]
    fn from(variant: REINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REINIT`"]
pub type REINIT_R = crate::R<bool, REINIT_A>;
impl REINIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REINIT_A {
        match self.bits {
            false => REINIT_A::_0,
            true => REINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REINIT_A::_1
    }
}
#[doc = "Write proxy for field `REINIT`"]
pub struct REINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> REINIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REINIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM counter continues to count normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REINIT_A::_0)
    }
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REINIT_A::_1)
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
#[doc = "Output Mask Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHOM_A {
    #[doc = "0: OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    _0 = 0,
    #[doc = "1: OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    _1 = 1,
}
impl From<SYNCHOM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCHOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCHOM`"]
pub type SYNCHOM_R = crate::R<bool, SYNCHOM_A>;
impl SYNCHOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCHOM_A {
        match self.bits {
            false => SYNCHOM_A::_0,
            true => SYNCHOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCHOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCHOM_A::_1
    }
}
#[doc = "Write proxy for field `SYNCHOM`"]
pub struct SYNCHOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCHOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCHOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCHOM_A::_0)
    }
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCHOM_A::_1)
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
#[doc = "PWM Synchronization Hardware Trigger 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG0_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG0`"]
pub type TRIG0_R = crate::R<bool, TRIG0_A>;
impl TRIG0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_A {
        match self.bits {
            false => TRIG0_A::_0,
            true => TRIG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG0_A::_1
    }
}
#[doc = "Write proxy for field `TRIG0`"]
pub struct TRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG0_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "PWM Synchronization Hardware Trigger 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG1_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG1`"]
pub type TRIG1_R = crate::R<bool, TRIG1_A>;
impl TRIG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_A {
        match self.bits {
            false => TRIG1_A::_0,
            true => TRIG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG1_A::_1
    }
}
#[doc = "Write proxy for field `TRIG1`"]
pub struct TRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG1_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "PWM Synchronization Hardware Trigger 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG2_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIG2`"]
pub type TRIG2_R = crate::R<bool, TRIG2_A>;
impl TRIG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_A {
        match self.bits {
            false => TRIG2_A::_0,
            true => TRIG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG2_A::_1
    }
}
#[doc = "Write proxy for field `TRIG2`"]
pub struct TRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG2_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "PWM Synchronization Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSYNC_A {
    #[doc = "0: Software trigger is not selected."]
    _0 = 0,
    #[doc = "1: Software trigger is selected."]
    _1 = 1,
}
impl From<SWSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWSYNC`"]
pub type SWSYNC_R = crate::R<bool, SWSYNC_A>;
impl SWSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSYNC_A {
        match self.bits {
            false => SWSYNC_A::_0,
            true => SWSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSYNC_A::_1
    }
}
#[doc = "Write proxy for field `SWSYNC`"]
pub struct SWSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSYNC_A::_0)
    }
    #[doc = "Software trigger is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSYNC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmin(&self) -> CNTMIN_R {
        CNTMIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmax(&self) -> CNTMAX_R {
        CNTMAX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline(always)]
    pub fn reinit(&self) -> REINIT_R {
        REINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    pub fn synchom(&self) -> SYNCHOM_R {
        SYNCHOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    pub fn trig0(&self) -> TRIG0_R {
        TRIG0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    pub fn trig1(&self) -> TRIG1_R {
        TRIG1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    pub fn trig2(&self) -> TRIG2_R {
        TRIG2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmin(&mut self) -> CNTMIN_W {
        CNTMIN_W { w: self }
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmax(&mut self) -> CNTMAX_W {
        CNTMAX_W { w: self }
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline(always)]
    pub fn reinit(&mut self) -> REINIT_W {
        REINIT_W { w: self }
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    pub fn synchom(&mut self) -> SYNCHOM_W {
        SYNCHOM_W { w: self }
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    pub fn trig0(&mut self) -> TRIG0_W {
        TRIG0_W { w: self }
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    pub fn trig1(&mut self) -> TRIG1_W {
        TRIG1_W { w: self }
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    pub fn trig2(&mut self) -> TRIG2_W {
        TRIG2_W { w: self }
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SWSYNC_W {
        SWSYNC_W { w: self }
    }
}
