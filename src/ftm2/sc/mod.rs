#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC {
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "Divide by 1"]
    _000,
    #[doc = "Divide by 2"]
    _001,
    #[doc = "Divide by 4"]
    _010,
    #[doc = "Divide by 8"]
    _011,
    #[doc = "Divide by 16"]
    _100,
    #[doc = "Divide by 32"]
    _101,
    #[doc = "Divide by 64"]
    _110,
    #[doc = "Divide by 128"]
    _111,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_000 => 0,
            PSR::_001 => 1,
            PSR::_010 => 2,
            PSR::_011 => 3,
            PSR::_100 => 4,
            PSR::_101 => 5,
            PSR::_110 => 6,
            PSR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_000,
            1 => PSR::_001,
            2 => PSR::_010,
            3 => PSR::_011,
            4 => PSR::_100,
            5 => PSR::_101,
            6 => PSR::_110,
            7 => PSR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSR::_111
    }
}
#[doc = "Possible values of the field `CLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSR {
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    _00,
    #[doc = "FTM input clock"]
    _01,
    #[doc = "Fixed frequency clock"]
    _10,
    #[doc = "External clock"]
    _11,
}
impl CLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKSR::_00 => 0,
            CLKSR::_01 => 1,
            CLKSR::_10 => 2,
            CLKSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKSR {
        match value {
            0 => CLKSR::_00,
            1 => CLKSR::_01,
            2 => CLKSR::_10,
            3 => CLKSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CLKSR::_11
    }
}
#[doc = "Possible values of the field `CPWMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMSR {
    #[doc = "FTM counter operates in Up Counting mode."]
    _0,
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    _1,
}
impl CPWMSR {
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
            CPWMSR::_0 => false,
            CPWMSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPWMSR {
        match value {
            false => CPWMSR::_0,
            true => CPWMSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPWMSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPWMSR::_1
    }
}
#[doc = "Possible values of the field `RIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIER {
    #[doc = "Reload point interrupt is disabled."]
    _0,
    #[doc = "Reload point interrupt is enabled."]
    _1,
}
impl RIER {
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
            RIER::_0 => false,
            RIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIER {
        match value {
            false => RIER::_0,
            true => RIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RIER::_1
    }
}
#[doc = "Possible values of the field `RF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFR {
    #[doc = "A selected reload point did not happen."]
    _0,
    #[doc = "A selected reload point happened."]
    _1,
}
impl RFR {
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
            RFR::_0 => false,
            RFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFR {
        match value {
            false => RFR::_0,
            true => RFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFR::_1
    }
}
#[doc = "Possible values of the field `TOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIER {
    #[doc = "Disable TOF interrupts. Use software polling."]
    _0,
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1,
}
impl TOIER {
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
            TOIER::_0 => false,
            TOIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOIER {
        match value {
            false => TOIER::_0,
            true => TOIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOIER::_1
    }
}
#[doc = "Possible values of the field `TOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOFR {
    #[doc = "FTM counter has not overflowed."]
    _0,
    #[doc = "FTM counter has overflowed."]
    _1,
}
impl TOFR {
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
            TOFR::_0 => false,
            TOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TOFR {
        match value {
            false => TOFR::_0,
            true => TOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOFR::_1
    }
}
#[doc = "Possible values of the field `PWMEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN0R {
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
            PWMEN0R::_0 => false,
            PWMEN0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN0R {
        match value {
            false => PWMEN0R::_0,
            true => PWMEN0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN0R::_1
    }
}
#[doc = "Possible values of the field `PWMEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN1R {
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
            PWMEN1R::_0 => false,
            PWMEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN1R {
        match value {
            false => PWMEN1R::_0,
            true => PWMEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN1R::_1
    }
}
#[doc = "Possible values of the field `PWMEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN2R {
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
            PWMEN2R::_0 => false,
            PWMEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN2R {
        match value {
            false => PWMEN2R::_0,
            true => PWMEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN2R::_1
    }
}
#[doc = "Possible values of the field `PWMEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN3R {
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
            PWMEN3R::_0 => false,
            PWMEN3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN3R {
        match value {
            false => PWMEN3R::_0,
            true => PWMEN3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN3R::_1
    }
}
#[doc = "Possible values of the field `PWMEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN4R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN4R {
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
            PWMEN4R::_0 => false,
            PWMEN4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN4R {
        match value {
            false => PWMEN4R::_0,
            true => PWMEN4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN4R::_1
    }
}
#[doc = "Possible values of the field `PWMEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN5R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN5R {
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
            PWMEN5R::_0 => false,
            PWMEN5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN5R {
        match value {
            false => PWMEN5R::_0,
            true => PWMEN5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN5R::_1
    }
}
#[doc = "Possible values of the field `PWMEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN6R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN6R {
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
            PWMEN6R::_0 => false,
            PWMEN6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN6R {
        match value {
            false => PWMEN6R::_0,
            true => PWMEN6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN6R::_1
    }
}
#[doc = "Possible values of the field `PWMEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN7R {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN7R {
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
            PWMEN7R::_0 => false,
            PWMEN7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMEN7R {
        match value {
            false => PWMEN7R::_0,
            true => PWMEN7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PWMEN7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PWMEN7R::_1
    }
}
#[doc = "Possible values of the field `FLTPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLTPSR {
    #[doc = "Divide by 1"]
    _0000,
    #[doc = "Divide by 2"]
    _0001,
    #[doc = "Divide by 3"]
    _0010,
    #[doc = "Divide by 4"]
    _0011,
    #[doc = "Divide by 5"]
    _0100,
    #[doc = "Divide by 6"]
    _0101,
    #[doc = "Divide by 7"]
    _0110,
    #[doc = "Divide by 8"]
    _0111,
    #[doc = "Divide by 9"]
    _1000,
    #[doc = "Divide by 10"]
    _1001,
    #[doc = "Divide by 11"]
    _1010,
    #[doc = "Divide by 12"]
    _1011,
    #[doc = "Divide by 13"]
    _1100,
    #[doc = "Divide by 14"]
    _1101,
    #[doc = "Divide by 15"]
    _1110,
    #[doc = "Divide by 16"]
    _1111,
}
impl FLTPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLTPSR::_0000 => 0,
            FLTPSR::_0001 => 1,
            FLTPSR::_0010 => 2,
            FLTPSR::_0011 => 3,
            FLTPSR::_0100 => 4,
            FLTPSR::_0101 => 5,
            FLTPSR::_0110 => 6,
            FLTPSR::_0111 => 7,
            FLTPSR::_1000 => 8,
            FLTPSR::_1001 => 9,
            FLTPSR::_1010 => 10,
            FLTPSR::_1011 => 11,
            FLTPSR::_1100 => 12,
            FLTPSR::_1101 => 13,
            FLTPSR::_1110 => 14,
            FLTPSR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLTPSR {
        match value {
            0 => FLTPSR::_0000,
            1 => FLTPSR::_0001,
            2 => FLTPSR::_0010,
            3 => FLTPSR::_0011,
            4 => FLTPSR::_0100,
            5 => FLTPSR::_0101,
            6 => FLTPSR::_0110,
            7 => FLTPSR::_0111,
            8 => FLTPSR::_1000,
            9 => FLTPSR::_1001,
            10 => FLTPSR::_1010,
            11 => FLTPSR::_1011,
            12 => FLTPSR::_1100,
            13 => FLTPSR::_1101,
            14 => FLTPSR::_1110,
            15 => FLTPSR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == FLTPSR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == FLTPSR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == FLTPSR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == FLTPSR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == FLTPSR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == FLTPSR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == FLTPSR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == FLTPSR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == FLTPSR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == FLTPSR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == FLTPSR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == FLTPSR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == FLTPSR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == FLTPSR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == FLTPSR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == FLTPSR::_1111
    }
}
#[doc = "Values that can be written to the field `PS`"]
pub enum PSW {
    #[doc = "Divide by 1"]
    _000,
    #[doc = "Divide by 2"]
    _001,
    #[doc = "Divide by 4"]
    _010,
    #[doc = "Divide by 8"]
    _011,
    #[doc = "Divide by 16"]
    _100,
    #[doc = "Divide by 32"]
    _101,
    #[doc = "Divide by 64"]
    _110,
    #[doc = "Divide by 128"]
    _111,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_000 => 0,
            PSW::_001 => 1,
            PSW::_010 => 2,
            PSW::_011 => 3,
            PSW::_100 => 4,
            PSW::_101 => 5,
            PSW::_110 => 6,
            PSW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSW::_000)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSW::_001)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSW::_010)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSW::_011)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSW::_100)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSW::_101)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSW::_110)
    }
    #[doc = "Divide by 128"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKS`"]
pub enum CLKSW {
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    _00,
    #[doc = "FTM input clock"]
    _01,
    #[doc = "Fixed frequency clock"]
    _10,
    #[doc = "External clock"]
    _11,
}
impl CLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKSW::_00 => 0,
            CLKSW::_01 => 1,
            CLKSW::_10 => 2,
            CLKSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKSW::_00)
    }
    #[doc = "FTM input clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKSW::_01)
    }
    #[doc = "Fixed frequency clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKSW::_10)
    }
    #[doc = "External clock"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPWMS`"]
pub enum CPWMSW {
    #[doc = "FTM counter operates in Up Counting mode."]
    _0,
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    _1,
}
impl CPWMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPWMSW::_0 => false,
            CPWMSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPWMSW<'a> {
    w: &'a mut W,
}
impl<'a> _CPWMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPWMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM counter operates in Up Counting mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMSW::_0)
    }
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMSW::_1)
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
#[doc = "Values that can be written to the field `RIE`"]
pub enum RIEW {
    #[doc = "Reload point interrupt is disabled."]
    _0,
    #[doc = "Reload point interrupt is enabled."]
    _1,
}
impl RIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIEW::_0 => false,
            RIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reload point interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIEW::_0)
    }
    #[doc = "Reload point interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIEW::_1)
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
#[doc = "Values that can be written to the field `TOIE`"]
pub enum TOIEW {
    #[doc = "Disable TOF interrupts. Use software polling."]
    _0,
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1,
}
impl TOIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TOIEW::_0 => false,
            TOIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TOIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEW::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIEW::_1)
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
#[doc = "Values that can be written to the field `PWMEN0`"]
pub enum PWMEN0W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN0W::_0 => false,
            PWMEN0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN0W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN0W::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMEN1`"]
pub enum PWMEN1W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN1W::_0 => false,
            PWMEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN1W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN1W::_1)
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
#[doc = "Values that can be written to the field `PWMEN2`"]
pub enum PWMEN2W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN2W::_0 => false,
            PWMEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN2W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN2W::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMEN3`"]
pub enum PWMEN3W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN3W::_0 => false,
            PWMEN3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN3W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN3W::_1)
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
#[doc = "Values that can be written to the field `PWMEN4`"]
pub enum PWMEN4W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN4W::_0 => false,
            PWMEN4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN4W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN4W::_1)
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
#[doc = "Values that can be written to the field `PWMEN5`"]
pub enum PWMEN5W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN5W::_0 => false,
            PWMEN5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN5W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN5W::_1)
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
#[doc = "Values that can be written to the field `PWMEN6`"]
pub enum PWMEN6W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN6W::_0 => false,
            PWMEN6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN6W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN6W::_1)
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
#[doc = "Values that can be written to the field `PWMEN7`"]
pub enum PWMEN7W {
    #[doc = "Channel output port is disabled"]
    _0,
    #[doc = "Channel output port is enabled"]
    _1,
}
impl PWMEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMEN7W::_0 => false,
            PWMEN7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PWMEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN7W::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN7W::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLTPS`"]
pub enum FLTPSW {
    #[doc = "Divide by 1"]
    _0000,
    #[doc = "Divide by 2"]
    _0001,
    #[doc = "Divide by 3"]
    _0010,
    #[doc = "Divide by 4"]
    _0011,
    #[doc = "Divide by 5"]
    _0100,
    #[doc = "Divide by 6"]
    _0101,
    #[doc = "Divide by 7"]
    _0110,
    #[doc = "Divide by 8"]
    _0111,
    #[doc = "Divide by 9"]
    _1000,
    #[doc = "Divide by 10"]
    _1001,
    #[doc = "Divide by 11"]
    _1010,
    #[doc = "Divide by 12"]
    _1011,
    #[doc = "Divide by 13"]
    _1100,
    #[doc = "Divide by 14"]
    _1101,
    #[doc = "Divide by 15"]
    _1110,
    #[doc = "Divide by 16"]
    _1111,
}
impl FLTPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLTPSW::_0000 => 0,
            FLTPSW::_0001 => 1,
            FLTPSW::_0010 => 2,
            FLTPSW::_0011 => 3,
            FLTPSW::_0100 => 4,
            FLTPSW::_0101 => 5,
            FLTPSW::_0110 => 6,
            FLTPSW::_0111 => 7,
            FLTPSW::_1000 => 8,
            FLTPSW::_1001 => 9,
            FLTPSW::_1010 => 10,
            FLTPSW::_1011 => 11,
            FLTPSW::_1100 => 12,
            FLTPSW::_1101 => 13,
            FLTPSW::_1110 => 14,
            FLTPSW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLTPSW<'a> {
    w: &'a mut W,
}
impl<'a> _FLTPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLTPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FLTPSW::_0000)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(FLTPSW::_0001)
    }
    #[doc = "Divide by 3"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(FLTPSW::_0010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(FLTPSW::_0011)
    }
    #[doc = "Divide by 5"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(FLTPSW::_0100)
    }
    #[doc = "Divide by 6"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(FLTPSW::_0101)
    }
    #[doc = "Divide by 7"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(FLTPSW::_0110)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(FLTPSW::_0111)
    }
    #[doc = "Divide by 9"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(FLTPSW::_1000)
    }
    #[doc = "Divide by 10"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(FLTPSW::_1001)
    }
    #[doc = "Divide by 11"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(FLTPSW::_1010)
    }
    #[doc = "Divide by 12"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(FLTPSW::_1011)
    }
    #[doc = "Divide by 13"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(FLTPSW::_1100)
    }
    #[doc = "Divide by 14"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(FLTPSW::_1101)
    }
    #[doc = "Divide by 15"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(FLTPSW::_1110)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(FLTPSW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline]
    pub fn clks(&self) -> CLKSR {
        CLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline]
    pub fn cpwms(&self) -> CPWMSR {
        CPWMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline]
    pub fn rie(&self) -> RIER {
        RIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Reload Flag"]
    #[inline]
    pub fn rf(&self) -> RFR {
        RFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&self) -> TOIER {
        TOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Timer Overflow Flag"]
    #[inline]
    pub fn tof(&self) -> TOFR {
        TOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline]
    pub fn pwmen0(&self) -> PWMEN0R {
        PWMEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline]
    pub fn pwmen1(&self) -> PWMEN1R {
        PWMEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline]
    pub fn pwmen2(&self) -> PWMEN2R {
        PWMEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline]
    pub fn pwmen3(&self) -> PWMEN3R {
        PWMEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline]
    pub fn pwmen4(&self) -> PWMEN4R {
        PWMEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline]
    pub fn pwmen5(&self) -> PWMEN5R {
        PWMEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline]
    pub fn pwmen6(&self) -> PWMEN6R {
        PWMEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline]
    pub fn pwmen7(&self) -> PWMEN7R {
        PWMEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline]
    pub fn fltps(&self) -> FLTPSR {
        FLTPSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline]
    pub fn clks(&mut self) -> _CLKSW {
        _CLKSW { w: self }
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline]
    pub fn cpwms(&mut self) -> _CPWMSW {
        _CPWMSW { w: self }
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&mut self) -> _TOIEW {
        _TOIEW { w: self }
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline]
    pub fn pwmen0(&mut self) -> _PWMEN0W {
        _PWMEN0W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline]
    pub fn pwmen1(&mut self) -> _PWMEN1W {
        _PWMEN1W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline]
    pub fn pwmen2(&mut self) -> _PWMEN2W {
        _PWMEN2W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline]
    pub fn pwmen3(&mut self) -> _PWMEN3W {
        _PWMEN3W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline]
    pub fn pwmen4(&mut self) -> _PWMEN4W {
        _PWMEN4W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline]
    pub fn pwmen5(&mut self) -> _PWMEN5W {
        _PWMEN5W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline]
    pub fn pwmen6(&mut self) -> _PWMEN6W {
        _PWMEN6W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline]
    pub fn pwmen7(&mut self) -> _PWMEN7W {
        _PWMEN7W { w: self }
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline]
    pub fn fltps(&mut self) -> _FLTPSW {
        _FLTPSW { w: self }
    }
}
