#[doc = "Reader of register MISCTRL0"]
pub type R = crate::R<u32, super::MISCTRL0>;
#[doc = "Writer for register MISCTRL0"]
pub type W = crate::W<u32, super::MISCTRL0>;
#[doc = "Register MISCTRL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MISCTRL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM0 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM0_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM0_OBE_CTRL`"]
pub type FTM0_OBE_CTRL_R = crate::R<bool, FTM0_OBE_CTRL_A>;
impl FTM0_OBE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_OBE_CTRL_A {
        match self.bits {
            false => FTM0_OBE_CTRL_A::_0,
            true => FTM0_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0_OBE_CTRL_A::_1
    }
}
#[doc = "Write proxy for field `FTM0_OBE_CTRL`"]
pub struct FTM0_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_OBE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "FTM1 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM1_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM1_OBE_CTRL`"]
pub type FTM1_OBE_CTRL_R = crate::R<bool, FTM1_OBE_CTRL_A>;
impl FTM1_OBE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_OBE_CTRL_A {
        match self.bits {
            false => FTM1_OBE_CTRL_A::_0,
            true => FTM1_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1_OBE_CTRL_A::_1
    }
}
#[doc = "Write proxy for field `FTM1_OBE_CTRL`"]
pub struct FTM1_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1_OBE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "FTM2 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM2_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2_OBE_CTRL`"]
pub type FTM2_OBE_CTRL_R = crate::R<bool, FTM2_OBE_CTRL_A>;
impl FTM2_OBE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_OBE_CTRL_A {
        match self.bits {
            false => FTM2_OBE_CTRL_A::_0,
            true => FTM2_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_OBE_CTRL_A::_1
    }
}
#[doc = "Write proxy for field `FTM2_OBE_CTRL`"]
pub struct FTM2_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_OBE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "FTM3 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM3_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM3_OBE_CTRL`"]
pub type FTM3_OBE_CTRL_R = crate::R<bool, FTM3_OBE_CTRL_A>;
impl FTM3_OBE_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3_OBE_CTRL_A {
        match self.bits {
            false => FTM3_OBE_CTRL_A::_0,
            true => FTM3_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3_OBE_CTRL_A::_1
    }
}
#[doc = "Write proxy for field `FTM3_OBE_CTRL`"]
pub struct FTM3_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3_OBE_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm0_obe_ctrl(&self) -> FTM0_OBE_CTRL_R {
        FTM0_OBE_CTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm1_obe_ctrl(&self) -> FTM1_OBE_CTRL_R {
        FTM1_OBE_CTRL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm2_obe_ctrl(&self) -> FTM2_OBE_CTRL_R {
        FTM2_OBE_CTRL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm3_obe_ctrl(&self) -> FTM3_OBE_CTRL_R {
        FTM3_OBE_CTRL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm0_obe_ctrl(&mut self) -> FTM0_OBE_CTRL_W {
        FTM0_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm1_obe_ctrl(&mut self) -> FTM1_OBE_CTRL_W {
        FTM1_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm2_obe_ctrl(&mut self) -> FTM2_OBE_CTRL_W {
        FTM2_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm3_obe_ctrl(&mut self) -> FTM3_OBE_CTRL_W {
        FTM3_OBE_CTRL_W { w: self }
    }
}
