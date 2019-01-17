#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERQ {
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
#[doc = "Possible values of the field `ERQ0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ0R {
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
            ERQ0R::_0 => false,
            ERQ0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ0R {
        match value {
            false => ERQ0R::_0,
            true => ERQ0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ0R::_1
    }
}
#[doc = "Possible values of the field `ERQ1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ1R {
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
            ERQ1R::_0 => false,
            ERQ1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ1R {
        match value {
            false => ERQ1R::_0,
            true => ERQ1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ1R::_1
    }
}
#[doc = "Possible values of the field `ERQ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ2R {
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
            ERQ2R::_0 => false,
            ERQ2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ2R {
        match value {
            false => ERQ2R::_0,
            true => ERQ2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ2R::_1
    }
}
#[doc = "Possible values of the field `ERQ3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ3R {
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
            ERQ3R::_0 => false,
            ERQ3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ3R {
        match value {
            false => ERQ3R::_0,
            true => ERQ3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ3R::_1
    }
}
#[doc = "Possible values of the field `ERQ4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ4R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ4R {
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
            ERQ4R::_0 => false,
            ERQ4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ4R {
        match value {
            false => ERQ4R::_0,
            true => ERQ4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ4R::_1
    }
}
#[doc = "Possible values of the field `ERQ5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ5R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ5R {
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
            ERQ5R::_0 => false,
            ERQ5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ5R {
        match value {
            false => ERQ5R::_0,
            true => ERQ5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ5R::_1
    }
}
#[doc = "Possible values of the field `ERQ6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ6R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ6R {
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
            ERQ6R::_0 => false,
            ERQ6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ6R {
        match value {
            false => ERQ6R::_0,
            true => ERQ6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ6R::_1
    }
}
#[doc = "Possible values of the field `ERQ7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ7R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ7R {
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
            ERQ7R::_0 => false,
            ERQ7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ7R {
        match value {
            false => ERQ7R::_0,
            true => ERQ7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ7R::_1
    }
}
#[doc = "Possible values of the field `ERQ8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ8R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ8R {
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
            ERQ8R::_0 => false,
            ERQ8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ8R {
        match value {
            false => ERQ8R::_0,
            true => ERQ8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ8R::_1
    }
}
#[doc = "Possible values of the field `ERQ9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ9R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ9R {
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
            ERQ9R::_0 => false,
            ERQ9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ9R {
        match value {
            false => ERQ9R::_0,
            true => ERQ9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ9R::_1
    }
}
#[doc = "Possible values of the field `ERQ10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ10R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ10R {
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
            ERQ10R::_0 => false,
            ERQ10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ10R {
        match value {
            false => ERQ10R::_0,
            true => ERQ10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ10R::_1
    }
}
#[doc = "Possible values of the field `ERQ11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ11R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ11R {
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
            ERQ11R::_0 => false,
            ERQ11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ11R {
        match value {
            false => ERQ11R::_0,
            true => ERQ11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ11R::_1
    }
}
#[doc = "Possible values of the field `ERQ12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ12R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ12R {
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
            ERQ12R::_0 => false,
            ERQ12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ12R {
        match value {
            false => ERQ12R::_0,
            true => ERQ12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ12R::_1
    }
}
#[doc = "Possible values of the field `ERQ13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ13R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ13R {
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
            ERQ13R::_0 => false,
            ERQ13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ13R {
        match value {
            false => ERQ13R::_0,
            true => ERQ13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ13R::_1
    }
}
#[doc = "Possible values of the field `ERQ14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ14R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ14R {
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
            ERQ14R::_0 => false,
            ERQ14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ14R {
        match value {
            false => ERQ14R::_0,
            true => ERQ14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ14R::_1
    }
}
#[doc = "Possible values of the field `ERQ15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ15R {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ15R {
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
            ERQ15R::_0 => false,
            ERQ15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERQ15R {
        match value {
            false => ERQ15R::_0,
            true => ERQ15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERQ15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERQ15R::_1
    }
}
#[doc = "Values that can be written to the field `ERQ0`"]
pub enum ERQ0W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ0W::_0 => false,
            ERQ0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ0W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ0W::_1)
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
#[doc = "Values that can be written to the field `ERQ1`"]
pub enum ERQ1W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ1W::_0 => false,
            ERQ1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ1W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ1W::_1)
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
#[doc = "Values that can be written to the field `ERQ2`"]
pub enum ERQ2W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ2W::_0 => false,
            ERQ2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ2W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ2W::_1)
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
#[doc = "Values that can be written to the field `ERQ3`"]
pub enum ERQ3W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ3W::_0 => false,
            ERQ3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ3W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ3W::_1)
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
#[doc = "Values that can be written to the field `ERQ4`"]
pub enum ERQ4W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ4W::_0 => false,
            ERQ4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ4W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ4W::_1)
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
#[doc = "Values that can be written to the field `ERQ5`"]
pub enum ERQ5W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ5W::_0 => false,
            ERQ5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ5W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ5W::_1)
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
#[doc = "Values that can be written to the field `ERQ6`"]
pub enum ERQ6W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ6W::_0 => false,
            ERQ6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ6W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ6W::_1)
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
#[doc = "Values that can be written to the field `ERQ7`"]
pub enum ERQ7W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ7W::_0 => false,
            ERQ7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ7W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ7W::_1)
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
#[doc = "Values that can be written to the field `ERQ8`"]
pub enum ERQ8W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ8W::_0 => false,
            ERQ8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ8W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ8W::_1)
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
#[doc = "Values that can be written to the field `ERQ9`"]
pub enum ERQ9W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ9W::_0 => false,
            ERQ9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ9W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ9W::_1)
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
#[doc = "Values that can be written to the field `ERQ10`"]
pub enum ERQ10W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ10W::_0 => false,
            ERQ10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ10W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ10W::_1)
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
#[doc = "Values that can be written to the field `ERQ11`"]
pub enum ERQ11W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ11W::_0 => false,
            ERQ11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ11W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ11W::_1)
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
#[doc = "Values that can be written to the field `ERQ12`"]
pub enum ERQ12W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ12W::_0 => false,
            ERQ12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ12W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ12W::_1)
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
#[doc = "Values that can be written to the field `ERQ13`"]
pub enum ERQ13W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ13W::_0 => false,
            ERQ13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ13W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ13W::_1)
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
#[doc = "Values that can be written to the field `ERQ14`"]
pub enum ERQ14W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ14W::_0 => false,
            ERQ14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ14W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ14W::_1)
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
#[doc = "Values that can be written to the field `ERQ15`"]
pub enum ERQ15W {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    _0,
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    _1,
}
impl ERQ15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERQ15W::_0 => false,
            ERQ15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _ERQ15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERQ15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ15W::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ15W::_1)
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&self) -> ERQ0R {
        ERQ0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&self) -> ERQ1R {
        ERQ1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&self) -> ERQ2R {
        ERQ2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&self) -> ERQ3R {
        ERQ3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline]
    pub fn erq4(&self) -> ERQ4R {
        ERQ4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline]
    pub fn erq5(&self) -> ERQ5R {
        ERQ5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline]
    pub fn erq6(&self) -> ERQ6R {
        ERQ6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline]
    pub fn erq7(&self) -> ERQ7R {
        ERQ7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline]
    pub fn erq8(&self) -> ERQ8R {
        ERQ8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline]
    pub fn erq9(&self) -> ERQ9R {
        ERQ9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline]
    pub fn erq10(&self) -> ERQ10R {
        ERQ10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline]
    pub fn erq11(&self) -> ERQ11R {
        ERQ11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline]
    pub fn erq12(&self) -> ERQ12R {
        ERQ12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline]
    pub fn erq13(&self) -> ERQ13R {
        ERQ13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline]
    pub fn erq14(&self) -> ERQ14R {
        ERQ14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline]
    pub fn erq15(&self) -> ERQ15R {
        ERQ15R::_from({
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline]
    pub fn erq0(&mut self) -> _ERQ0W {
        _ERQ0W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline]
    pub fn erq1(&mut self) -> _ERQ1W {
        _ERQ1W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline]
    pub fn erq2(&mut self) -> _ERQ2W {
        _ERQ2W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline]
    pub fn erq3(&mut self) -> _ERQ3W {
        _ERQ3W { w: self }
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline]
    pub fn erq4(&mut self) -> _ERQ4W {
        _ERQ4W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline]
    pub fn erq5(&mut self) -> _ERQ5W {
        _ERQ5W { w: self }
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline]
    pub fn erq6(&mut self) -> _ERQ6W {
        _ERQ6W { w: self }
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline]
    pub fn erq7(&mut self) -> _ERQ7W {
        _ERQ7W { w: self }
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline]
    pub fn erq8(&mut self) -> _ERQ8W {
        _ERQ8W { w: self }
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline]
    pub fn erq9(&mut self) -> _ERQ9W {
        _ERQ9W { w: self }
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline]
    pub fn erq10(&mut self) -> _ERQ10W {
        _ERQ10W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline]
    pub fn erq11(&mut self) -> _ERQ11W {
        _ERQ11W { w: self }
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline]
    pub fn erq12(&mut self) -> _ERQ12W {
        _ERQ12W { w: self }
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline]
    pub fn erq13(&mut self) -> _ERQ13W {
        _ERQ13W { w: self }
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline]
    pub fn erq14(&mut self) -> _ERQ14W {
        _ERQ14W { w: self }
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline]
    pub fn erq15(&mut self) -> _ERQ15W {
        _ERQ15W { w: self }
    }
}
