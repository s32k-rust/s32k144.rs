#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `WCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl WCIER {
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
            WCIER::_0 => false,
            WCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCIER {
        match value {
            false => WCIER::_0,
            true => WCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WCIER::_1
    }
}
#[doc = "Possible values of the field `FCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl FCIER {
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
            FCIER::_0 => false,
            FCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCIER {
        match value {
            false => FCIER::_0,
            true => FCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FCIER::_1
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TCIER {
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
            TCIER::_0 => false,
            TCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::_0,
            true => TCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCIER::_1
    }
}
#[doc = "Possible values of the field `TEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TEIER {
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
            TEIER::_0 => false,
            TEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIER {
        match value {
            false => TEIER::_0,
            true => TEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TEIER::_1
    }
}
#[doc = "Possible values of the field `REIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl REIER {
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
            REIER::_0 => false,
            REIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REIER {
        match value {
            false => REIER::_0,
            true => REIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REIER::_1
    }
}
#[doc = "Possible values of the field `DMIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIER {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl DMIER {
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
            DMIER::_0 => false,
            DMIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMIER {
        match value {
            false => DMIER::_0,
            true => DMIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMIER::_1
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
#[doc = "Values that can be written to the field `WCIE`"]
pub enum WCIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl WCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCIEW::_0 => false,
            WCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _WCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WCIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WCIEW::_1)
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
#[doc = "Values that can be written to the field `FCIE`"]
pub enum FCIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl FCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCIEW::_0 => false,
            FCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCIEW::_1)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::_0 => false,
            TCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIEW::_1)
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
#[doc = "Values that can be written to the field `TEIE`"]
pub enum TEIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl TEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEIEW::_0 => false,
            TEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEIEW::_1)
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
#[doc = "Values that can be written to the field `REIE`"]
pub enum REIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl REIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REIEW::_0 => false,
            REIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REIEW<'a> {
    w: &'a mut W,
}
impl<'a> _REIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REIEW::_1)
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
#[doc = "Values that can be written to the field `DMIE`"]
pub enum DMIEW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl DMIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMIEW::_0 => false,
            DMIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMIEW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMIEW::_1)
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
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline]
    pub fn wcie(&self) -> WCIER {
        WCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline]
    pub fn fcie(&self) -> FCIER {
        FCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline]
    pub fn teie(&self) -> TEIER {
        TEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline]
    pub fn reie(&self) -> REIER {
        REIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&self) -> DMIER {
        DMIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline]
    pub fn wcie(&mut self) -> _WCIEW {
        _WCIEW { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline]
    pub fn fcie(&mut self) -> _FCIEW {
        _FCIEW { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline]
    pub fn teie(&mut self) -> _TEIEW {
        _TEIEW { w: self }
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline]
    pub fn reie(&mut self) -> _REIEW {
        _REIEW { w: self }
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline]
    pub fn dmie(&mut self) -> _DMIEW {
        _DMIEW { w: self }
    }
}
