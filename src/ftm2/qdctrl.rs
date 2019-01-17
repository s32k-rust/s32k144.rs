#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QDCTRL {
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
#[doc = "Possible values of the field `QUADEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADENR {
    #[doc = "Quadrature Decoder mode is disabled."]
    _0,
    #[doc = "Quadrature Decoder mode is enabled."]
    _1,
}
impl QUADENR {
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
            QUADENR::_0 => false,
            QUADENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADENR {
        match value {
            false => QUADENR::_0,
            true => QUADENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADENR::_1
    }
}
#[doc = "Possible values of the field `TOFDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFDIRR {
    #[doc = "TOF bit was set on the bottom of counting. There was an FTM counter decrement and FTM counter changes from its minimum value (CNTIN register) to its maximum value (MOD register)."]
    _0,
    #[doc = "TOF bit was set on the top of counting. There was an FTM counter increment and FTM counter changes from its maximum value (MOD register) to its minimum value (CNTIN register)."]
    _1,
}
impl TOFDIRR {
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
            TOFDIRR::_0 => false,
            TOFDIRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOFDIRR {
        match value {
            false => TOFDIRR::_0,
            true => TOFDIRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOFDIRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOFDIRR::_1
    }
}
#[doc = "Possible values of the field `QUADIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADIRR {
    #[doc = "Counting direction is decreasing (FTM counter decrement)."]
    _0,
    #[doc = "Counting direction is increasing (FTM counter increment)."]
    _1,
}
impl QUADIRR {
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
            QUADIRR::_0 => false,
            QUADIRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADIRR {
        match value {
            false => QUADIRR::_0,
            true => QUADIRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADIRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADIRR::_1
    }
}
#[doc = "Possible values of the field `QUADMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUADMODER {
    #[doc = "Phase A and phase B encoding mode."]
    _0,
    #[doc = "Count and direction encoding mode."]
    _1,
}
impl QUADMODER {
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
            QUADMODER::_0 => false,
            QUADMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUADMODER {
        match value {
            false => QUADMODER::_0,
            true => QUADMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == QUADMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == QUADMODER::_1
    }
}
#[doc = "Possible values of the field `PHBPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBPOLR {
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0,
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    _1,
}
impl PHBPOLR {
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
            PHBPOLR::_0 => false,
            PHBPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHBPOLR {
        match value {
            false => PHBPOLR::_0,
            true => PHBPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PHBPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PHBPOLR::_1
    }
}
#[doc = "Possible values of the field `PHAPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAPOLR {
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0,
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    _1,
}
impl PHAPOLR {
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
            PHAPOLR::_0 => false,
            PHAPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHAPOLR {
        match value {
            false => PHAPOLR::_0,
            true => PHAPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PHAPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PHAPOLR::_1
    }
}
#[doc = "Possible values of the field `PHBFLTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHBFLTRENR {
    #[doc = "Phase B input filter is disabled."]
    _0,
    #[doc = "Phase B input filter is enabled."]
    _1,
}
impl PHBFLTRENR {
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
            PHBFLTRENR::_0 => false,
            PHBFLTRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHBFLTRENR {
        match value {
            false => PHBFLTRENR::_0,
            true => PHBFLTRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PHBFLTRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PHBFLTRENR::_1
    }
}
#[doc = "Possible values of the field `PHAFLTREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHAFLTRENR {
    #[doc = "Phase A input filter is disabled."]
    _0,
    #[doc = "Phase A input filter is enabled."]
    _1,
}
impl PHAFLTRENR {
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
            PHAFLTRENR::_0 => false,
            PHAFLTRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PHAFLTRENR {
        match value {
            false => PHAFLTRENR::_0,
            true => PHAFLTRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PHAFLTRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PHAFLTRENR::_1
    }
}
#[doc = "Values that can be written to the field `QUADEN`"]
pub enum QUADENW {
    #[doc = "Quadrature Decoder mode is disabled."]
    _0,
    #[doc = "Quadrature Decoder mode is enabled."]
    _1,
}
impl QUADENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QUADENW::_0 => false,
            QUADENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUADENW<'a> {
    w: &'a mut W,
}
impl<'a> _QUADENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUADENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Quadrature Decoder mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADENW::_0)
    }
    #[doc = "Quadrature Decoder mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADENW::_1)
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
#[doc = "Values that can be written to the field `QUADMODE`"]
pub enum QUADMODEW {
    #[doc = "Phase A and phase B encoding mode."]
    _0,
    #[doc = "Count and direction encoding mode."]
    _1,
}
impl QUADMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            QUADMODEW::_0 => false,
            QUADMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _QUADMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _QUADMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUADMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase A and phase B encoding mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(QUADMODEW::_0)
    }
    #[doc = "Count and direction encoding mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(QUADMODEW::_1)
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
#[doc = "Values that can be written to the field `PHBPOL`"]
pub enum PHBPOLW {
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0,
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    _1,
}
impl PHBPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHBPOLW::_0 => false,
            PHBPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHBPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PHBPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHBPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal polarity. Phase B input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBPOLW::_0)
    }
    #[doc = "Inverted polarity. Phase B input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBPOLW::_1)
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
#[doc = "Values that can be written to the field `PHAPOL`"]
pub enum PHAPOLW {
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    _0,
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    _1,
}
impl PHAPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHAPOLW::_0 => false,
            PHAPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHAPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PHAPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHAPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal polarity. Phase A input signal is not inverted before identifying the rising and falling edges of this signal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAPOLW::_0)
    }
    #[doc = "Inverted polarity. Phase A input signal is inverted before identifying the rising and falling edges of this signal."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAPOLW::_1)
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
#[doc = "Values that can be written to the field `PHBFLTREN`"]
pub enum PHBFLTRENW {
    #[doc = "Phase B input filter is disabled."]
    _0,
    #[doc = "Phase B input filter is enabled."]
    _1,
}
impl PHBFLTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHBFLTRENW::_0 => false,
            PHBFLTRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHBFLTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PHBFLTRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHBFLTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase B input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHBFLTRENW::_0)
    }
    #[doc = "Phase B input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHBFLTRENW::_1)
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
#[doc = "Values that can be written to the field `PHAFLTREN`"]
pub enum PHAFLTRENW {
    #[doc = "Phase A input filter is disabled."]
    _0,
    #[doc = "Phase A input filter is enabled."]
    _1,
}
impl PHAFLTRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PHAFLTRENW::_0 => false,
            PHAFLTRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHAFLTRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PHAFLTRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHAFLTRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Phase A input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHAFLTRENW::_0)
    }
    #[doc = "Phase A input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHAFLTRENW::_1)
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
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline]
    pub fn quaden(&self) -> QUADENR {
        QUADENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer Overflow Direction In Quadrature Decoder Mode"]
    #[inline]
    pub fn tofdir(&self) -> TOFDIRR {
        TOFDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - FTM Counter Direction In Quadrature Decoder Mode"]
    #[inline]
    pub fn quadir(&self) -> QUADIRR {
        QUADIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline]
    pub fn quadmode(&self) -> QUADMODER {
        QUADMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline]
    pub fn phbpol(&self) -> PHBPOLR {
        PHBPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline]
    pub fn phapol(&self) -> PHAPOLR {
        PHAPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline]
    pub fn phbfltren(&self) -> PHBFLTRENR {
        PHBFLTRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline]
    pub fn phafltren(&self) -> PHAFLTRENR {
        PHAFLTRENR::_from({
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
    #[doc = "Bit 0 - Quadrature Decoder Mode Enable"]
    #[inline]
    pub fn quaden(&mut self) -> _QUADENW {
        _QUADENW { w: self }
    }
    #[doc = "Bit 3 - Quadrature Decoder Mode"]
    #[inline]
    pub fn quadmode(&mut self) -> _QUADMODEW {
        _QUADMODEW { w: self }
    }
    #[doc = "Bit 4 - Phase B Input Polarity"]
    #[inline]
    pub fn phbpol(&mut self) -> _PHBPOLW {
        _PHBPOLW { w: self }
    }
    #[doc = "Bit 5 - Phase A Input Polarity"]
    #[inline]
    pub fn phapol(&mut self) -> _PHAPOLW {
        _PHAPOLW { w: self }
    }
    #[doc = "Bit 6 - Phase B Input Filter Enable"]
    #[inline]
    pub fn phbfltren(&mut self) -> _PHBFLTRENW {
        _PHBFLTRENW { w: self }
    }
    #[doc = "Bit 7 - Phase A Input Filter Enable"]
    #[inline]
    pub fn phafltren(&mut self) -> _PHAFLTRENW {
        _PHAFLTRENW { w: self }
    }
}
