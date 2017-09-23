#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::COMBINE {
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
#[doc = r" Value of the field"]
pub struct COMBINE0R {
    bits: bool,
}
impl COMBINE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `COMP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP0R {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP0R {
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
            COMP0R::_0 => false,
            COMP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMP0R {
        match value {
            false => COMP0R::_0,
            true => COMP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMP0R::_1
    }
}
#[doc = r" Value of the field"]
pub struct DECAPEN0R {
    bits: bool,
}
impl DECAPEN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DECAP0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP0R {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP0R {
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
            DECAP0R::_0 => false,
            DECAP0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DECAP0R {
        match value {
            false => DECAP0R::_0,
            true => DECAP0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DECAP0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DECAP0R::_1
    }
}
#[doc = "Possible values of the field `DTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN0R {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN0R {
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
            DTEN0R::_0 => false,
            DTEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEN0R {
        match value {
            false => DTEN0R::_0,
            true => DTEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEN0R::_1
    }
}
#[doc = "Possible values of the field `SYNCEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN0R {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN0R {
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
            SYNCEN0R::_0 => false,
            SYNCEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCEN0R {
        match value {
            false => SYNCEN0R::_0,
            true => SYNCEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN0R::_1
    }
}
#[doc = "Possible values of the field `FAULTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN0R {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN0R {
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
            FAULTEN0R::_0 => false,
            FAULTEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTEN0R {
        match value {
            false => FAULTEN0R::_0,
            true => FAULTEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN0R::_1
    }
}
#[doc = r" Value of the field"]
pub struct MCOMBINE0R {
    bits: bool,
}
impl MCOMBINE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct COMBINE1R {
    bits: bool,
}
impl COMBINE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `COMP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1R {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP1R {
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
            COMP1R::_0 => false,
            COMP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMP1R {
        match value {
            false => COMP1R::_0,
            true => COMP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMP1R::_1
    }
}
#[doc = r" Value of the field"]
pub struct DECAPEN1R {
    bits: bool,
}
impl DECAPEN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DECAP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP1R {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP1R {
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
            DECAP1R::_0 => false,
            DECAP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DECAP1R {
        match value {
            false => DECAP1R::_0,
            true => DECAP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DECAP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DECAP1R::_1
    }
}
#[doc = "Possible values of the field `DTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN1R {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN1R {
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
            DTEN1R::_0 => false,
            DTEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEN1R {
        match value {
            false => DTEN1R::_0,
            true => DTEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEN1R::_1
    }
}
#[doc = "Possible values of the field `SYNCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN1R {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN1R {
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
            SYNCEN1R::_0 => false,
            SYNCEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCEN1R {
        match value {
            false => SYNCEN1R::_0,
            true => SYNCEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN1R::_1
    }
}
#[doc = "Possible values of the field `FAULTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN1R {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN1R {
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
            FAULTEN1R::_0 => false,
            FAULTEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTEN1R {
        match value {
            false => FAULTEN1R::_0,
            true => FAULTEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN1R::_1
    }
}
#[doc = r" Value of the field"]
pub struct MCOMBINE1R {
    bits: bool,
}
impl MCOMBINE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct COMBINE2R {
    bits: bool,
}
impl COMBINE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `COMP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2R {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP2R {
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
            COMP2R::_0 => false,
            COMP2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMP2R {
        match value {
            false => COMP2R::_0,
            true => COMP2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMP2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMP2R::_1
    }
}
#[doc = r" Value of the field"]
pub struct DECAPEN2R {
    bits: bool,
}
impl DECAPEN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DECAP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP2R {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP2R {
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
            DECAP2R::_0 => false,
            DECAP2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DECAP2R {
        match value {
            false => DECAP2R::_0,
            true => DECAP2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DECAP2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DECAP2R::_1
    }
}
#[doc = "Possible values of the field `DTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN2R {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN2R {
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
            DTEN2R::_0 => false,
            DTEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEN2R {
        match value {
            false => DTEN2R::_0,
            true => DTEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEN2R::_1
    }
}
#[doc = "Possible values of the field `SYNCEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN2R {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN2R {
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
            SYNCEN2R::_0 => false,
            SYNCEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCEN2R {
        match value {
            false => SYNCEN2R::_0,
            true => SYNCEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN2R::_1
    }
}
#[doc = "Possible values of the field `FAULTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN2R {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN2R {
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
            FAULTEN2R::_0 => false,
            FAULTEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTEN2R {
        match value {
            false => FAULTEN2R::_0,
            true => FAULTEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN2R::_1
    }
}
#[doc = r" Value of the field"]
pub struct MCOMBINE2R {
    bits: bool,
}
impl MCOMBINE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct COMBINE3R {
    bits: bool,
}
impl COMBINE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `COMP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP3R {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP3R {
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
            COMP3R::_0 => false,
            COMP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMP3R {
        match value {
            false => COMP3R::_0,
            true => COMP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COMP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COMP3R::_1
    }
}
#[doc = r" Value of the field"]
pub struct DECAPEN3R {
    bits: bool,
}
impl DECAPEN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DECAP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP3R {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP3R {
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
            DECAP3R::_0 => false,
            DECAP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DECAP3R {
        match value {
            false => DECAP3R::_0,
            true => DECAP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DECAP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DECAP3R::_1
    }
}
#[doc = "Possible values of the field `DTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN3R {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN3R {
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
            DTEN3R::_0 => false,
            DTEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DTEN3R {
        match value {
            false => DTEN3R::_0,
            true => DTEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DTEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DTEN3R::_1
    }
}
#[doc = "Possible values of the field `SYNCEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN3R {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN3R {
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
            SYNCEN3R::_0 => false,
            SYNCEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCEN3R {
        match value {
            false => SYNCEN3R::_0,
            true => SYNCEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCEN3R::_1
    }
}
#[doc = "Possible values of the field `FAULTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN3R {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN3R {
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
            FAULTEN3R::_0 => false,
            FAULTEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAULTEN3R {
        match value {
            false => FAULTEN3R::_0,
            true => FAULTEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FAULTEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FAULTEN3R::_1
    }
}
#[doc = r" Value of the field"]
pub struct MCOMBINE3R {
    bits: bool,
}
impl MCOMBINE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _COMBINE0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE0W<'a> {
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
#[doc = "Values that can be written to the field `COMP0`"]
pub enum COMP0W {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMP0W::_0 => false,
            COMP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP0W::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP0W::_1)
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
#[doc = r" Proxy"]
pub struct _DECAPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAPEN0W<'a> {
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
#[doc = "Values that can be written to the field `DECAP0`"]
pub enum DECAP0W {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DECAP0W::_0 => false,
            DECAP0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECAP0W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAP0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECAP0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP0W::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP0W::_1)
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
#[doc = "Values that can be written to the field `DTEN0`"]
pub enum DTEN0W {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEN0W::_0 => false,
            DTEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _DTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN0W::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN0W::_1)
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
#[doc = "Values that can be written to the field `SYNCEN0`"]
pub enum SYNCEN0W {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCEN0W::_0 => false,
            SYNCEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN0W::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN0W::_1)
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
#[doc = "Values that can be written to the field `FAULTEN0`"]
pub enum FAULTEN0W {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTEN0W::_0 => false,
            FAULTEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN0W::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN0W::_1)
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
#[doc = r" Proxy"]
pub struct _MCOMBINE0W<'a> {
    w: &'a mut W,
}
impl<'a> _MCOMBINE0W<'a> {
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
pub struct _COMBINE1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE1W<'a> {
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
#[doc = "Values that can be written to the field `COMP1`"]
pub enum COMP1W {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMP1W::_0 => false,
            COMP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP1W::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP1W::_1)
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
#[doc = r" Proxy"]
pub struct _DECAPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAPEN1W<'a> {
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
#[doc = "Values that can be written to the field `DECAP1`"]
pub enum DECAP1W {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DECAP1W::_0 => false,
            DECAP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECAP1W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECAP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP1W::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP1W::_1)
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
#[doc = "Values that can be written to the field `DTEN1`"]
pub enum DTEN1W {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEN1W::_0 => false,
            DTEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _DTEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN1W::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN1W::_1)
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
#[doc = "Values that can be written to the field `SYNCEN1`"]
pub enum SYNCEN1W {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCEN1W::_0 => false,
            SYNCEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN1W::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN1W::_1)
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
#[doc = "Values that can be written to the field `FAULTEN1`"]
pub enum FAULTEN1W {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTEN1W::_0 => false,
            FAULTEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN1W::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN1W::_1)
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
#[doc = r" Proxy"]
pub struct _MCOMBINE1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCOMBINE1W<'a> {
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
#[doc = r" Proxy"]
pub struct _COMBINE2W<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE2W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMP2`"]
pub enum COMP2W {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMP2W::_0 => false,
            COMP2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP2W::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP2W::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DECAPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAPEN2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DECAP2`"]
pub enum DECAP2W {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DECAP2W::_0 => false,
            DECAP2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECAP2W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECAP2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP2W::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP2W::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTEN2`"]
pub enum DTEN2W {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEN2W::_0 => false,
            DTEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _DTEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN2W::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN2W::_1)
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
#[doc = "Values that can be written to the field `SYNCEN2`"]
pub enum SYNCEN2W {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCEN2W::_0 => false,
            SYNCEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN2W::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN2W::_1)
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
#[doc = "Values that can be written to the field `FAULTEN2`"]
pub enum FAULTEN2W {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTEN2W::_0 => false,
            FAULTEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN2W::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN2W::_1)
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
#[doc = r" Proxy"]
pub struct _MCOMBINE2W<'a> {
    w: &'a mut W,
}
impl<'a> _MCOMBINE2W<'a> {
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
#[doc = r" Proxy"]
pub struct _COMBINE3W<'a> {
    w: &'a mut W,
}
impl<'a> _COMBINE3W<'a> {
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
#[doc = "Values that can be written to the field `COMP3`"]
pub enum COMP3W {
    #[doc = "The channel (n+1) output is the same as the channel (n) output."] _0,
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."] _1,
}
impl COMP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMP3W::_0 => false,
            COMP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP3W::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP3W::_1)
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
#[doc = r" Proxy"]
pub struct _DECAPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAPEN3W<'a> {
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
#[doc = "Values that can be written to the field `DECAP3`"]
pub enum DECAP3W {
    #[doc = "The dual edge captures are inactive."] _0,
    #[doc = "The dual edge captures are active."] _1,
}
impl DECAP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DECAP3W::_0 => false,
            DECAP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DECAP3W<'a> {
    w: &'a mut W,
}
impl<'a> _DECAP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DECAP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP3W::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP3W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTEN3`"]
pub enum DTEN3W {
    #[doc = "The deadtime insertion in this pair of channels is disabled."] _0,
    #[doc = "The deadtime insertion in this pair of channels is enabled."] _1,
}
impl DTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DTEN3W::_0 => false,
            DTEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _DTEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN3W::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN3W::_1)
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
#[doc = "Values that can be written to the field `SYNCEN3`"]
pub enum SYNCEN3W {
    #[doc = "The PWM synchronization in this pair of channels is disabled."] _0,
    #[doc = "The PWM synchronization in this pair of channels is enabled."] _1,
}
impl SYNCEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCEN3W::_0 => false,
            SYNCEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN3W::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN3W::_1)
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
#[doc = "Values that can be written to the field `FAULTEN3`"]
pub enum FAULTEN3W {
    #[doc = "The fault control in this pair of channels is disabled."] _0,
    #[doc = "The fault control in this pair of channels is enabled."] _1,
}
impl FAULTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FAULTEN3W::_0 => false,
            FAULTEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FAULTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _FAULTEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAULTEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN3W::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN3W::_1)
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
#[doc = r" Proxy"]
pub struct _MCOMBINE3W<'a> {
    w: &'a mut W,
}
impl<'a> _MCOMBINE3W<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline]
    pub fn combine0(&self) -> COMBINE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBINE0R { bits }
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline]
    pub fn comp0(&self) -> COMP0R {
        COMP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline]
    pub fn decapen0(&self) -> DECAPEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECAPEN0R { bits }
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline]
    pub fn decap0(&self) -> DECAP0R {
        DECAP0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline]
    pub fn dten0(&self) -> DTEN0R {
        DTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline]
    pub fn syncen0(&self) -> SYNCEN0R {
        SYNCEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline]
    pub fn faulten0(&self) -> FAULTEN0R {
        FAULTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline]
    pub fn mcombine0(&self) -> MCOMBINE0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCOMBINE0R { bits }
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline]
    pub fn combine1(&self) -> COMBINE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBINE1R { bits }
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline]
    pub fn comp1(&self) -> COMP1R {
        COMP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline]
    pub fn decapen1(&self) -> DECAPEN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECAPEN1R { bits }
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline]
    pub fn decap1(&self) -> DECAP1R {
        DECAP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline]
    pub fn dten1(&self) -> DTEN1R {
        DTEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline]
    pub fn syncen1(&self) -> SYNCEN1R {
        SYNCEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline]
    pub fn faulten1(&self) -> FAULTEN1R {
        FAULTEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline]
    pub fn mcombine1(&self) -> MCOMBINE1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCOMBINE1R { bits }
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline]
    pub fn combine2(&self) -> COMBINE2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBINE2R { bits }
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline]
    pub fn comp2(&self) -> COMP2R {
        COMP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline]
    pub fn decapen2(&self) -> DECAPEN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECAPEN2R { bits }
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline]
    pub fn decap2(&self) -> DECAP2R {
        DECAP2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline]
    pub fn dten2(&self) -> DTEN2R {
        DTEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline]
    pub fn syncen2(&self) -> SYNCEN2R {
        SYNCEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline]
    pub fn faulten2(&self) -> FAULTEN2R {
        FAULTEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline]
    pub fn mcombine2(&self) -> MCOMBINE2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCOMBINE2R { bits }
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline]
    pub fn combine3(&self) -> COMBINE3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMBINE3R { bits }
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline]
    pub fn comp3(&self) -> COMP3R {
        COMP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline]
    pub fn decapen3(&self) -> DECAPEN3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DECAPEN3R { bits }
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline]
    pub fn decap3(&self) -> DECAP3R {
        DECAP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline]
    pub fn dten3(&self) -> DTEN3R {
        DTEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline]
    pub fn syncen3(&self) -> SYNCEN3R {
        SYNCEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline]
    pub fn faulten3(&self) -> FAULTEN3R {
        FAULTEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline]
    pub fn mcombine3(&self) -> MCOMBINE3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MCOMBINE3R { bits }
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
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline]
    pub fn combine0(&mut self) -> _COMBINE0W {
        _COMBINE0W { w: self }
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline]
    pub fn comp0(&mut self) -> _COMP0W {
        _COMP0W { w: self }
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline]
    pub fn decapen0(&mut self) -> _DECAPEN0W {
        _DECAPEN0W { w: self }
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline]
    pub fn decap0(&mut self) -> _DECAP0W {
        _DECAP0W { w: self }
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline]
    pub fn dten0(&mut self) -> _DTEN0W {
        _DTEN0W { w: self }
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline]
    pub fn syncen0(&mut self) -> _SYNCEN0W {
        _SYNCEN0W { w: self }
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline]
    pub fn faulten0(&mut self) -> _FAULTEN0W {
        _FAULTEN0W { w: self }
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline]
    pub fn mcombine0(&mut self) -> _MCOMBINE0W {
        _MCOMBINE0W { w: self }
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline]
    pub fn combine1(&mut self) -> _COMBINE1W {
        _COMBINE1W { w: self }
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline]
    pub fn comp1(&mut self) -> _COMP1W {
        _COMP1W { w: self }
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline]
    pub fn decapen1(&mut self) -> _DECAPEN1W {
        _DECAPEN1W { w: self }
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline]
    pub fn decap1(&mut self) -> _DECAP1W {
        _DECAP1W { w: self }
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline]
    pub fn dten1(&mut self) -> _DTEN1W {
        _DTEN1W { w: self }
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline]
    pub fn syncen1(&mut self) -> _SYNCEN1W {
        _SYNCEN1W { w: self }
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline]
    pub fn faulten1(&mut self) -> _FAULTEN1W {
        _FAULTEN1W { w: self }
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline]
    pub fn mcombine1(&mut self) -> _MCOMBINE1W {
        _MCOMBINE1W { w: self }
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline]
    pub fn combine2(&mut self) -> _COMBINE2W {
        _COMBINE2W { w: self }
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline]
    pub fn comp2(&mut self) -> _COMP2W {
        _COMP2W { w: self }
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline]
    pub fn decapen2(&mut self) -> _DECAPEN2W {
        _DECAPEN2W { w: self }
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline]
    pub fn decap2(&mut self) -> _DECAP2W {
        _DECAP2W { w: self }
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline]
    pub fn dten2(&mut self) -> _DTEN2W {
        _DTEN2W { w: self }
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline]
    pub fn syncen2(&mut self) -> _SYNCEN2W {
        _SYNCEN2W { w: self }
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline]
    pub fn faulten2(&mut self) -> _FAULTEN2W {
        _FAULTEN2W { w: self }
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline]
    pub fn mcombine2(&mut self) -> _MCOMBINE2W {
        _MCOMBINE2W { w: self }
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline]
    pub fn combine3(&mut self) -> _COMBINE3W {
        _COMBINE3W { w: self }
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline]
    pub fn comp3(&mut self) -> _COMP3W {
        _COMP3W { w: self }
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline]
    pub fn decapen3(&mut self) -> _DECAPEN3W {
        _DECAPEN3W { w: self }
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline]
    pub fn decap3(&mut self) -> _DECAP3W {
        _DECAP3W { w: self }
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline]
    pub fn dten3(&mut self) -> _DTEN3W {
        _DTEN3W { w: self }
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline]
    pub fn syncen3(&mut self) -> _SYNCEN3W {
        _SYNCEN3W { w: self }
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline]
    pub fn faulten3(&mut self) -> _FAULTEN3W {
        _FAULTEN3W { w: self }
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline]
    pub fn mcombine3(&mut self) -> _MCOMBINE3W {
        _MCOMBINE3W { w: self }
    }
}
