#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MISCTRL0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `FTM0_OBE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_OBE_CTRLR {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM0_OBE_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FTM0_OBE_CTRLR::_0 => false,
            FTM0_OBE_CTRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM0_OBE_CTRLR {
        match value {
            false => FTM0_OBE_CTRLR::_0,
            true => FTM0_OBE_CTRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM0_OBE_CTRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM0_OBE_CTRLR::_1
    }
}
#[doc = "Possible values of the field `FTM1_OBE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_OBE_CTRLR {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM1_OBE_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FTM1_OBE_CTRLR::_0 => false,
            FTM1_OBE_CTRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM1_OBE_CTRLR {
        match value {
            false => FTM1_OBE_CTRLR::_0,
            true => FTM1_OBE_CTRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM1_OBE_CTRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM1_OBE_CTRLR::_1
    }
}
#[doc = "Possible values of the field `FTM2_OBE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_OBE_CTRLR {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM2_OBE_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FTM2_OBE_CTRLR::_0 => false,
            FTM2_OBE_CTRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM2_OBE_CTRLR {
        match value {
            false => FTM2_OBE_CTRLR::_0,
            true => FTM2_OBE_CTRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM2_OBE_CTRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM2_OBE_CTRLR::_1
    }
}
#[doc = "Possible values of the field `FTM3_OBE_CTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3_OBE_CTRLR {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM3_OBE_CTRLR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FTM3_OBE_CTRLR::_0 => false,
            FTM3_OBE_CTRLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FTM3_OBE_CTRLR {
        match value {
            false => FTM3_OBE_CTRLR::_0,
            true => FTM3_OBE_CTRLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FTM3_OBE_CTRLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FTM3_OBE_CTRLR::_1
    }
}
#[doc = "Values that can be written to the field `FTM0_OBE_CTRL`"]
pub enum FTM0_OBE_CTRLW {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM0_OBE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM0_OBE_CTRLW::_0 => false,
            FTM0_OBE_CTRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM0_OBE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM0_OBE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM0_OBE_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRLW::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM1_OBE_CTRL`"]
pub enum FTM1_OBE_CTRLW {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM1_OBE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM1_OBE_CTRLW::_0 => false,
            FTM1_OBE_CTRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM1_OBE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM1_OBE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM1_OBE_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRLW::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM2_OBE_CTRL`"]
pub enum FTM2_OBE_CTRLW {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM2_OBE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM2_OBE_CTRLW::_0 => false,
            FTM2_OBE_CTRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM2_OBE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM2_OBE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM2_OBE_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRLW::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FTM3_OBE_CTRL`"]
pub enum FTM3_OBE_CTRLW {
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    _0,
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1,
}
impl FTM3_OBE_CTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FTM3_OBE_CTRLW::_0 => false,
            FTM3_OBE_CTRLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FTM3_OBE_CTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _FTM3_OBE_CTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FTM3_OBE_CTRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\] = 1'b1). Otherwise the channel output is tristated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRLW::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\] or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRLW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline]
    pub fn ftm0_obe_ctrl(&self) -> FTM0_OBE_CTRLR {
        FTM0_OBE_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline]
    pub fn ftm1_obe_ctrl(&self) -> FTM1_OBE_CTRLR {
        FTM1_OBE_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline]
    pub fn ftm2_obe_ctrl(&self) -> FTM2_OBE_CTRLR {
        FTM2_OBE_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline]
    pub fn ftm3_obe_ctrl(&self) -> FTM3_OBE_CTRLR {
        FTM3_OBE_CTRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline]
    pub fn ftm0_obe_ctrl(&mut self) -> _FTM0_OBE_CTRLW {
        _FTM0_OBE_CTRLW { w: self }
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline]
    pub fn ftm1_obe_ctrl(&mut self) -> _FTM1_OBE_CTRLW {
        _FTM1_OBE_CTRLW { w: self }
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline]
    pub fn ftm2_obe_ctrl(&mut self) -> _FTM2_OBE_CTRLW {
        _FTM2_OBE_CTRLW { w: self }
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline]
    pub fn ftm3_obe_ctrl(&mut self) -> _FTM3_OBE_CTRLW {
        _FTM3_OBE_CTRLW { w: self }
    }
}
