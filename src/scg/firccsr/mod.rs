#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIRCCSR {
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
#[doc = "Possible values of the field `FIRCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCENR {
    #[doc = "Fast IRC is disabled"]
    _0,
    #[doc = "Fast IRC is enabled"]
    _1,
}
impl FIRCENR {
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
            FIRCENR::_0 => false,
            FIRCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRCENR {
        match value {
            false => FIRCENR::_0,
            true => FIRCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIRCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIRCENR::_1
    }
}
#[doc = "Possible values of the field `FIRCREGOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCREGOFFR {
    #[doc = "Fast IRC Regulator is enabled."]
    _0,
    #[doc = "Fast IRC Regulator is disabled."]
    _1,
}
impl FIRCREGOFFR {
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
            FIRCREGOFFR::_0 => false,
            FIRCREGOFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRCREGOFFR {
        match value {
            false => FIRCREGOFFR::_0,
            true => FIRCREGOFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIRCREGOFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIRCREGOFFR::_1
    }
}
#[doc = "Possible values of the field `LK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKR {
    #[doc = "Control Status Register can be written."]
    _0,
    #[doc = "Control Status Register cannot be written."]
    _1,
}
impl LKR {
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
            LKR::_0 => false,
            LKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LKR {
        match value {
            false => LKR::_0,
            true => LKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LKR::_1
    }
}
#[doc = "Possible values of the field `FIRCVLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCVLDR {
    #[doc = "Fast IRC is not enabled or clock is not valid."]
    _0,
    #[doc = "Fast IRC is enabled and output clock is valid. The clock is valid once there is an output clock from the FIRC analog."]
    _1,
}
impl FIRCVLDR {
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
            FIRCVLDR::_0 => false,
            FIRCVLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRCVLDR {
        match value {
            false => FIRCVLDR::_0,
            true => FIRCVLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIRCVLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIRCVLDR::_1
    }
}
#[doc = "Possible values of the field `FIRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCSELR {
    #[doc = "Fast IRC is not the system clock source"]
    _0,
    #[doc = "Fast IRC is the system clock source"]
    _1,
}
impl FIRCSELR {
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
            FIRCSELR::_0 => false,
            FIRCSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRCSELR {
        match value {
            false => FIRCSELR::_0,
            true => FIRCSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIRCSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIRCSELR::_1
    }
}
#[doc = "Possible values of the field `FIRCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCERRR {
    #[doc = "Error not detected with the Fast IRC trimming."]
    _0,
    #[doc = "Error detected with the Fast IRC trimming."]
    _1,
}
impl FIRCERRR {
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
            FIRCERRR::_0 => false,
            FIRCERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRCERRR {
        match value {
            false => FIRCERRR::_0,
            true => FIRCERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIRCERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIRCERRR::_1
    }
}
#[doc = "Values that can be written to the field `FIRCEN`"]
pub enum FIRCENW {
    #[doc = "Fast IRC is disabled"]
    _0,
    #[doc = "Fast IRC is enabled"]
    _1,
}
impl FIRCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIRCENW::_0 => false,
            FIRCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _FIRCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast IRC is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCENW::_0)
    }
    #[doc = "Fast IRC is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCENW::_1)
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
#[doc = "Values that can be written to the field `FIRCREGOFF`"]
pub enum FIRCREGOFFW {
    #[doc = "Fast IRC Regulator is enabled."]
    _0,
    #[doc = "Fast IRC Regulator is disabled."]
    _1,
}
impl FIRCREGOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIRCREGOFFW::_0 => false,
            FIRCREGOFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIRCREGOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _FIRCREGOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRCREGOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fast IRC Regulator is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCREGOFFW::_0)
    }
    #[doc = "Fast IRC Regulator is disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCREGOFFW::_1)
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
#[doc = "Values that can be written to the field `LK`"]
pub enum LKW {
    #[doc = "Control Status Register can be written."]
    _0,
    #[doc = "Control Status Register cannot be written."]
    _1,
}
impl LKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LKW::_0 => false,
            LKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LKW<'a> {
    w: &'a mut W,
}
impl<'a> _LKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Control Status Register can be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LKW::_0)
    }
    #[doc = "Control Status Register cannot be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LKW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FIRCERR`"]
pub enum FIRCERRW {
    #[doc = "Error not detected with the Fast IRC trimming."]
    _0,
    #[doc = "Error detected with the Fast IRC trimming."]
    _1,
}
impl FIRCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIRCERRW::_0 => false,
            FIRCERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIRCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIRCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error not detected with the Fast IRC trimming."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIRCERRW::_0)
    }
    #[doc = "Error detected with the Fast IRC trimming."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIRCERRW::_1)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - Fast IRC Enable"]
    #[inline]
    pub fn fircen(&self) -> FIRCENR {
        FIRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Fast IRC Regulator Enable"]
    #[inline]
    pub fn fircregoff(&self) -> FIRCREGOFFR {
        FIRCREGOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&self) -> LKR {
        LKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Fast IRC Valid status"]
    #[inline]
    pub fn fircvld(&self) -> FIRCVLDR {
        FIRCVLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Fast IRC Selected status"]
    #[inline]
    pub fn fircsel(&self) -> FIRCSELR {
        FIRCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Fast IRC Clock Error"]
    #[inline]
    pub fn fircerr(&self) -> FIRCERRR {
        FIRCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50331649 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast IRC Enable"]
    #[inline]
    pub fn fircen(&mut self) -> _FIRCENW {
        _FIRCENW { w: self }
    }
    #[doc = "Bit 3 - Fast IRC Regulator Enable"]
    #[inline]
    pub fn fircregoff(&mut self) -> _FIRCREGOFFW {
        _FIRCREGOFFW { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
    #[doc = "Bit 26 - Fast IRC Clock Error"]
    #[inline]
    pub fn fircerr(&mut self) -> _FIRCERRW {
        _FIRCERRW { w: self }
    }
}
