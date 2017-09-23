#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLTCTRL {
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
#[doc = "Possible values of the field `FAULT0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT0ENR {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT0ENR {
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
            FAULT0ENR::_0 => false,
            FAULT0ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULT0ENR {
        match value {
            false => FAULT0ENR::_0,
            true => FAULT0ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULT0ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULT0ENR::_1
    }
}
#[doc = "Possible values of the field `FAULT1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT1ENR {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT1ENR {
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
            FAULT1ENR::_0 => false,
            FAULT1ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULT1ENR {
        match value {
            false => FAULT1ENR::_0,
            true => FAULT1ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULT1ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULT1ENR::_1
    }
}
#[doc = "Possible values of the field `FAULT2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT2ENR {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT2ENR {
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
            FAULT2ENR::_0 => false,
            FAULT2ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULT2ENR {
        match value {
            false => FAULT2ENR::_0,
            true => FAULT2ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULT2ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULT2ENR::_1
    }
}
#[doc = "Possible values of the field `FAULT3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT3ENR {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT3ENR {
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
            FAULT3ENR::_0 => false,
            FAULT3ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULT3ENR {
        match value {
            false => FAULT3ENR::_0,
            true => FAULT3ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULT3ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULT3ENR::_1
    }
}
#[doc = "Possible values of the field `FFLTR0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR0ENR {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR0ENR {
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
            FFLTR0ENR::_0 => false,
            FFLTR0ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFLTR0ENR {
        match value {
            false => FFLTR0ENR::_0,
            true => FFLTR0ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FFLTR0ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FFLTR0ENR::_1
    }
}
#[doc = "Possible values of the field `FFLTR1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR1ENR {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR1ENR {
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
            FFLTR1ENR::_0 => false,
            FFLTR1ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFLTR1ENR {
        match value {
            false => FFLTR1ENR::_0,
            true => FFLTR1ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FFLTR1ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FFLTR1ENR::_1
    }
}
#[doc = "Possible values of the field `FFLTR2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR2ENR {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR2ENR {
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
            FFLTR2ENR::_0 => false,
            FFLTR2ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFLTR2ENR {
        match value {
            false => FFLTR2ENR::_0,
            true => FFLTR2ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FFLTR2ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FFLTR2ENR::_1
    }
}
#[doc = "Possible values of the field `FFLTR3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR3ENR {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR3ENR {
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
            FFLTR3ENR::_0 => false,
            FFLTR3ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FFLTR3ENR {
        match value {
            false => FFLTR3ENR::_0,
            true => FFLTR3ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FFLTR3ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FFLTR3ENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct FFVALR {
    bits: u8,
}
impl FFVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSTATER {
    #[doc = "FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    _0,
    #[doc = "FTM outputs will be tri-stated when fault event is ongoing"] _1,
}
impl FSTATER {
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
            FSTATER::_0 => false,
            FSTATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSTATER {
        match value {
            false => FSTATER::_0,
            true => FSTATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FSTATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FSTATER::_1
    }
}
#[doc = "Values that can be written to the field `FAULT0EN`"]
pub enum FAULT0ENW {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULT0ENW::_0 => false,
            FAULT0ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULT0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT0ENW::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT0ENW::_1)
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
#[doc = "Values that can be written to the field `FAULT1EN`"]
pub enum FAULT1ENW {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULT1ENW::_0 => false,
            FAULT1ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULT1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT1ENW::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT1ENW::_1)
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
#[doc = "Values that can be written to the field `FAULT2EN`"]
pub enum FAULT2ENW {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULT2ENW::_0 => false,
            FAULT2ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULT2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT2ENW::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT2ENW::_1)
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
#[doc = "Values that can be written to the field `FAULT3EN`"]
pub enum FAULT3ENW {
    #[doc = "Fault input is disabled."] _0,
    #[doc = "Fault input is enabled."] _1,
}
impl FAULT3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULT3ENW::_0 => false,
            FAULT3ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULT3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAULT3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULT3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT3ENW::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT3ENW::_1)
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
#[doc = "Values that can be written to the field `FFLTR0EN`"]
pub enum FFLTR0ENW {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFLTR0ENW::_0 => false,
            FFLTR0ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFLTR0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLTR0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFLTR0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR0ENW::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR0ENW::_1)
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
#[doc = "Values that can be written to the field `FFLTR1EN`"]
pub enum FFLTR1ENW {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFLTR1ENW::_0 => false,
            FFLTR1ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFLTR1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLTR1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFLTR1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR1ENW::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR1ENW::_1)
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
#[doc = "Values that can be written to the field `FFLTR2EN`"]
pub enum FFLTR2ENW {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFLTR2ENW::_0 => false,
            FFLTR2ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFLTR2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLTR2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFLTR2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR2ENW::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR2ENW::_1)
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
#[doc = "Values that can be written to the field `FFLTR3EN`"]
pub enum FFLTR3ENW {
    #[doc = "Fault input filter is disabled."] _0,
    #[doc = "Fault input filter is enabled."] _1,
}
impl FFLTR3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FFLTR3ENW::_0 => false,
            FFLTR3ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FFLTR3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLTR3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FFLTR3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fault input filter is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR3ENW::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR3ENW::_1)
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
#[doc = r" Proxy"]
pub struct _FFVALW<'a> {
    w: &'a mut W,
}
impl<'a> _FFVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSTATE`"]
pub enum FSTATEW {
    #[doc = "FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    _0,
    #[doc = "FTM outputs will be tri-stated when fault event is ongoing"] _1,
}
impl FSTATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSTATEW::_0 => false,
            FSTATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSTATEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSTATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSTATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSTATEW::_0)
    }
    #[doc = "FTM outputs will be tri-stated when fault event is ongoing"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSTATEW::_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline]
    pub fn fault0en(&self) -> FAULT0ENR {
        FAULT0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline]
    pub fn fault1en(&self) -> FAULT1ENR {
        FAULT1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline]
    pub fn fault2en(&self) -> FAULT2ENR {
        FAULT2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline]
    pub fn fault3en(&self) -> FAULT3ENR {
        FAULT3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline]
    pub fn ffltr0en(&self) -> FFLTR0ENR {
        FFLTR0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline]
    pub fn ffltr1en(&self) -> FFLTR1ENR {
        FFLTR1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline]
    pub fn ffltr2en(&self) -> FFLTR2ENR {
        FFLTR2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline]
    pub fn ffltr3en(&self) -> FFLTR3ENR {
        FFLTR3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline]
    pub fn ffval(&self) -> FFVALR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FFVALR { bits }
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline]
    pub fn fstate(&self) -> FSTATER {
        FSTATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline]
    pub fn fault0en(&mut self) -> _FAULT0ENW {
        _FAULT0ENW { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline]
    pub fn fault1en(&mut self) -> _FAULT1ENW {
        _FAULT1ENW { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline]
    pub fn fault2en(&mut self) -> _FAULT2ENW {
        _FAULT2ENW { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline]
    pub fn fault3en(&mut self) -> _FAULT3ENW {
        _FAULT3ENW { w: self }
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline]
    pub fn ffltr0en(&mut self) -> _FFLTR0ENW {
        _FFLTR0ENW { w: self }
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline]
    pub fn ffltr1en(&mut self) -> _FFLTR1ENW {
        _FFLTR1ENW { w: self }
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline]
    pub fn ffltr2en(&mut self) -> _FFLTR2ENW {
        _FFLTR2ENW { w: self }
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline]
    pub fn ffltr3en(&mut self) -> _FFLTR3ENW {
        _FFLTR3ENW { w: self }
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline]
    pub fn ffval(&mut self) -> _FFVALW {
        _FFVALW { w: self }
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline]
    pub fn fstate(&mut self) -> _FSTATEW {
        _FSTATEW { w: self }
    }
}
