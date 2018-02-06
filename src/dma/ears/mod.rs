#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EARS {
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
#[doc = "Possible values of the field `EDREQ_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_0R {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0R {
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
            EDREQ_0R::_0 => false,
            EDREQ_0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_0R {
        match value {
            false => EDREQ_0R::_0,
            true => EDREQ_0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_0R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_1R {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1R {
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
            EDREQ_1R::_0 => false,
            EDREQ_1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_1R {
        match value {
            false => EDREQ_1R::_0,
            true => EDREQ_1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_1R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_2R {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2R {
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
            EDREQ_2R::_0 => false,
            EDREQ_2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_2R {
        match value {
            false => EDREQ_2R::_0,
            true => EDREQ_2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_2R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_3R {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3R {
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
            EDREQ_3R::_0 => false,
            EDREQ_3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_3R {
        match value {
            false => EDREQ_3R::_0,
            true => EDREQ_3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_3R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_4R {
    #[doc = "Disable asynchronous DMA request for channel 4."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 4."]
    _1,
}
impl EDREQ_4R {
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
            EDREQ_4R::_0 => false,
            EDREQ_4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_4R {
        match value {
            false => EDREQ_4R::_0,
            true => EDREQ_4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_4R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_5R {
    #[doc = "Disable asynchronous DMA request for channel 5."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 5."]
    _1,
}
impl EDREQ_5R {
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
            EDREQ_5R::_0 => false,
            EDREQ_5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_5R {
        match value {
            false => EDREQ_5R::_0,
            true => EDREQ_5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_5R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_6R {
    #[doc = "Disable asynchronous DMA request for channel 6."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 6."]
    _1,
}
impl EDREQ_6R {
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
            EDREQ_6R::_0 => false,
            EDREQ_6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_6R {
        match value {
            false => EDREQ_6R::_0,
            true => EDREQ_6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_6R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_7R {
    #[doc = "Disable asynchronous DMA request for channel 7."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 7."]
    _1,
}
impl EDREQ_7R {
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
            EDREQ_7R::_0 => false,
            EDREQ_7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_7R {
        match value {
            false => EDREQ_7R::_0,
            true => EDREQ_7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_7R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_8R {
    #[doc = "Disable asynchronous DMA request for channel 8."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 8."]
    _1,
}
impl EDREQ_8R {
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
            EDREQ_8R::_0 => false,
            EDREQ_8R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_8R {
        match value {
            false => EDREQ_8R::_0,
            true => EDREQ_8R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_8R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_8R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_9R {
    #[doc = "Disable asynchronous DMA request for channel 9."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 9."]
    _1,
}
impl EDREQ_9R {
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
            EDREQ_9R::_0 => false,
            EDREQ_9R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_9R {
        match value {
            false => EDREQ_9R::_0,
            true => EDREQ_9R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_9R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_9R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_10R {
    #[doc = "Disable asynchronous DMA request for channel 10."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 10."]
    _1,
}
impl EDREQ_10R {
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
            EDREQ_10R::_0 => false,
            EDREQ_10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_10R {
        match value {
            false => EDREQ_10R::_0,
            true => EDREQ_10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_10R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_11R {
    #[doc = "Disable asynchronous DMA request for channel 11."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 11."]
    _1,
}
impl EDREQ_11R {
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
            EDREQ_11R::_0 => false,
            EDREQ_11R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_11R {
        match value {
            false => EDREQ_11R::_0,
            true => EDREQ_11R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_11R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_11R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_12R {
    #[doc = "Disable asynchronous DMA request for channel 12."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 12."]
    _1,
}
impl EDREQ_12R {
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
            EDREQ_12R::_0 => false,
            EDREQ_12R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_12R {
        match value {
            false => EDREQ_12R::_0,
            true => EDREQ_12R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_12R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_12R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_13R {
    #[doc = "Disable asynchronous DMA request for channel 13."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 13."]
    _1,
}
impl EDREQ_13R {
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
            EDREQ_13R::_0 => false,
            EDREQ_13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_13R {
        match value {
            false => EDREQ_13R::_0,
            true => EDREQ_13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_13R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_14R {
    #[doc = "Disable asynchronous DMA request for channel 14."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 14."]
    _1,
}
impl EDREQ_14R {
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
            EDREQ_14R::_0 => false,
            EDREQ_14R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_14R {
        match value {
            false => EDREQ_14R::_0,
            true => EDREQ_14R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_14R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_14R::_1
    }
}
#[doc = "Possible values of the field `EDREQ_15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDREQ_15R {
    #[doc = "Disable asynchronous DMA request for channel 15."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 15."]
    _1,
}
impl EDREQ_15R {
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
            EDREQ_15R::_0 => false,
            EDREQ_15R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EDREQ_15R {
        match value {
            false => EDREQ_15R::_0,
            true => EDREQ_15R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_15R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_15R::_1
    }
}
#[doc = "Values that can be written to the field `EDREQ_0`"]
pub enum EDREQ_0W {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 0."]
    _1,
}
impl EDREQ_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_0W::_0 => false,
            EDREQ_0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_0W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_1`"]
pub enum EDREQ_1W {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 1."]
    _1,
}
impl EDREQ_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_1W::_0 => false,
            EDREQ_1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_1W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_2`"]
pub enum EDREQ_2W {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 2."]
    _1,
}
impl EDREQ_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_2W::_0 => false,
            EDREQ_2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_2W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_3`"]
pub enum EDREQ_3W {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 3."]
    _1,
}
impl EDREQ_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_3W::_0 => false,
            EDREQ_3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_3W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_4`"]
pub enum EDREQ_4W {
    #[doc = "Disable asynchronous DMA request for channel 4."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 4."]
    _1,
}
impl EDREQ_4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_4W::_0 => false,
            EDREQ_4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_4W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 4."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_4W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 4."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_4W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_5`"]
pub enum EDREQ_5W {
    #[doc = "Disable asynchronous DMA request for channel 5."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 5."]
    _1,
}
impl EDREQ_5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_5W::_0 => false,
            EDREQ_5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_5W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 5."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_5W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 5."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_5W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_6`"]
pub enum EDREQ_6W {
    #[doc = "Disable asynchronous DMA request for channel 6."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 6."]
    _1,
}
impl EDREQ_6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_6W::_0 => false,
            EDREQ_6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_6W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 6."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_6W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 6."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_6W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_7`"]
pub enum EDREQ_7W {
    #[doc = "Disable asynchronous DMA request for channel 7."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 7."]
    _1,
}
impl EDREQ_7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_7W::_0 => false,
            EDREQ_7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_7W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 7."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_7W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 7."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_7W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_8`"]
pub enum EDREQ_8W {
    #[doc = "Disable asynchronous DMA request for channel 8."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 8."]
    _1,
}
impl EDREQ_8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_8W::_0 => false,
            EDREQ_8W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_8W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 8."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_8W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 8."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_8W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_9`"]
pub enum EDREQ_9W {
    #[doc = "Disable asynchronous DMA request for channel 9."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 9."]
    _1,
}
impl EDREQ_9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_9W::_0 => false,
            EDREQ_9W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_9W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 9."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_9W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 9."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_9W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_10`"]
pub enum EDREQ_10W {
    #[doc = "Disable asynchronous DMA request for channel 10."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 10."]
    _1,
}
impl EDREQ_10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_10W::_0 => false,
            EDREQ_10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_10W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 10."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_10W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 10."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_10W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_11`"]
pub enum EDREQ_11W {
    #[doc = "Disable asynchronous DMA request for channel 11."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 11."]
    _1,
}
impl EDREQ_11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_11W::_0 => false,
            EDREQ_11W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_11W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 11."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_11W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 11."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_11W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_12`"]
pub enum EDREQ_12W {
    #[doc = "Disable asynchronous DMA request for channel 12."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 12."]
    _1,
}
impl EDREQ_12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_12W::_0 => false,
            EDREQ_12W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_12W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 12."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_12W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 12."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_12W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_13`"]
pub enum EDREQ_13W {
    #[doc = "Disable asynchronous DMA request for channel 13."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 13."]
    _1,
}
impl EDREQ_13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_13W::_0 => false,
            EDREQ_13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_13W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 13."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_13W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 13."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_13W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_14`"]
pub enum EDREQ_14W {
    #[doc = "Disable asynchronous DMA request for channel 14."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 14."]
    _1,
}
impl EDREQ_14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_14W::_0 => false,
            EDREQ_14W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_14W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 14."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_14W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 14."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_14W::_1)
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
#[doc = "Values that can be written to the field `EDREQ_15`"]
pub enum EDREQ_15W {
    #[doc = "Disable asynchronous DMA request for channel 15."]
    _0,
    #[doc = "Enable asynchronous DMA request for channel 15."]
    _1,
}
impl EDREQ_15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EDREQ_15W::_0 => false,
            EDREQ_15W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EDREQ_15W<'a> {
    w: &'a mut W,
}
impl<'a> _EDREQ_15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EDREQ_15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable asynchronous DMA request for channel 15."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_15W::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 15."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_15W::_1)
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&self) -> EDREQ_0R {
        EDREQ_0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&self) -> EDREQ_1R {
        EDREQ_1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&self) -> EDREQ_2R {
        EDREQ_2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&self) -> EDREQ_3R {
        EDREQ_3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline]
    pub fn edreq_4(&self) -> EDREQ_4R {
        EDREQ_4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline]
    pub fn edreq_5(&self) -> EDREQ_5R {
        EDREQ_5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline]
    pub fn edreq_6(&self) -> EDREQ_6R {
        EDREQ_6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline]
    pub fn edreq_7(&self) -> EDREQ_7R {
        EDREQ_7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline]
    pub fn edreq_8(&self) -> EDREQ_8R {
        EDREQ_8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline]
    pub fn edreq_9(&self) -> EDREQ_9R {
        EDREQ_9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline]
    pub fn edreq_10(&self) -> EDREQ_10R {
        EDREQ_10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline]
    pub fn edreq_11(&self) -> EDREQ_11R {
        EDREQ_11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline]
    pub fn edreq_12(&self) -> EDREQ_12R {
        EDREQ_12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline]
    pub fn edreq_13(&self) -> EDREQ_13R {
        EDREQ_13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline]
    pub fn edreq_14(&self) -> EDREQ_14R {
        EDREQ_14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline]
    pub fn edreq_15(&self) -> EDREQ_15R {
        EDREQ_15R::_from({
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
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline]
    pub fn edreq_0(&mut self) -> _EDREQ_0W {
        _EDREQ_0W { w: self }
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline]
    pub fn edreq_1(&mut self) -> _EDREQ_1W {
        _EDREQ_1W { w: self }
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline]
    pub fn edreq_2(&mut self) -> _EDREQ_2W {
        _EDREQ_2W { w: self }
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline]
    pub fn edreq_3(&mut self) -> _EDREQ_3W {
        _EDREQ_3W { w: self }
    }
    #[doc = "Bit 4 - Enable asynchronous DMA request in stop mode for channel 4"]
    #[inline]
    pub fn edreq_4(&mut self) -> _EDREQ_4W {
        _EDREQ_4W { w: self }
    }
    #[doc = "Bit 5 - Enable asynchronous DMA request in stop mode for channel 5"]
    #[inline]
    pub fn edreq_5(&mut self) -> _EDREQ_5W {
        _EDREQ_5W { w: self }
    }
    #[doc = "Bit 6 - Enable asynchronous DMA request in stop mode for channel 6"]
    #[inline]
    pub fn edreq_6(&mut self) -> _EDREQ_6W {
        _EDREQ_6W { w: self }
    }
    #[doc = "Bit 7 - Enable asynchronous DMA request in stop mode for channel 7"]
    #[inline]
    pub fn edreq_7(&mut self) -> _EDREQ_7W {
        _EDREQ_7W { w: self }
    }
    #[doc = "Bit 8 - Enable asynchronous DMA request in stop mode for channel 8"]
    #[inline]
    pub fn edreq_8(&mut self) -> _EDREQ_8W {
        _EDREQ_8W { w: self }
    }
    #[doc = "Bit 9 - Enable asynchronous DMA request in stop mode for channel 9"]
    #[inline]
    pub fn edreq_9(&mut self) -> _EDREQ_9W {
        _EDREQ_9W { w: self }
    }
    #[doc = "Bit 10 - Enable asynchronous DMA request in stop mode for channel 10"]
    #[inline]
    pub fn edreq_10(&mut self) -> _EDREQ_10W {
        _EDREQ_10W { w: self }
    }
    #[doc = "Bit 11 - Enable asynchronous DMA request in stop mode for channel 11"]
    #[inline]
    pub fn edreq_11(&mut self) -> _EDREQ_11W {
        _EDREQ_11W { w: self }
    }
    #[doc = "Bit 12 - Enable asynchronous DMA request in stop mode for channel 12"]
    #[inline]
    pub fn edreq_12(&mut self) -> _EDREQ_12W {
        _EDREQ_12W { w: self }
    }
    #[doc = "Bit 13 - Enable asynchronous DMA request in stop mode for channel 13"]
    #[inline]
    pub fn edreq_13(&mut self) -> _EDREQ_13W {
        _EDREQ_13W { w: self }
    }
    #[doc = "Bit 14 - Enable asynchronous DMA request in stop mode for channel 14"]
    #[inline]
    pub fn edreq_14(&mut self) -> _EDREQ_14W {
        _EDREQ_14W { w: self }
    }
    #[doc = "Bit 15 - Enable asynchronous DMA request in stop mode for channel 15"]
    #[inline]
    pub fn edreq_15(&mut self) -> _EDREQ_15W {
        _EDREQ_15W { w: self }
    }
}
