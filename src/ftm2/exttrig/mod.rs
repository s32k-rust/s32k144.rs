#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTTRIG {
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
#[doc = "Possible values of the field `CH2TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH2TRIGR {
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
            CH2TRIGR::_0 => false,
            CH2TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2TRIGR {
        match value {
            false => CH2TRIGR::_0,
            true => CH2TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH3TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH3TRIGR {
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
            CH3TRIGR::_0 => false,
            CH3TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3TRIGR {
        match value {
            false => CH3TRIGR::_0,
            true => CH3TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH4TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH4TRIGR {
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
            CH4TRIGR::_0 => false,
            CH4TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4TRIGR {
        match value {
            false => CH4TRIGR::_0,
            true => CH4TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH5TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH5TRIGR {
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
            CH5TRIGR::_0 => false,
            CH5TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5TRIGR {
        match value {
            false => CH5TRIGR::_0,
            true => CH5TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH0TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH0TRIGR {
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
            CH0TRIGR::_0 => false,
            CH0TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0TRIGR {
        match value {
            false => CH0TRIGR::_0,
            true => CH0TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH1TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH1TRIGR {
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
            CH1TRIGR::_0 => false,
            CH1TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1TRIGR {
        match value {
            false => CH1TRIGR::_0,
            true => CH1TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1TRIGR::_1
    }
}
#[doc = "Possible values of the field `INITTRIGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITTRIGENR {
    #[doc = "The generation of initialization trigger is disabled."]
    _0,
    #[doc = "The generation of initialization trigger is enabled."]
    _1,
}
impl INITTRIGENR {
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
            INITTRIGENR::_0 => false,
            INITTRIGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITTRIGENR {
        match value {
            false => INITTRIGENR::_0,
            true => INITTRIGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INITTRIGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INITTRIGENR::_1
    }
}
#[doc = "Possible values of the field `TRIGF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGFR {
    #[doc = "No channel trigger was generated."]
    _0,
    #[doc = "A channel trigger was generated."]
    _1,
}
impl TRIGFR {
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
            TRIGFR::_0 => false,
            TRIGFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGFR {
        match value {
            false => TRIGFR::_0,
            true => TRIGFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIGFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIGFR::_1
    }
}
#[doc = "Possible values of the field `CH6TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH6TRIGR {
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
            CH6TRIGR::_0 => false,
            CH6TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6TRIGR {
        match value {
            false => CH6TRIGR::_0,
            true => CH6TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6TRIGR::_1
    }
}
#[doc = "Possible values of the field `CH7TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7TRIGR {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH7TRIGR {
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
            CH7TRIGR::_0 => false,
            CH7TRIGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7TRIGR {
        match value {
            false => CH7TRIGR::_0,
            true => CH7TRIGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7TRIGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7TRIGR::_1
    }
}
#[doc = "Values that can be written to the field `CH2TRIG`"]
pub enum CH2TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH2TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2TRIGW::_0 => false,
            CH2TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH3TRIG`"]
pub enum CH3TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH3TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3TRIGW::_0 => false,
            CH3TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH4TRIG`"]
pub enum CH4TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH4TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4TRIGW::_0 => false,
            CH4TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH5TRIG`"]
pub enum CH5TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH5TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5TRIGW::_0 => false,
            CH5TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH0TRIG`"]
pub enum CH0TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH0TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0TRIGW::_0 => false,
            CH0TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH1TRIG`"]
pub enum CH1TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH1TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1TRIGW::_0 => false,
            CH1TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1TRIGW::_1)
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
#[doc = "Values that can be written to the field `INITTRIGEN`"]
pub enum INITTRIGENW {
    #[doc = "The generation of initialization trigger is disabled."]
    _0,
    #[doc = "The generation of initialization trigger is enabled."]
    _1,
}
impl INITTRIGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITTRIGENW::_0 => false,
            INITTRIGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITTRIGENW<'a> {
    w: &'a mut W,
}
impl<'a> _INITTRIGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITTRIGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of initialization trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITTRIGENW::_0)
    }
    #[doc = "The generation of initialization trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITTRIGENW::_1)
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
#[doc = "Values that can be written to the field `CH6TRIG`"]
pub enum CH6TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH6TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6TRIGW::_0 => false,
            CH6TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6TRIGW::_1)
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
#[doc = "Values that can be written to the field `CH7TRIG`"]
pub enum CH7TRIGW {
    #[doc = "The generation of this external trigger is disabled."]
    _0,
    #[doc = "The generation of this external trigger is enabled."]
    _1,
}
impl CH7TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7TRIGW::_0 => false,
            CH7TRIGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The generation of this external trigger is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7TRIGW::_0)
    }
    #[doc = "The generation of this external trigger is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7TRIGW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 2 External Trigger Enable"]
    #[inline]
    pub fn ch2trig(&self) -> CH2TRIGR {
        CH2TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 3 External Trigger Enable"]
    #[inline]
    pub fn ch3trig(&self) -> CH3TRIGR {
        CH3TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 4 External Trigger Enable"]
    #[inline]
    pub fn ch4trig(&self) -> CH4TRIGR {
        CH4TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 5 External Trigger Enable"]
    #[inline]
    pub fn ch5trig(&self) -> CH5TRIGR {
        CH5TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 0 External Trigger Enable"]
    #[inline]
    pub fn ch0trig(&self) -> CH0TRIGR {
        CH0TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 1 External Trigger Enable"]
    #[inline]
    pub fn ch1trig(&self) -> CH1TRIGR {
        CH1TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Initialization Trigger Enable"]
    #[inline]
    pub fn inittrigen(&self) -> INITTRIGENR {
        INITTRIGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel Trigger Flag"]
    #[inline]
    pub fn trigf(&self) -> TRIGFR {
        TRIGFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 6 External Trigger Enable"]
    #[inline]
    pub fn ch6trig(&self) -> CH6TRIGR {
        CH6TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel 7 External Trigger Enable"]
    #[inline]
    pub fn ch7trig(&self) -> CH7TRIGR {
        CH7TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Channel 2 External Trigger Enable"]
    #[inline]
    pub fn ch2trig(&mut self) -> _CH2TRIGW {
        _CH2TRIGW { w: self }
    }
    #[doc = "Bit 1 - Channel 3 External Trigger Enable"]
    #[inline]
    pub fn ch3trig(&mut self) -> _CH3TRIGW {
        _CH3TRIGW { w: self }
    }
    #[doc = "Bit 2 - Channel 4 External Trigger Enable"]
    #[inline]
    pub fn ch4trig(&mut self) -> _CH4TRIGW {
        _CH4TRIGW { w: self }
    }
    #[doc = "Bit 3 - Channel 5 External Trigger Enable"]
    #[inline]
    pub fn ch5trig(&mut self) -> _CH5TRIGW {
        _CH5TRIGW { w: self }
    }
    #[doc = "Bit 4 - Channel 0 External Trigger Enable"]
    #[inline]
    pub fn ch0trig(&mut self) -> _CH0TRIGW {
        _CH0TRIGW { w: self }
    }
    #[doc = "Bit 5 - Channel 1 External Trigger Enable"]
    #[inline]
    pub fn ch1trig(&mut self) -> _CH1TRIGW {
        _CH1TRIGW { w: self }
    }
    #[doc = "Bit 6 - Initialization Trigger Enable"]
    #[inline]
    pub fn inittrigen(&mut self) -> _INITTRIGENW {
        _INITTRIGENW { w: self }
    }
    #[doc = "Bit 8 - Channel 6 External Trigger Enable"]
    #[inline]
    pub fn ch6trig(&mut self) -> _CH6TRIGW {
        _CH6TRIGW { w: self }
    }
    #[doc = "Bit 9 - Channel 7 External Trigger Enable"]
    #[inline]
    pub fn ch7trig(&mut self) -> _CH7TRIGW {
        _CH7TRIGW { w: self }
    }
}
