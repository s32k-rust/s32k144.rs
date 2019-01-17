#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIRCCSR {
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
#[doc = "Possible values of the field `SIRCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCENR {
    #[doc = "Slow IRC is disabled"]
    _0,
    #[doc = "Slow IRC is enabled"]
    _1,
}
impl SIRCENR {
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
            SIRCENR::_0 => false,
            SIRCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIRCENR {
        match value {
            false => SIRCENR::_0,
            true => SIRCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIRCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIRCENR::_1
    }
}
#[doc = "Possible values of the field `SIRCSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSTENR {
    #[doc = "Slow IRC is disabled in supported Stop modes"]
    _0,
    #[doc = "Slow IRC is enabled in supported Stop modes"]
    _1,
}
impl SIRCSTENR {
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
            SIRCSTENR::_0 => false,
            SIRCSTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIRCSTENR {
        match value {
            false => SIRCSTENR::_0,
            true => SIRCSTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIRCSTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIRCSTENR::_1
    }
}
#[doc = "Possible values of the field `SIRCLPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCLPENR {
    #[doc = "Slow IRC is disabled in VLP modes"]
    _0,
    #[doc = "Slow IRC is enabled in VLP modes"]
    _1,
}
impl SIRCLPENR {
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
            SIRCLPENR::_0 => false,
            SIRCLPENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIRCLPENR {
        match value {
            false => SIRCLPENR::_0,
            true => SIRCLPENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIRCLPENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIRCLPENR::_1
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
#[doc = "Possible values of the field `SIRCVLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCVLDR {
    #[doc = "Slow IRC is not enabled or clock is not valid"]
    _0,
    #[doc = "Slow IRC is enabled and output clock is valid"]
    _1,
}
impl SIRCVLDR {
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
            SIRCVLDR::_0 => false,
            SIRCVLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIRCVLDR {
        match value {
            false => SIRCVLDR::_0,
            true => SIRCVLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIRCVLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIRCVLDR::_1
    }
}
#[doc = "Possible values of the field `SIRCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCSELR {
    #[doc = "Slow IRC is not the system clock source"]
    _0,
    #[doc = "Slow IRC is the system clock source"]
    _1,
}
impl SIRCSELR {
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
            SIRCSELR::_0 => false,
            SIRCSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIRCSELR {
        match value {
            false => SIRCSELR::_0,
            true => SIRCSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIRCSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIRCSELR::_1
    }
}
#[doc = "Values that can be written to the field `SIRCEN`"]
pub enum SIRCENW {
    #[doc = "Slow IRC is disabled"]
    _0,
    #[doc = "Slow IRC is enabled"]
    _1,
}
impl SIRCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIRCENW::_0 => false,
            SIRCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIRCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIRCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slow IRC is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCENW::_0)
    }
    #[doc = "Slow IRC is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCENW::_1)
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
#[doc = "Values that can be written to the field `SIRCSTEN`"]
pub enum SIRCSTENW {
    #[doc = "Slow IRC is disabled in supported Stop modes"]
    _0,
    #[doc = "Slow IRC is enabled in supported Stop modes"]
    _1,
}
impl SIRCSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIRCSTENW::_0 => false,
            SIRCSTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIRCSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIRCSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIRCSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slow IRC is disabled in supported Stop modes"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCSTENW::_0)
    }
    #[doc = "Slow IRC is enabled in supported Stop modes"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCSTENW::_1)
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
#[doc = "Values that can be written to the field `SIRCLPEN`"]
pub enum SIRCLPENW {
    #[doc = "Slow IRC is disabled in VLP modes"]
    _0,
    #[doc = "Slow IRC is enabled in VLP modes"]
    _1,
}
impl SIRCLPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIRCLPENW::_0 => false,
            SIRCLPENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIRCLPENW<'a> {
    w: &'a mut W,
}
impl<'a> _SIRCLPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIRCLPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slow IRC is disabled in VLP modes"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIRCLPENW::_0)
    }
    #[doc = "Slow IRC is enabled in VLP modes"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIRCLPENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slow IRC Enable"]
    #[inline]
    pub fn sircen(&self) -> SIRCENR {
        SIRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Slow IRC Stop Enable"]
    #[inline]
    pub fn sircsten(&self) -> SIRCSTENR {
        SIRCSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Slow IRC Low Power Enable"]
    #[inline]
    pub fn sirclpen(&self) -> SIRCLPENR {
        SIRCLPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 24 - Slow IRC Valid"]
    #[inline]
    pub fn sircvld(&self) -> SIRCVLDR {
        SIRCVLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Slow IRC Selected"]
    #[inline]
    pub fn sircsel(&self) -> SIRCSELR {
        SIRCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777221 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slow IRC Enable"]
    #[inline]
    pub fn sircen(&mut self) -> _SIRCENW {
        _SIRCENW { w: self }
    }
    #[doc = "Bit 1 - Slow IRC Stop Enable"]
    #[inline]
    pub fn sircsten(&mut self) -> _SIRCSTENW {
        _SIRCSTENW { w: self }
    }
    #[doc = "Bit 2 - Slow IRC Low Power Enable"]
    #[inline]
    pub fn sirclpen(&mut self) -> _SIRCLPENW {
        _SIRCLPENW { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
}
