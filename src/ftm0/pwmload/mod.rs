#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWMLOAD {
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
#[doc = "Possible values of the field `CH0SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH0SELR {
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
            CH0SELR::_0 => false,
            CH0SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0SELR {
        match value {
            false => CH0SELR::_0,
            true => CH0SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH0SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH0SELR::_1
    }
}
#[doc = "Possible values of the field `CH1SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH1SELR {
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
            CH1SELR::_0 => false,
            CH1SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1SELR {
        match value {
            false => CH1SELR::_0,
            true => CH1SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH1SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH1SELR::_1
    }
}
#[doc = "Possible values of the field `CH2SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH2SELR {
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
            CH2SELR::_0 => false,
            CH2SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2SELR {
        match value {
            false => CH2SELR::_0,
            true => CH2SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH2SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH2SELR::_1
    }
}
#[doc = "Possible values of the field `CH3SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH3SELR {
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
            CH3SELR::_0 => false,
            CH3SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3SELR {
        match value {
            false => CH3SELR::_0,
            true => CH3SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH3SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH3SELR::_1
    }
}
#[doc = "Possible values of the field `CH4SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH4SELR {
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
            CH4SELR::_0 => false,
            CH4SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4SELR {
        match value {
            false => CH4SELR::_0,
            true => CH4SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH4SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH4SELR::_1
    }
}
#[doc = "Possible values of the field `CH5SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH5SELR {
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
            CH5SELR::_0 => false,
            CH5SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5SELR {
        match value {
            false => CH5SELR::_0,
            true => CH5SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH5SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH5SELR::_1
    }
}
#[doc = "Possible values of the field `CH6SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH6SELR {
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
            CH6SELR::_0 => false,
            CH6SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6SELR {
        match value {
            false => CH6SELR::_0,
            true => CH6SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH6SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH6SELR::_1
    }
}
#[doc = "Possible values of the field `CH7SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7SELR {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH7SELR {
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
            CH7SELR::_0 => false,
            CH7SELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7SELR {
        match value {
            false => CH7SELR::_0,
            true => CH7SELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CH7SELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CH7SELR::_1
    }
}
#[doc = "Possible values of the field `HCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCSELR {
    #[doc = "Half cycle reload is disabled and it is not considered as a reload opportunity."]
    _0,
    #[doc = "Half cycle reload is enabled and it is considered as a reload opportunity."]
    _1,
}
impl HCSELR {
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
            HCSELR::_0 => false,
            HCSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HCSELR {
        match value {
            false => HCSELR::_0,
            true => HCSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HCSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HCSELR::_1
    }
}
#[doc = "Possible values of the field `LDOK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOKR {
    #[doc = "Loading updated values is disabled."]
    _0,
    #[doc = "Loading updated values is enabled."]
    _1,
}
impl LDOKR {
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
            LDOKR::_0 => false,
            LDOKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDOKR {
        match value {
            false => LDOKR::_0,
            true => LDOKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LDOKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LDOKR::_1
    }
}
#[doc = "Possible values of the field `GLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLENR {
    #[doc = "Global Load Ok disabled."]
    _0,
    #[doc = "Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    _1,
}
impl GLENR {
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
            GLENR::_0 => false,
            GLENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GLENR {
        match value {
            false => GLENR::_0,
            true => GLENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GLENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GLENR::_1
    }
}
#[doc = "Values that can be written to the field `CH0SEL`"]
pub enum CH0SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH0SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0SELW::_0 => false,
            CH0SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0SELW::_1)
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
#[doc = "Values that can be written to the field `CH1SEL`"]
pub enum CH1SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH1SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1SELW::_0 => false,
            CH1SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1SELW::_1)
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
#[doc = "Values that can be written to the field `CH2SEL`"]
pub enum CH2SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH2SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2SELW::_0 => false,
            CH2SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2SELW::_1)
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
#[doc = "Values that can be written to the field `CH3SEL`"]
pub enum CH3SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH3SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3SELW::_0 => false,
            CH3SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3SELW::_1)
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
#[doc = "Values that can be written to the field `CH4SEL`"]
pub enum CH4SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH4SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4SELW::_0 => false,
            CH4SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4SELW::_1)
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
#[doc = "Values that can be written to the field `CH5SEL`"]
pub enum CH5SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH5SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5SELW::_0 => false,
            CH5SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5SELW::_1)
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
#[doc = "Values that can be written to the field `CH6SEL`"]
pub enum CH6SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH6SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6SELW::_0 => false,
            CH6SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6SELW::_1)
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
#[doc = "Values that can be written to the field `CH7SEL`"]
pub enum CH7SELW {
    #[doc = "Channel match is not included as a reload opportunity."]
    _0,
    #[doc = "Channel match is included as a reload opportunity."]
    _1,
}
impl CH7SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7SELW::_0 => false,
            CH7SELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7SELW::_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7SELW::_1)
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
#[doc = "Values that can be written to the field `HCSEL`"]
pub enum HCSELW {
    #[doc = "Half cycle reload is disabled and it is not considered as a reload opportunity."]
    _0,
    #[doc = "Half cycle reload is enabled and it is considered as a reload opportunity."]
    _1,
}
impl HCSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HCSELW::_0 => false,
            HCSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HCSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HCSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half cycle reload is disabled and it is not considered as a reload opportunity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCSELW::_0)
    }
    #[doc = "Half cycle reload is enabled and it is considered as a reload opportunity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCSELW::_1)
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
#[doc = "Values that can be written to the field `LDOK`"]
pub enum LDOKW {
    #[doc = "Loading updated values is disabled."]
    _0,
    #[doc = "Loading updated values is enabled."]
    _1,
}
impl LDOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDOKW::_0 => false,
            LDOKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loading updated values is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDOKW::_0)
    }
    #[doc = "Loading updated values is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDOKW::_1)
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
#[doc = "Values that can be written to the field `GLEN`"]
pub enum GLENW {
    #[doc = "Global Load Ok disabled."]
    _0,
    #[doc = "Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    _1,
}
impl GLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GLENW::_0 => false,
            GLENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GLENW<'a> {
    w: &'a mut W,
}
impl<'a> _GLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Global Load Ok disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GLENW::_0)
    }
    #[doc = "Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GLENW::_1)
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
#[doc = "Values that can be written to the field `GLDOK`"]
pub enum GLDOKW {
    #[doc = "No action."]
    _0,
    #[doc = "LDOK bit is set."]
    _1,
}
impl GLDOKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GLDOKW::_0 => false,
            GLDOKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GLDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _GLDOKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GLDOKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GLDOKW::_0)
    }
    #[doc = "LDOK bit is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GLDOKW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline]
    pub fn ch0sel(&self) -> CH0SELR {
        CH0SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline]
    pub fn ch1sel(&self) -> CH1SELR {
        CH1SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline]
    pub fn ch2sel(&self) -> CH2SELR {
        CH2SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline]
    pub fn ch3sel(&self) -> CH3SELR {
        CH3SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline]
    pub fn ch4sel(&self) -> CH4SELR {
        CH4SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline]
    pub fn ch5sel(&self) -> CH5SELR {
        CH5SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline]
    pub fn ch6sel(&self) -> CH6SELR {
        CH6SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline]
    pub fn ch7sel(&self) -> CH7SELR {
        CH7SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline]
    pub fn hcsel(&self) -> HCSELR {
        HCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline]
    pub fn ldok(&self) -> LDOKR {
        LDOKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline]
    pub fn glen(&self) -> GLENR {
        GLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline]
    pub fn ch0sel(&mut self) -> _CH0SELW {
        _CH0SELW { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline]
    pub fn ch1sel(&mut self) -> _CH1SELW {
        _CH1SELW { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline]
    pub fn ch2sel(&mut self) -> _CH2SELW {
        _CH2SELW { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline]
    pub fn ch3sel(&mut self) -> _CH3SELW {
        _CH3SELW { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline]
    pub fn ch4sel(&mut self) -> _CH4SELW {
        _CH4SELW { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline]
    pub fn ch5sel(&mut self) -> _CH5SELW {
        _CH5SELW { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline]
    pub fn ch6sel(&mut self) -> _CH6SELW {
        _CH6SELW { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline]
    pub fn ch7sel(&mut self) -> _CH7SELW {
        _CH7SELW { w: self }
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline]
    pub fn hcsel(&mut self) -> _HCSELW {
        _HCSELW { w: self }
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline]
    pub fn ldok(&mut self) -> _LDOKW {
        _LDOKW { w: self }
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline]
    pub fn glen(&mut self) -> _GLENW {
        _GLENW { w: self }
    }
    #[doc = "Bit 11 - Global Load OK"]
    #[inline]
    pub fn gldok(&mut self) -> _GLDOKW {
        _GLDOKW { w: self }
    }
}
