#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C2 {
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
#[doc = r" Value of the field"]
pub struct ACONR {
    bits: u8,
}
impl ACONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `INITMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITMODR {
    #[doc = "The modulus is set to 64(same with 111111)."]
    _000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INITMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INITMODR::_000000 => 0,
            INITMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INITMODR {
        match value {
            0 => INITMODR::_000000,
            i => INITMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline]
    pub fn is_000000(&self) -> bool {
        *self == INITMODR::_000000
    }
}
#[doc = "Possible values of the field `NSAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSAMR {
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    _00,
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    _01,
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    _10,
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    _11,
}
impl NSAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NSAMR::_00 => 0,
            NSAMR::_01 => 1,
            NSAMR::_10 => 2,
            NSAMR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NSAMR {
        match value {
            0 => NSAMR::_00,
            1 => NSAMR::_01,
            2 => NSAMR::_10,
            3 => NSAMR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == NSAMR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == NSAMR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == NSAMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == NSAMR::_11
    }
}
#[doc = r" Value of the field"]
pub struct CH0FR {
    bits: bool,
}
impl CH0FR {
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
pub struct CH1FR {
    bits: bool,
}
impl CH1FR {
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
pub struct CH2FR {
    bits: bool,
}
impl CH2FR {
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
pub struct CH3FR {
    bits: bool,
}
impl CH3FR {
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
pub struct CH4FR {
    bits: bool,
}
impl CH4FR {
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
pub struct CH5FR {
    bits: bool,
}
impl CH5FR {
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
pub struct CH6FR {
    bits: bool,
}
impl CH6FR {
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
pub struct CH7FR {
    bits: bool,
}
impl CH7FR {
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
#[doc = "Possible values of the field `FXMXCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXMXCHR {
    #[doc = "Channel 0 is selected as the fixed reference input for the fixed mux port."]
    _000,
    #[doc = "Channel 1 is selected as the fixed reference input for the fixed mux port."]
    _001,
    #[doc = "Channel 2 is selected as the fixed reference input for the fixed mux port."]
    _010,
    #[doc = "Channel 3 is selected as the fixed reference input for the fixed mux port."]
    _011,
    #[doc = "Channel 4 is selected as the fixed reference input for the fixed mux port."]
    _100,
    #[doc = "Channel 5 is selected as the fixed reference input for the fixed mux port."]
    _101,
    #[doc = "Channel 6 is selected as the fixed reference input for the fixed mux port."]
    _110,
    #[doc = "Channel 7 is selected as the fixed reference input for the fixed mux port."]
    _111,
}
impl FXMXCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FXMXCHR::_000 => 0,
            FXMXCHR::_001 => 1,
            FXMXCHR::_010 => 2,
            FXMXCHR::_011 => 3,
            FXMXCHR::_100 => 4,
            FXMXCHR::_101 => 5,
            FXMXCHR::_110 => 6,
            FXMXCHR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FXMXCHR {
        match value {
            0 => FXMXCHR::_000,
            1 => FXMXCHR::_001,
            2 => FXMXCHR::_010,
            3 => FXMXCHR::_011,
            4 => FXMXCHR::_100,
            5 => FXMXCHR::_101,
            6 => FXMXCHR::_110,
            7 => FXMXCHR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FXMXCHR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FXMXCHR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FXMXCHR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FXMXCHR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FXMXCHR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FXMXCHR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FXMXCHR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FXMXCHR::_111
    }
}
#[doc = "Possible values of the field `FXMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXMPR {
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    _0,
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    _1,
}
impl FXMPR {
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
            FXMPR::_0 => false,
            FXMPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FXMPR {
        match value {
            false => FXMPR::_0,
            true => FXMPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FXMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FXMPR::_1
    }
}
#[doc = "Possible values of the field `RRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIER {
    #[doc = "The round-robin interrupt is disabled."]
    _0,
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    _1,
}
impl RRIER {
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
            RRIER::_0 => false,
            RRIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRIER {
        match value {
            false => RRIER::_0,
            true => RRIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RRIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RRIER::_1
    }
}
#[doc = "Possible values of the field `RRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRER {
    #[doc = "Round-robin operation is disabled."]
    _0,
    #[doc = "Round-robin operation is enabled."]
    _1,
}
impl RRER {
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
            RRER::_0 => false,
            RRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRER {
        match value {
            false => RRER::_0,
            true => RRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RRER::_1
    }
}
#[doc = r" Proxy"]
pub struct _ACONW<'a> {
    w: &'a mut W,
}
impl<'a> _ACONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INITMOD`"]
pub enum INITMODW {
    #[doc = "The modulus is set to 64(same with 111111)."]
    _000000,
}
impl INITMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INITMODW::_000000 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITMODW<'a> {
    w: &'a mut W,
}
impl<'a> _INITMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The modulus is set to 64(same with 111111)."]
    #[inline]
    pub fn _000000(self) -> &'a mut W {
        self.variant(INITMODW::_000000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NSAM`"]
pub enum NSAMW {
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    _00,
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    _01,
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    _10,
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    _11,
}
impl NSAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NSAMW::_00 => 0,
            NSAMW::_01 => 1,
            NSAMW::_10 => 2,
            NSAMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NSAMW<'a> {
    w: &'a mut W,
}
impl<'a> _NSAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NSAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(NSAMW::_00)
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(NSAMW::_01)
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(NSAMW::_10)
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(NSAMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH0FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0FW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH1FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1FW<'a> {
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
pub struct _CH2FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2FW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH3FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3FW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH4FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH4FW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH5FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH5FW<'a> {
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
#[doc = r" Proxy"]
pub struct _CH6FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH6FW<'a> {
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
pub struct _CH7FW<'a> {
    w: &'a mut W,
}
impl<'a> _CH7FW<'a> {
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
#[doc = "Values that can be written to the field `FXMXCH`"]
pub enum FXMXCHW {
    #[doc = "Channel 0 is selected as the fixed reference input for the fixed mux port."]
    _000,
    #[doc = "Channel 1 is selected as the fixed reference input for the fixed mux port."]
    _001,
    #[doc = "Channel 2 is selected as the fixed reference input for the fixed mux port."]
    _010,
    #[doc = "Channel 3 is selected as the fixed reference input for the fixed mux port."]
    _011,
    #[doc = "Channel 4 is selected as the fixed reference input for the fixed mux port."]
    _100,
    #[doc = "Channel 5 is selected as the fixed reference input for the fixed mux port."]
    _101,
    #[doc = "Channel 6 is selected as the fixed reference input for the fixed mux port."]
    _110,
    #[doc = "Channel 7 is selected as the fixed reference input for the fixed mux port."]
    _111,
}
impl FXMXCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FXMXCHW::_000 => 0,
            FXMXCHW::_001 => 1,
            FXMXCHW::_010 => 2,
            FXMXCHW::_011 => 3,
            FXMXCHW::_100 => 4,
            FXMXCHW::_101 => 5,
            FXMXCHW::_110 => 6,
            FXMXCHW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FXMXCHW<'a> {
    w: &'a mut W,
}
impl<'a> _FXMXCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FXMXCHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Channel 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FXMXCHW::_000)
    }
    #[doc = "Channel 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FXMXCHW::_001)
    }
    #[doc = "Channel 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FXMXCHW::_010)
    }
    #[doc = "Channel 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FXMXCHW::_011)
    }
    #[doc = "Channel 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FXMXCHW::_100)
    }
    #[doc = "Channel 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FXMXCHW::_101)
    }
    #[doc = "Channel 6 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FXMXCHW::_110)
    }
    #[doc = "Channel 7 is selected as the fixed reference input for the fixed mux port."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FXMXCHW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FXMP`"]
pub enum FXMPW {
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    _0,
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    _1,
}
impl FXMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FXMPW::_0 => false,
            FXMPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FXMPW<'a> {
    w: &'a mut W,
}
impl<'a> _FXMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FXMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FXMPW::_0)
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FXMPW::_1)
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
#[doc = "Values that can be written to the field `RRIE`"]
pub enum RRIEW {
    #[doc = "The round-robin interrupt is disabled."]
    _0,
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    _1,
}
impl RRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRIEW::_0 => false,
            RRIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The round-robin interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRIEW::_0)
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRIEW::_1)
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
#[doc = "Values that can be written to the field `RRE`"]
pub enum RREW {
    #[doc = "Round-robin operation is disabled."]
    _0,
    #[doc = "Round-robin operation is enabled."]
    _1,
}
impl RREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RREW::_0 => false,
            RREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RREW<'a> {
    w: &'a mut W,
}
impl<'a> _RREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Round-robin operation is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RREW::_0)
    }
    #[doc = "Round-robin operation is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RREW::_1)
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
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline]
    pub fn acon(&self) -> ACONR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACONR { bits }
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline]
    pub fn initmod(&self) -> INITMODR {
        INITMODR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline]
    pub fn nsam(&self) -> NSAMR {
        NSAMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline]
    pub fn ch0f(&self) -> CH0FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH0FR { bits }
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline]
    pub fn ch1f(&self) -> CH1FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH1FR { bits }
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline]
    pub fn ch2f(&self) -> CH2FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH2FR { bits }
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline]
    pub fn ch3f(&self) -> CH3FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH3FR { bits }
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline]
    pub fn ch4f(&self) -> CH4FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH4FR { bits }
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline]
    pub fn ch5f(&self) -> CH5FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH5FR { bits }
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline]
    pub fn ch6f(&self) -> CH6FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH6FR { bits }
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline]
    pub fn ch7f(&self) -> CH7FR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CH7FR { bits }
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline]
    pub fn fxmxch(&self) -> FXMXCHR {
        FXMXCHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline]
    pub fn fxmp(&self) -> FXMPR {
        FXMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline]
    pub fn rrie(&self) -> RRIER {
        RRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline]
    pub fn rre(&self) -> RRER {
        RRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline]
    pub fn acon(&mut self) -> _ACONW {
        _ACONW { w: self }
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline]
    pub fn initmod(&mut self) -> _INITMODW {
        _INITMODW { w: self }
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline]
    pub fn nsam(&mut self) -> _NSAMW {
        _NSAMW { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline]
    pub fn ch0f(&mut self) -> _CH0FW {
        _CH0FW { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline]
    pub fn ch1f(&mut self) -> _CH1FW {
        _CH1FW { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline]
    pub fn ch2f(&mut self) -> _CH2FW {
        _CH2FW { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline]
    pub fn ch3f(&mut self) -> _CH3FW {
        _CH3FW { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline]
    pub fn ch4f(&mut self) -> _CH4FW {
        _CH4FW { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline]
    pub fn ch5f(&mut self) -> _CH5FW {
        _CH5FW { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline]
    pub fn ch6f(&mut self) -> _CH6FW {
        _CH6FW { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline]
    pub fn ch7f(&mut self) -> _CH7FW {
        _CH7FW { w: self }
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline]
    pub fn fxmxch(&mut self) -> _FXMXCHW {
        _FXMXCHW { w: self }
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline]
    pub fn fxmp(&mut self) -> _FXMPW {
        _FXMPW { w: self }
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline]
    pub fn rrie(&mut self) -> _RRIEW {
        _RRIEW { w: self }
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline]
    pub fn rre(&mut self) -> _RREW {
        _RREW { w: self }
    }
}
