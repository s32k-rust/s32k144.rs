#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POL {
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
#[doc = "Possible values of the field `POL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL0R {
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
            POL0R::_0 => false,
            POL0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL0R {
        match value {
            false => POL0R::_0,
            true => POL0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL0R::_1
    }
}
#[doc = "Possible values of the field `POL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL1R {
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
            POL1R::_0 => false,
            POL1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL1R {
        match value {
            false => POL1R::_0,
            true => POL1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL1R::_1
    }
}
#[doc = "Possible values of the field `POL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL2R {
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
            POL2R::_0 => false,
            POL2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL2R {
        match value {
            false => POL2R::_0,
            true => POL2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL2R::_1
    }
}
#[doc = "Possible values of the field `POL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL3R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL3R {
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
            POL3R::_0 => false,
            POL3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL3R {
        match value {
            false => POL3R::_0,
            true => POL3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL3R::_1
    }
}
#[doc = "Possible values of the field `POL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL4R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL4R {
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
            POL4R::_0 => false,
            POL4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL4R {
        match value {
            false => POL4R::_0,
            true => POL4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL4R::_1
    }
}
#[doc = "Possible values of the field `POL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL5R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL5R {
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
            POL5R::_0 => false,
            POL5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL5R {
        match value {
            false => POL5R::_0,
            true => POL5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL5R::_1
    }
}
#[doc = "Possible values of the field `POL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL6R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL6R {
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
            POL6R::_0 => false,
            POL6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL6R {
        match value {
            false => POL6R::_0,
            true => POL6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL6R::_1
    }
}
#[doc = "Possible values of the field `POL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL7R {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL7R {
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
            POL7R::_0 => false,
            POL7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> POL7R {
        match value {
            false => POL7R::_0,
            true => POL7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POL7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POL7R::_1
    }
}
#[doc = "Values that can be written to the field `POL0`"]
pub enum POL0W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL0W::_0 => false,
            POL0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL0W<'a> {
    w: &'a mut W,
}
impl<'a> _POL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL0W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL0W::_1)
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
#[doc = "Values that can be written to the field `POL1`"]
pub enum POL1W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL1W::_0 => false,
            POL1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL1W<'a> {
    w: &'a mut W,
}
impl<'a> _POL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL1W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL1W::_1)
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
#[doc = "Values that can be written to the field `POL2`"]
pub enum POL2W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL2W::_0 => false,
            POL2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL2W<'a> {
    w: &'a mut W,
}
impl<'a> _POL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL2W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL2W::_1)
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
#[doc = "Values that can be written to the field `POL3`"]
pub enum POL3W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL3W::_0 => false,
            POL3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL3W<'a> {
    w: &'a mut W,
}
impl<'a> _POL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL3W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL3W::_1)
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
#[doc = "Values that can be written to the field `POL4`"]
pub enum POL4W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL4W::_0 => false,
            POL4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL4W<'a> {
    w: &'a mut W,
}
impl<'a> _POL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL4W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL4W::_1)
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
#[doc = "Values that can be written to the field `POL5`"]
pub enum POL5W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL5W::_0 => false,
            POL5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL5W<'a> {
    w: &'a mut W,
}
impl<'a> _POL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL5W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL5W::_1)
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
#[doc = "Values that can be written to the field `POL6`"]
pub enum POL6W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL6W::_0 => false,
            POL6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL6W<'a> {
    w: &'a mut W,
}
impl<'a> _POL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL6W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL6W::_1)
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
#[doc = "Values that can be written to the field `POL7`"]
pub enum POL7W {
    #[doc = "The channel polarity is active high."]
    _0,
    #[doc = "The channel polarity is active low."]
    _1,
}
impl POL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            POL7W::_0 => false,
            POL7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POL7W<'a> {
    w: &'a mut W,
}
impl<'a> _POL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POL7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL7W::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL7W::_1)
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
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline]
    pub fn pol0(&self) -> POL0R {
        POL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline]
    pub fn pol1(&self) -> POL1R {
        POL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline]
    pub fn pol2(&self) -> POL2R {
        POL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline]
    pub fn pol3(&self) -> POL3R {
        POL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline]
    pub fn pol4(&self) -> POL4R {
        POL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline]
    pub fn pol5(&self) -> POL5R {
        POL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline]
    pub fn pol6(&self) -> POL6R {
        POL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline]
    pub fn pol7(&self) -> POL7R {
        POL7R::_from({
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
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline]
    pub fn pol0(&mut self) -> _POL0W {
        _POL0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline]
    pub fn pol1(&mut self) -> _POL1W {
        _POL1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline]
    pub fn pol2(&mut self) -> _POL2W {
        _POL2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline]
    pub fn pol3(&mut self) -> _POL3W {
        _POL3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline]
    pub fn pol4(&mut self) -> _POL4W {
        _POL4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline]
    pub fn pol5(&mut self) -> _POL5W {
        _POL5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline]
    pub fn pol6(&mut self) -> _POL6W {
        _POL6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline]
    pub fn pol7(&mut self) -> _POL7W {
        _POL7W { w: self }
    }
}
