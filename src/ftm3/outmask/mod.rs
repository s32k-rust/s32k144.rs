#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTMASK {
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
#[doc = "Possible values of the field `CH0OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH0OMR {
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
            CH0OMR::_0 => false,
            CH0OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0OMR {
        match value {
            false => CH0OMR::_0,
            true => CH0OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0OMR::_1
    }
}
#[doc = "Possible values of the field `CH1OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH1OMR {
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
            CH1OMR::_0 => false,
            CH1OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1OMR {
        match value {
            false => CH1OMR::_0,
            true => CH1OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1OMR::_1
    }
}
#[doc = "Possible values of the field `CH2OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH2OMR {
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
            CH2OMR::_0 => false,
            CH2OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2OMR {
        match value {
            false => CH2OMR::_0,
            true => CH2OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2OMR::_1
    }
}
#[doc = "Possible values of the field `CH3OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH3OMR {
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
            CH3OMR::_0 => false,
            CH3OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3OMR {
        match value {
            false => CH3OMR::_0,
            true => CH3OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3OMR::_1
    }
}
#[doc = "Possible values of the field `CH4OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH4OMR {
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
            CH4OMR::_0 => false,
            CH4OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4OMR {
        match value {
            false => CH4OMR::_0,
            true => CH4OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4OMR::_1
    }
}
#[doc = "Possible values of the field `CH5OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH5OMR {
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
            CH5OMR::_0 => false,
            CH5OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5OMR {
        match value {
            false => CH5OMR::_0,
            true => CH5OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5OMR::_1
    }
}
#[doc = "Possible values of the field `CH6OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH6OMR {
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
            CH6OMR::_0 => false,
            CH6OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6OMR {
        match value {
            false => CH6OMR::_0,
            true => CH6OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6OMR::_1
    }
}
#[doc = "Possible values of the field `CH7OM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OMR {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH7OMR {
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
            CH7OMR::_0 => false,
            CH7OMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7OMR {
        match value {
            false => CH7OMR::_0,
            true => CH7OMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7OMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7OMR::_1
    }
}
#[doc = "Values that can be written to the field `CH0OM`"]
pub enum CH0OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH0OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0OMW::_0 => false,
            CH0OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OMW::_1)
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
#[doc = "Values that can be written to the field `CH1OM`"]
pub enum CH1OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH1OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1OMW::_0 => false,
            CH1OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OMW::_1)
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
#[doc = "Values that can be written to the field `CH2OM`"]
pub enum CH2OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH2OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2OMW::_0 => false,
            CH2OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OMW::_1)
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
#[doc = "Values that can be written to the field `CH3OM`"]
pub enum CH3OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH3OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3OMW::_0 => false,
            CH3OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OMW::_1)
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
#[doc = "Values that can be written to the field `CH4OM`"]
pub enum CH4OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH4OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4OMW::_0 => false,
            CH4OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OMW::_1)
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
#[doc = "Values that can be written to the field `CH5OM`"]
pub enum CH5OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH5OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5OMW::_0 => false,
            CH5OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OMW::_1)
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
#[doc = "Values that can be written to the field `CH6OM`"]
pub enum CH6OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH6OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6OMW::_0 => false,
            CH6OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OMW::_1)
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
#[doc = "Values that can be written to the field `CH7OM`"]
pub enum CH7OMW {
    #[doc = "Channel output is not masked. It continues to operate normally."] _0,
    #[doc = "Channel output is masked. It is forced to its inactive state."] _1,
}
impl CH7OMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7OMW::_0 => false,
            CH7OMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7OMW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7OMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7OMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OMW::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OMW::_1)
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
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline]
    pub fn ch0om(&self) -> CH0OMR {
        CH0OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline]
    pub fn ch1om(&self) -> CH1OMR {
        CH1OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline]
    pub fn ch2om(&self) -> CH2OMR {
        CH2OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline]
    pub fn ch3om(&self) -> CH3OMR {
        CH3OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline]
    pub fn ch4om(&self) -> CH4OMR {
        CH4OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline]
    pub fn ch5om(&self) -> CH5OMR {
        CH5OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline]
    pub fn ch6om(&self) -> CH6OMR {
        CH6OMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline]
    pub fn ch7om(&self) -> CH7OMR {
        CH7OMR::_from({
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
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline]
    pub fn ch0om(&mut self) -> _CH0OMW {
        _CH0OMW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline]
    pub fn ch1om(&mut self) -> _CH1OMW {
        _CH1OMW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline]
    pub fn ch2om(&mut self) -> _CH2OMW {
        _CH2OMW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline]
    pub fn ch3om(&mut self) -> _CH3OMW {
        _CH3OMW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline]
    pub fn ch4om(&mut self) -> _CH4OMW {
        _CH4OMW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline]
    pub fn ch5om(&mut self) -> _CH5OMW {
        _CH5OMW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline]
    pub fn ch6om(&mut self) -> _CH6OMW {
        _CH6OMW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline]
    pub fn ch7om(&mut self) -> _CH7OMW {
        _CH7OMW { w: self }
    }
}
