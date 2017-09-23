#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIER {
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
#[doc = "Possible values of the field `TDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled"]
    _1,
}
impl TDIER {
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
            TDIER::_0 => false,
            TDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDIER {
        match value {
            false => TDIER::_0,
            true => TDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDIER::_1
    }
}
#[doc = "Possible values of the field `RDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl RDIER {
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
            RDIER::_0 => false,
            RDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDIER {
        match value {
            false => RDIER::_0,
            true => RDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDIER::_1
    }
}
#[doc = "Possible values of the field `AVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl AVIER {
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
            AVIER::_0 => false,
            AVIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVIER {
        match value {
            false => AVIER::_0,
            true => AVIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVIER::_1
    }
}
#[doc = "Possible values of the field `TAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TAIER {
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
            TAIER::_0 => false,
            TAIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIER {
        match value {
            false => TAIER::_0,
            true => TAIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TAIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TAIER::_1
    }
}
#[doc = "Possible values of the field `RSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl RSIER {
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
            RSIER::_0 => false,
            RSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSIER {
        match value {
            false => RSIER::_0,
            true => RSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSIER::_1
    }
}
#[doc = "Possible values of the field `SDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SDIER {
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
            SDIER::_0 => false,
            SDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDIER {
        match value {
            false => SDIER::_0,
            true => SDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDIER::_1
    }
}
#[doc = "Possible values of the field `BEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl BEIER {
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
            BEIER::_0 => false,
            BEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEIER {
        match value {
            false => BEIER::_0,
            true => BEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEIER::_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl FEIER {
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
            FEIER::_0 => false,
            FEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::_0,
            true => FEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEIER::_1
    }
}
#[doc = "Possible values of the field `AM0IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0IER {
    #[doc = "Interrupt enabled."]
    _0,
    #[doc = "Interrupt disabled."]
    _1,
}
impl AM0IER {
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
            AM0IER::_0 => false,
            AM0IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM0IER {
        match value {
            false => AM0IER::_0,
            true => AM0IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AM0IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AM0IER::_1
    }
}
#[doc = "Possible values of the field `AM1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1FR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl AM1FR {
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
            AM1FR::_0 => false,
            AM1FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM1FR {
        match value {
            false => AM1FR::_0,
            true => AM1FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AM1FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AM1FR::_1
    }
}
#[doc = "Possible values of the field `GCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl GCIER {
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
            GCIER::_0 => false,
            GCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCIER {
        match value {
            false => GCIER::_0,
            true => GCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GCIER::_1
    }
}
#[doc = "Possible values of the field `SARIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SARIER {
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
            SARIER::_0 => false,
            SARIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SARIER {
        match value {
            false => SARIER::_0,
            true => SARIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SARIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SARIER::_1
    }
}
#[doc = "Values that can be written to the field `TDIE`"]
pub enum TDIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled"]
    _1,
}
impl TDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDIEW::_0 => false,
            TDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIEW::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIEW::_1)
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
#[doc = "Values that can be written to the field `RDIE`"]
pub enum RDIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl RDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDIEW::_0 => false,
            RDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDIEW::_1)
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
#[doc = "Values that can be written to the field `AVIE`"]
pub enum AVIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl AVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVIEW::_0 => false,
            AVIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVIEW::_1)
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
#[doc = "Values that can be written to the field `TAIE`"]
pub enum TAIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAIEW::_0 => false,
            TAIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIEW::_1)
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
#[doc = "Values that can be written to the field `RSIE`"]
pub enum RSIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl RSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSIEW::_0 => false,
            RSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSIEW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIE`"]
pub enum SDIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDIEW::_0 => false,
            SDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDIEW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEIE`"]
pub enum BEIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl BEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEIEW::_0 => false,
            BEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _BEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEIEW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::_0 => false,
            FEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIEW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AM0IE`"]
pub enum AM0IEW {
    #[doc = "Interrupt enabled."]
    _0,
    #[doc = "Interrupt disabled."]
    _1,
}
impl AM0IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AM0IEW::_0 => false,
            AM0IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AM0IEW<'a> {
    w: &'a mut W,
}
impl<'a> _AM0IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AM0IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AM0IEW::_0)
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AM0IEW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AM1F`"]
pub enum AM1FW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl AM1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AM1FW::_0 => false,
            AM1FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AM1FW<'a> {
    w: &'a mut W,
}
impl<'a> _AM1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AM1FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AM1FW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AM1FW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GCIE`"]
pub enum GCIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl GCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCIEW::_0 => false,
            GCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _GCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCIEW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SARIE`"]
pub enum SARIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SARIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SARIEW::_0 => false,
            SARIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SARIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SARIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SARIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SARIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SARIEW::_1)
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
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline]
    pub fn tdie(&self) -> TDIER {
        TDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline]
    pub fn rdie(&self) -> RDIER {
        RDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline]
    pub fn avie(&self) -> AVIER {
        AVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline]
    pub fn taie(&self) -> TAIER {
        TAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline]
    pub fn rsie(&self) -> RSIER {
        RSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline]
    pub fn sdie(&self) -> SDIER {
        SDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline]
    pub fn beie(&self) -> BEIER {
        BEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline]
    pub fn am0ie(&self) -> AM0IER {
        AM0IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline]
    pub fn am1f(&self) -> AM1FR {
        AM1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline]
    pub fn gcie(&self) -> GCIER {
        GCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline]
    pub fn sarie(&self) -> SARIER {
        SARIER::_from({
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
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline]
    pub fn tdie(&mut self) -> _TDIEW {
        _TDIEW { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline]
    pub fn rdie(&mut self) -> _RDIEW {
        _RDIEW { w: self }
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline]
    pub fn avie(&mut self) -> _AVIEW {
        _AVIEW { w: self }
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline]
    pub fn taie(&mut self) -> _TAIEW {
        _TAIEW { w: self }
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline]
    pub fn rsie(&mut self) -> _RSIEW {
        _RSIEW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline]
    pub fn sdie(&mut self) -> _SDIEW {
        _SDIEW { w: self }
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline]
    pub fn beie(&mut self) -> _BEIEW {
        _BEIEW { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline]
    pub fn am0ie(&mut self) -> _AM0IEW {
        _AM0IEW { w: self }
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline]
    pub fn am1f(&mut self) -> _AM1FW {
        _AM1FW { w: self }
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline]
    pub fn gcie(&mut self) -> _GCIEW {
        _GCIEW { w: self }
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline]
    pub fn sarie(&mut self) -> _SARIEW {
        _SARIEW { w: self }
    }
}
