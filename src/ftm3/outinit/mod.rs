#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTINIT {
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
#[doc = "Possible values of the field `CH0OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH0OIR {
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
            CH0OIR::_0 => false,
            CH0OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0OIR {
        match value {
            false => CH0OIR::_0,
            true => CH0OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0OIR::_1
    }
}
#[doc = "Possible values of the field `CH1OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH1OIR {
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
            CH1OIR::_0 => false,
            CH1OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1OIR {
        match value {
            false => CH1OIR::_0,
            true => CH1OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1OIR::_1
    }
}
#[doc = "Possible values of the field `CH2OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH2OIR {
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
            CH2OIR::_0 => false,
            CH2OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2OIR {
        match value {
            false => CH2OIR::_0,
            true => CH2OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2OIR::_1
    }
}
#[doc = "Possible values of the field `CH3OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH3OIR {
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
            CH3OIR::_0 => false,
            CH3OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3OIR {
        match value {
            false => CH3OIR::_0,
            true => CH3OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3OIR::_1
    }
}
#[doc = "Possible values of the field `CH4OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH4OIR {
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
            CH4OIR::_0 => false,
            CH4OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4OIR {
        match value {
            false => CH4OIR::_0,
            true => CH4OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4OIR::_1
    }
}
#[doc = "Possible values of the field `CH5OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH5OIR {
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
            CH5OIR::_0 => false,
            CH5OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5OIR {
        match value {
            false => CH5OIR::_0,
            true => CH5OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5OIR::_1
    }
}
#[doc = "Possible values of the field `CH6OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH6OIR {
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
            CH6OIR::_0 => false,
            CH6OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6OIR {
        match value {
            false => CH6OIR::_0,
            true => CH6OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6OIR::_1
    }
}
#[doc = "Possible values of the field `CH7OI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OIR {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH7OIR {
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
            CH7OIR::_0 => false,
            CH7OIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7OIR {
        match value {
            false => CH7OIR::_0,
            true => CH7OIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7OIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7OIR::_1
    }
}
#[doc = "Values that can be written to the field `CH0OI`"]
pub enum CH0OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH0OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0OIW::_0 => false,
            CH0OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OIW::_1)
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
#[doc = "Values that can be written to the field `CH1OI`"]
pub enum CH1OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH1OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1OIW::_0 => false,
            CH1OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OIW::_1)
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
#[doc = "Values that can be written to the field `CH2OI`"]
pub enum CH2OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH2OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2OIW::_0 => false,
            CH2OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OIW::_1)
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
#[doc = "Values that can be written to the field `CH3OI`"]
pub enum CH3OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH3OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3OIW::_0 => false,
            CH3OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OIW::_1)
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
#[doc = "Values that can be written to the field `CH4OI`"]
pub enum CH4OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH4OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4OIW::_0 => false,
            CH4OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OIW::_1)
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
#[doc = "Values that can be written to the field `CH5OI`"]
pub enum CH5OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH5OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5OIW::_0 => false,
            CH5OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OIW::_1)
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
#[doc = "Values that can be written to the field `CH6OI`"]
pub enum CH6OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH6OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6OIW::_0 => false,
            CH6OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OIW::_1)
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
#[doc = "Values that can be written to the field `CH7OI`"]
pub enum CH7OIW {
    #[doc = "The initialization value is 0."] _0,
    #[doc = "The initialization value is 1."] _1,
}
impl CH7OIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7OIW::_0 => false,
            CH7OIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7OIW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7OIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7OIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The initialization value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OIW::_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OIW::_1)
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
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline]
    pub fn ch0oi(&self) -> CH0OIR {
        CH0OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline]
    pub fn ch1oi(&self) -> CH1OIR {
        CH1OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline]
    pub fn ch2oi(&self) -> CH2OIR {
        CH2OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline]
    pub fn ch3oi(&self) -> CH3OIR {
        CH3OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline]
    pub fn ch4oi(&self) -> CH4OIR {
        CH4OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline]
    pub fn ch5oi(&self) -> CH5OIR {
        CH5OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline]
    pub fn ch6oi(&self) -> CH6OIR {
        CH6OIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline]
    pub fn ch7oi(&self) -> CH7OIR {
        CH7OIR::_from({
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
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline]
    pub fn ch0oi(&mut self) -> _CH0OIW {
        _CH0OIW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline]
    pub fn ch1oi(&mut self) -> _CH1OIW {
        _CH1OIW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline]
    pub fn ch2oi(&mut self) -> _CH2OIW {
        _CH2OIW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline]
    pub fn ch3oi(&mut self) -> _CH3OIW {
        _CH3OIW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline]
    pub fn ch4oi(&mut self) -> _CH4OIW {
        _CH4OIW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline]
    pub fn ch5oi(&mut self) -> _CH5OIW {
        _CH5OIW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline]
    pub fn ch6oi(&mut self) -> _CH6OIW {
        _CH6OIW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline]
    pub fn ch7oi(&mut self) -> _CH7OIW {
        _CH7OIW { w: self }
    }
}
