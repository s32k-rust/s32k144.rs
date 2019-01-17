#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SWOCTRL {
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
#[doc = "Possible values of the field `CH0OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH0OCR {
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
            CH0OCR::_0 => false,
            CH0OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0OCR {
        match value {
            false => CH0OCR::_0,
            true => CH0OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0OCR::_1
    }
}
#[doc = "Possible values of the field `CH1OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH1OCR {
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
            CH1OCR::_0 => false,
            CH1OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1OCR {
        match value {
            false => CH1OCR::_0,
            true => CH1OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1OCR::_1
    }
}
#[doc = "Possible values of the field `CH2OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH2OCR {
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
            CH2OCR::_0 => false,
            CH2OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2OCR {
        match value {
            false => CH2OCR::_0,
            true => CH2OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2OCR::_1
    }
}
#[doc = "Possible values of the field `CH3OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH3OCR {
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
            CH3OCR::_0 => false,
            CH3OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3OCR {
        match value {
            false => CH3OCR::_0,
            true => CH3OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3OCR::_1
    }
}
#[doc = "Possible values of the field `CH4OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH4OCR {
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
            CH4OCR::_0 => false,
            CH4OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4OCR {
        match value {
            false => CH4OCR::_0,
            true => CH4OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4OCR::_1
    }
}
#[doc = "Possible values of the field `CH5OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH5OCR {
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
            CH5OCR::_0 => false,
            CH5OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5OCR {
        match value {
            false => CH5OCR::_0,
            true => CH5OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5OCR::_1
    }
}
#[doc = "Possible values of the field `CH6OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH6OCR {
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
            CH6OCR::_0 => false,
            CH6OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6OCR {
        match value {
            false => CH6OCR::_0,
            true => CH6OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6OCR::_1
    }
}
#[doc = "Possible values of the field `CH7OC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OCR {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH7OCR {
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
            CH7OCR::_0 => false,
            CH7OCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7OCR {
        match value {
            false => CH7OCR::_0,
            true => CH7OCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7OCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7OCR::_1
    }
}
#[doc = "Possible values of the field `CH0OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH0OCVR {
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
            CH0OCVR::_0 => false,
            CH0OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0OCVR {
        match value {
            false => CH0OCVR::_0,
            true => CH0OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0OCVR::_1
    }
}
#[doc = "Possible values of the field `CH1OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH1OCVR {
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
            CH1OCVR::_0 => false,
            CH1OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1OCVR {
        match value {
            false => CH1OCVR::_0,
            true => CH1OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1OCVR::_1
    }
}
#[doc = "Possible values of the field `CH2OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH2OCVR {
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
            CH2OCVR::_0 => false,
            CH2OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2OCVR {
        match value {
            false => CH2OCVR::_0,
            true => CH2OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2OCVR::_1
    }
}
#[doc = "Possible values of the field `CH3OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH3OCVR {
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
            CH3OCVR::_0 => false,
            CH3OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3OCVR {
        match value {
            false => CH3OCVR::_0,
            true => CH3OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3OCVR::_1
    }
}
#[doc = "Possible values of the field `CH4OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH4OCVR {
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
            CH4OCVR::_0 => false,
            CH4OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4OCVR {
        match value {
            false => CH4OCVR::_0,
            true => CH4OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4OCVR::_1
    }
}
#[doc = "Possible values of the field `CH5OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH5OCVR {
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
            CH5OCVR::_0 => false,
            CH5OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5OCVR {
        match value {
            false => CH5OCVR::_0,
            true => CH5OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5OCVR::_1
    }
}
#[doc = "Possible values of the field `CH6OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH6OCVR {
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
            CH6OCVR::_0 => false,
            CH6OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6OCVR {
        match value {
            false => CH6OCVR::_0,
            true => CH6OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6OCVR::_1
    }
}
#[doc = "Possible values of the field `CH7OCV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OCVR {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH7OCVR {
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
            CH7OCVR::_0 => false,
            CH7OCVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7OCVR {
        match value {
            false => CH7OCVR::_0,
            true => CH7OCVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7OCVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7OCVR::_1
    }
}
#[doc = "Values that can be written to the field `CH0OC`"]
pub enum CH0OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH0OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0OCW::_0 => false,
            CH0OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OCW::_1)
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
#[doc = "Values that can be written to the field `CH1OC`"]
pub enum CH1OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH1OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1OCW::_0 => false,
            CH1OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OCW::_1)
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
#[doc = "Values that can be written to the field `CH2OC`"]
pub enum CH2OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH2OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2OCW::_0 => false,
            CH2OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OCW::_1)
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
#[doc = "Values that can be written to the field `CH3OC`"]
pub enum CH3OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH3OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3OCW::_0 => false,
            CH3OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OCW::_1)
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
#[doc = "Values that can be written to the field `CH4OC`"]
pub enum CH4OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH4OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4OCW::_0 => false,
            CH4OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OCW::_1)
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
#[doc = "Values that can be written to the field `CH5OC`"]
pub enum CH5OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH5OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5OCW::_0 => false,
            CH5OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OCW::_1)
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
#[doc = "Values that can be written to the field `CH6OC`"]
pub enum CH6OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH6OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6OCW::_0 => false,
            CH6OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OCW::_1)
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
#[doc = "Values that can be written to the field `CH7OC`"]
pub enum CH7OCW {
    #[doc = "The channel output is not affected by software output control."]
    _0,
    #[doc = "The channel output is affected by software output control."]
    _1,
}
impl CH7OCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7OCW::_0 => false,
            CH7OCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7OCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7OCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7OCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OCW::_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OCW::_1)
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
#[doc = "Values that can be written to the field `CH0OCV`"]
pub enum CH0OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH0OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0OCVW::_0 => false,
            CH0OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OCVW::_1)
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
#[doc = "Values that can be written to the field `CH1OCV`"]
pub enum CH1OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH1OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1OCVW::_0 => false,
            CH1OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OCVW::_1)
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
#[doc = "Values that can be written to the field `CH2OCV`"]
pub enum CH2OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH2OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2OCVW::_0 => false,
            CH2OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OCVW::_1)
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
#[doc = "Values that can be written to the field `CH3OCV`"]
pub enum CH3OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH3OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3OCVW::_0 => false,
            CH3OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OCVW::_1)
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
#[doc = "Values that can be written to the field `CH4OCV`"]
pub enum CH4OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH4OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4OCVW::_0 => false,
            CH4OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OCVW::_1)
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
#[doc = "Values that can be written to the field `CH5OCV`"]
pub enum CH5OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH5OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5OCVW::_0 => false,
            CH5OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OCVW::_1)
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
#[doc = "Values that can be written to the field `CH6OCV`"]
pub enum CH6OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH6OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6OCVW::_0 => false,
            CH6OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OCVW::_1)
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
#[doc = "Values that can be written to the field `CH7OCV`"]
pub enum CH7OCVW {
    #[doc = "The software output control forces 0 to the channel output."]
    _0,
    #[doc = "The software output control forces 1 to the channel output."]
    _1,
}
impl CH7OCVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7OCVW::_0 => false,
            CH7OCVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7OCVW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7OCVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7OCVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OCVW::_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OCVW::_1)
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
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline]
    pub fn ch0oc(&self) -> CH0OCR {
        CH0OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline]
    pub fn ch1oc(&self) -> CH1OCR {
        CH1OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline]
    pub fn ch2oc(&self) -> CH2OCR {
        CH2OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline]
    pub fn ch3oc(&self) -> CH3OCR {
        CH3OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline]
    pub fn ch4oc(&self) -> CH4OCR {
        CH4OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline]
    pub fn ch5oc(&self) -> CH5OCR {
        CH5OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline]
    pub fn ch6oc(&self) -> CH6OCR {
        CH6OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline]
    pub fn ch7oc(&self) -> CH7OCR {
        CH7OCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline]
    pub fn ch0ocv(&self) -> CH0OCVR {
        CH0OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline]
    pub fn ch1ocv(&self) -> CH1OCVR {
        CH1OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline]
    pub fn ch2ocv(&self) -> CH2OCVR {
        CH2OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline]
    pub fn ch3ocv(&self) -> CH3OCVR {
        CH3OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline]
    pub fn ch4ocv(&self) -> CH4OCVR {
        CH4OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline]
    pub fn ch5ocv(&self) -> CH5OCVR {
        CH5OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline]
    pub fn ch6ocv(&self) -> CH6OCVR {
        CH6OCVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline]
    pub fn ch7ocv(&self) -> CH7OCVR {
        CH7OCVR::_from({
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
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline]
    pub fn ch0oc(&mut self) -> _CH0OCW {
        _CH0OCW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline]
    pub fn ch1oc(&mut self) -> _CH1OCW {
        _CH1OCW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline]
    pub fn ch2oc(&mut self) -> _CH2OCW {
        _CH2OCW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline]
    pub fn ch3oc(&mut self) -> _CH3OCW {
        _CH3OCW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline]
    pub fn ch4oc(&mut self) -> _CH4OCW {
        _CH4OCW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline]
    pub fn ch5oc(&mut self) -> _CH5OCW {
        _CH5OCW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline]
    pub fn ch6oc(&mut self) -> _CH6OCW {
        _CH6OCW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline]
    pub fn ch7oc(&mut self) -> _CH7OCW {
        _CH7OCW { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline]
    pub fn ch0ocv(&mut self) -> _CH0OCVW {
        _CH0OCVW { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline]
    pub fn ch1ocv(&mut self) -> _CH1OCVW {
        _CH1OCVW { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline]
    pub fn ch2ocv(&mut self) -> _CH2OCVW {
        _CH2OCVW { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline]
    pub fn ch3ocv(&mut self) -> _CH3OCVW {
        _CH3OCVW { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline]
    pub fn ch4ocv(&mut self) -> _CH4OCVW {
        _CH4OCVW { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline]
    pub fn ch5ocv(&mut self) -> _CH5OCVW {
        _CH5OCVW { w: self }
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline]
    pub fn ch6ocv(&mut self) -> _CH6OCVW {
        _CH6OCVW { w: self }
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline]
    pub fn ch7ocv(&mut self) -> _CH7OCVW {
        _CH7OCVW { w: self }
    }
}
