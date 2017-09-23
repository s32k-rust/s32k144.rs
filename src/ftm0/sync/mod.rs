#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNC {
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
#[doc = "Possible values of the field `CNTMIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMINR {
    #[doc = "The minimum loading point is disabled."]
    _0,
    #[doc = "The minimum loading point is enabled."]
    _1,
}
impl CNTMINR {
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
            CNTMINR::_0 => false,
            CNTMINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTMINR {
        match value {
            false => CNTMINR::_0,
            true => CNTMINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CNTMINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CNTMINR::_1
    }
}
#[doc = "Possible values of the field `CNTMAX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAXR {
    #[doc = "The maximum loading point is disabled."]
    _0,
    #[doc = "The maximum loading point is enabled."]
    _1,
}
impl CNTMAXR {
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
            CNTMAXR::_0 => false,
            CNTMAXR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTMAXR {
        match value {
            false => CNTMAXR::_0,
            true => CNTMAXR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CNTMAXR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CNTMAXR::_1
    }
}
#[doc = "Possible values of the field `REINIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REINITR {
    #[doc = "FTM counter continues to count normally."]
    _0,
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    _1,
}
impl REINITR {
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
            REINITR::_0 => false,
            REINITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REINITR {
        match value {
            false => REINITR::_0,
            true => REINITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REINITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REINITR::_1
    }
}
#[doc = "Possible values of the field `SYNCHOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHOMR {
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    _0,
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    _1,
}
impl SYNCHOMR {
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
            SYNCHOMR::_0 => false,
            SYNCHOMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCHOMR {
        match value {
            false => SYNCHOMR::_0,
            true => SYNCHOMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCHOMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCHOMR::_1
    }
}
#[doc = "Possible values of the field `TRIG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG0R {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG0R {
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
            TRIG0R::_0 => false,
            TRIG0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIG0R {
        match value {
            false => TRIG0R::_0,
            true => TRIG0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIG0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIG0R::_1
    }
}
#[doc = "Possible values of the field `TRIG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG1R {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG1R {
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
            TRIG1R::_0 => false,
            TRIG1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIG1R {
        match value {
            false => TRIG1R::_0,
            true => TRIG1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIG1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIG1R::_1
    }
}
#[doc = "Possible values of the field `TRIG2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG2R {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG2R {
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
            TRIG2R::_0 => false,
            TRIG2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIG2R {
        match value {
            false => TRIG2R::_0,
            true => TRIG2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIG2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIG2R::_1
    }
}
#[doc = "Possible values of the field `SWSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSYNCR {
    #[doc = "Software trigger is not selected."]
    _0,
    #[doc = "Software trigger is selected."]
    _1,
}
impl SWSYNCR {
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
            SWSYNCR::_0 => false,
            SWSYNCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWSYNCR {
        match value {
            false => SWSYNCR::_0,
            true => SWSYNCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWSYNCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWSYNCR::_1
    }
}
#[doc = "Values that can be written to the field `CNTMIN`"]
pub enum CNTMINW {
    #[doc = "The minimum loading point is disabled."]
    _0,
    #[doc = "The minimum loading point is enabled."]
    _1,
}
impl CNTMINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNTMINW::_0 => false,
            CNTMINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTMINW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTMINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTMINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The minimum loading point is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMINW::_0)
    }
    #[doc = "The minimum loading point is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMINW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CNTMAX`"]
pub enum CNTMAXW {
    #[doc = "The maximum loading point is disabled."]
    _0,
    #[doc = "The maximum loading point is enabled."]
    _1,
}
impl CNTMAXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNTMAXW::_0 => false,
            CNTMAXW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTMAXW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTMAXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTMAXW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The maximum loading point is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAXW::_0)
    }
    #[doc = "The maximum loading point is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAXW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `REINIT`"]
pub enum REINITW {
    #[doc = "FTM counter continues to count normally."]
    _0,
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    _1,
}
impl REINITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REINITW::_0 => false,
            REINITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REINITW<'a> {
    w: &'a mut W,
}
impl<'a> _REINITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REINITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM counter continues to count normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REINITW::_0)
    }
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REINITW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYNCHOM`"]
pub enum SYNCHOMW {
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    _0,
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    _1,
}
impl SYNCHOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCHOMW::_0 => false,
            SYNCHOMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCHOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCHOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCHOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCHOMW::_0)
    }
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCHOMW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG0`"]
pub enum TRIG0W {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIG0W::_0 => false,
            TRIG0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIG0W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIG0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG0W::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG0W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG1`"]
pub enum TRIG1W {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIG1W::_0 => false,
            TRIG1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIG1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG1W::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG1W::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG2`"]
pub enum TRIG2W {
    #[doc = "Trigger is disabled."]
    _0,
    #[doc = "Trigger is enabled."]
    _1,
}
impl TRIG2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIG2W::_0 => false,
            TRIG2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _TRIG2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIG2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG2W::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG2W::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWSYNC`"]
pub enum SWSYNCW {
    #[doc = "Software trigger is not selected."]
    _0,
    #[doc = "Software trigger is selected."]
    _1,
}
impl SWSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWSYNCW::_0 => false,
            SWSYNCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software trigger is not selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSYNCW::_0)
    }
    #[doc = "Software trigger is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSYNCW::_1)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline]
    pub fn cntmin(&self) -> CNTMINR {
        CNTMINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline]
    pub fn cntmax(&self) -> CNTMAXR {
        CNTMAXR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline]
    pub fn reinit(&self) -> REINITR {
        REINITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline]
    pub fn synchom(&self) -> SYNCHOMR {
        SYNCHOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline]
    pub fn trig0(&self) -> TRIG0R {
        TRIG0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline]
    pub fn trig1(&self) -> TRIG1R {
        TRIG1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline]
    pub fn trig2(&self) -> TRIG2R {
        TRIG2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline]
    pub fn swsync(&self) -> SWSYNCR {
        SWSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline]
    pub fn cntmin(&mut self) -> _CNTMINW {
        _CNTMINW { w: self }
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline]
    pub fn cntmax(&mut self) -> _CNTMAXW {
        _CNTMAXW { w: self }
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline]
    pub fn reinit(&mut self) -> _REINITW {
        _REINITW { w: self }
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline]
    pub fn synchom(&mut self) -> _SYNCHOMW {
        _SYNCHOMW { w: self }
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline]
    pub fn trig0(&mut self) -> _TRIG0W {
        _TRIG0W { w: self }
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline]
    pub fn trig1(&mut self) -> _TRIG1W {
        _TRIG1W { w: self }
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline]
    pub fn trig2(&mut self) -> _TRIG2W {
        _TRIG2W { w: self }
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline]
    pub fn swsync(&mut self) -> _SWSYNCW {
        _SWSYNCW { w: self }
    }
}
