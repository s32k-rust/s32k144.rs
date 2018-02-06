#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FMS {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Possible values of the field `FAULTF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF0R {
    #[doc = "No fault condition was detected at the fault input."]
    _0,
    #[doc = "A fault condition was detected at the fault input."]
    _1,
}
impl FAULTF0R {
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
            FAULTF0R::_0 => false,
            FAULTF0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTF0R {
        match value {
            false => FAULTF0R::_0,
            true => FAULTF0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTF0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTF0R::_1
    }
}
#[doc = "Possible values of the field `FAULTF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF1R {
    #[doc = "No fault condition was detected at the fault input."]
    _0,
    #[doc = "A fault condition was detected at the fault input."]
    _1,
}
impl FAULTF1R {
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
            FAULTF1R::_0 => false,
            FAULTF1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTF1R {
        match value {
            false => FAULTF1R::_0,
            true => FAULTF1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTF1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTF1R::_1
    }
}
#[doc = "Possible values of the field `FAULTF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF2R {
    #[doc = "No fault condition was detected at the fault input."]
    _0,
    #[doc = "A fault condition was detected at the fault input."]
    _1,
}
impl FAULTF2R {
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
            FAULTF2R::_0 => false,
            FAULTF2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTF2R {
        match value {
            false => FAULTF2R::_0,
            true => FAULTF2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTF2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTF2R::_1
    }
}
#[doc = "Possible values of the field `FAULTF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF3R {
    #[doc = "No fault condition was detected at the fault input."]
    _0,
    #[doc = "A fault condition was detected at the fault input."]
    _1,
}
impl FAULTF3R {
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
            FAULTF3R::_0 => false,
            FAULTF3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTF3R {
        match value {
            false => FAULTF3R::_0,
            true => FAULTF3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTF3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTF3R::_1
    }
}
#[doc = "Possible values of the field `FAULTIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTINR {
    #[doc = "The logic OR of the enabled fault inputs is 0."]
    _0,
    #[doc = "The logic OR of the enabled fault inputs is 1."]
    _1,
}
impl FAULTINR {
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
            FAULTINR::_0 => false,
            FAULTINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTINR {
        match value {
            false => FAULTINR::_0,
            true => FAULTINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTINR::_1
    }
}
#[doc = "Possible values of the field `WPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPENR {
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    _0,
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    _1,
}
impl WPENR {
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
            WPENR::_0 => false,
            WPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPENR {
        match value {
            false => WPENR::_0,
            true => WPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPENR::_1
    }
}
#[doc = "Possible values of the field `FAULTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTFR {
    #[doc = "No fault condition was detected."]
    _0,
    #[doc = "A fault condition was detected."]
    _1,
}
impl FAULTFR {
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
            FAULTFR::_0 => false,
            FAULTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTFR {
        match value {
            false => FAULTFR::_0,
            true => FAULTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTFR::_1
    }
}
#[doc = "Values that can be written to the field `WPEN`"]
pub enum WPENW {
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    _0,
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    _1,
}
impl WPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPENW::_0 => false,
            WPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPENW::_0)
    }
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline]
    pub fn faultf0(&self) -> FAULTF0R {
        FAULTF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline]
    pub fn faultf1(&self) -> FAULTF1R {
        FAULTF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline]
    pub fn faultf2(&self) -> FAULTF2R {
        FAULTF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline]
    pub fn faultf3(&self) -> FAULTF3R {
        FAULTF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Fault Inputs"]
    #[inline]
    pub fn faultin(&self) -> FAULTINR {
        FAULTINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline]
    pub fn wpen(&self) -> WPENR {
        WPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline]
    pub fn faultf(&self) -> FAULTFR {
        FAULTFR::_from({
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
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline]
    pub fn wpen(&mut self) -> _WPENW {
        _WPENW { w: self }
    }
}
