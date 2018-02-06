#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MPRA {
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
#[doc = "Possible values of the field `MPL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL2R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL2R {
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
            MPL2R::_0 => false,
            MPL2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL2R {
        match value {
            false => MPL2R::_0,
            true => MPL2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL2R::_1
    }
}
#[doc = "Possible values of the field `MTW2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW2R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW2R {
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
            MTW2R::_0 => false,
            MTW2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW2R {
        match value {
            false => MTW2R::_0,
            true => MTW2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW2R::_1
    }
}
#[doc = "Possible values of the field `MTR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR2R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR2R {
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
            MTR2R::_0 => false,
            MTR2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR2R {
        match value {
            false => MTR2R::_0,
            true => MTR2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR2R::_1
    }
}
#[doc = "Possible values of the field `MPL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL1R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL1R {
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
            MPL1R::_0 => false,
            MPL1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL1R {
        match value {
            false => MPL1R::_0,
            true => MPL1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL1R::_1
    }
}
#[doc = "Possible values of the field `MTW1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW1R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW1R {
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
            MTW1R::_0 => false,
            MTW1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW1R {
        match value {
            false => MTW1R::_0,
            true => MTW1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW1R::_1
    }
}
#[doc = "Possible values of the field `MTR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR1R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR1R {
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
            MTR1R::_0 => false,
            MTR1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR1R {
        match value {
            false => MTR1R::_0,
            true => MTR1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR1R::_1
    }
}
#[doc = "Possible values of the field `MPL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPL0R {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL0R {
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
            MPL0R::_0 => false,
            MPL0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MPL0R {
        match value {
            false => MPL0R::_0,
            true => MPL0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MPL0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MPL0R::_1
    }
}
#[doc = "Possible values of the field `MTW0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTW0R {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW0R {
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
            MTW0R::_0 => false,
            MTW0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTW0R {
        match value {
            false => MTW0R::_0,
            true => MTW0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTW0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTW0R::_1
    }
}
#[doc = "Possible values of the field `MTR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR0R {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR0R {
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
            MTR0R::_0 => false,
            MTR0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MTR0R {
        match value {
            false => MTR0R::_0,
            true => MTR0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MTR0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MTR0R::_1
    }
}
#[doc = "Values that can be written to the field `MPL2`"]
pub enum MPL2W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL2W::_0 => false,
            MPL2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL2W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL2W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL2W::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW2`"]
pub enum MTW2W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW2W::_0 => false,
            MTW2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW2W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW2W::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR2`"]
pub enum MTR2W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR2W::_0 => false,
            MTR2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR2W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR2W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR2W::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MPL1`"]
pub enum MPL1W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL1W::_0 => false,
            MPL1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL1W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL1W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL1W::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW1`"]
pub enum MTW1W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW1W::_0 => false,
            MTW1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW1W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW1W::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR1`"]
pub enum MTR1W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR1W::_0 => false,
            MTR1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR1W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR1W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR1W::_1)
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
#[doc = "Values that can be written to the field `MPL0`"]
pub enum MPL0W {
    #[doc = "Accesses from this master are forced to user-mode."]
    _0,
    #[doc = "Accesses from this master are not forced to user-mode."]
    _1,
}
impl MPL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MPL0W::_0 => false,
            MPL0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MPL0W<'a> {
    w: &'a mut W,
}
impl<'a> _MPL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MPL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from this master are forced to user-mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPL0W::_0)
    }
    #[doc = "Accesses from this master are not forced to user-mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPL0W::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTW0`"]
pub enum MTW0W {
    #[doc = "This master is not trusted for write accesses."]
    _0,
    #[doc = "This master is trusted for write accesses."]
    _1,
}
impl MTW0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTW0W::_0 => false,
            MTW0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTW0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTW0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTW0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTW0W::_0)
    }
    #[doc = "This master is trusted for write accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTW0W::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MTR0`"]
pub enum MTR0W {
    #[doc = "This master is not trusted for read accesses."]
    _0,
    #[doc = "This master is trusted for read accesses."]
    _1,
}
impl MTR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MTR0W::_0 => false,
            MTR0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MTR0W<'a> {
    w: &'a mut W,
}
impl<'a> _MTR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MTR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This master is not trusted for read accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTR0W::_0)
    }
    #[doc = "This master is trusted for read accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTR0W::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline]
    pub fn mpl2(&self) -> MPL2R {
        MPL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline]
    pub fn mtw2(&self) -> MTW2R {
        MTW2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline]
    pub fn mtr2(&self) -> MTR2R {
        MTR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline]
    pub fn mpl1(&self) -> MPL1R {
        MPL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline]
    pub fn mtw1(&self) -> MTW1R {
        MTW1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline]
    pub fn mtr1(&self) -> MTR1R {
        MTR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline]
    pub fn mpl0(&self) -> MPL0R {
        MPL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline]
    pub fn mtw0(&self) -> MTW0R {
        MTW0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline]
    pub fn mtr0(&self) -> MTR0R {
        MTR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2003828736 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 20 - Master 2 Privilege Level"]
    #[inline]
    pub fn mpl2(&mut self) -> _MPL2W {
        _MPL2W { w: self }
    }
    #[doc = "Bit 21 - Master 2 Trusted For Writes"]
    #[inline]
    pub fn mtw2(&mut self) -> _MTW2W {
        _MTW2W { w: self }
    }
    #[doc = "Bit 22 - Master 2 Trusted For Read"]
    #[inline]
    pub fn mtr2(&mut self) -> _MTR2W {
        _MTR2W { w: self }
    }
    #[doc = "Bit 24 - Master 1 Privilege Level"]
    #[inline]
    pub fn mpl1(&mut self) -> _MPL1W {
        _MPL1W { w: self }
    }
    #[doc = "Bit 25 - Master 1 Trusted for Writes"]
    #[inline]
    pub fn mtw1(&mut self) -> _MTW1W {
        _MTW1W { w: self }
    }
    #[doc = "Bit 26 - Master 1 Trusted for Read"]
    #[inline]
    pub fn mtr1(&mut self) -> _MTR1W {
        _MTR1W { w: self }
    }
    #[doc = "Bit 28 - Master 0 Privilege Level"]
    #[inline]
    pub fn mpl0(&mut self) -> _MPL0W {
        _MPL0W { w: self }
    }
    #[doc = "Bit 29 - Master 0 Trusted For Writes"]
    #[inline]
    pub fn mtw0(&mut self) -> _MTW0W {
        _MTW0W { w: self }
    }
    #[doc = "Bit 30 - Master 0 Trusted For Read"]
    #[inline]
    pub fn mtr0(&mut self) -> _MTR0W {
        _MTR0W { w: self }
    }
}
