#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C0 {
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
#[doc = "Possible values of the field `HYSTCTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYSTCTRR {
    #[doc = "The hard block output has level 0 hysteresis internally."]
    _00,
    #[doc = "The hard block output has level 1 hysteresis internally."]
    _01,
    #[doc = "The hard block output has level 2 hysteresis internally."]
    _10,
    #[doc = "The hard block output has level 3 hysteresis internally."]
    _11,
}
impl HYSTCTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HYSTCTRR::_00 => 0,
            HYSTCTRR::_01 => 1,
            HYSTCTRR::_10 => 2,
            HYSTCTRR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HYSTCTRR {
        match value {
            0 => HYSTCTRR::_00,
            1 => HYSTCTRR::_01,
            2 => HYSTCTRR::_10,
            3 => HYSTCTRR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == HYSTCTRR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == HYSTCTRR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == HYSTCTRR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == HYSTCTRR::_11
    }
}
#[doc = "Possible values of the field `OFFSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFSETR {
    #[doc = "The comparator hard block output has level 0 offset internally."]
    _0,
    #[doc = "The comparator hard block output has level 1 offset internally."]
    _1,
}
impl OFFSETR {
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
            OFFSETR::_0 => false,
            OFFSETR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFSETR {
        match value {
            false => OFFSETR::_0,
            true => OFFSETR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OFFSETR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OFFSETR::_1
    }
}
#[doc = "Possible values of the field `FILTER_CNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_CNTR {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    _000,
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    _001,
    #[doc = "2 consecutive samples must agree."]
    _010,
    #[doc = "3 consecutive samples must agree."]
    _011,
    #[doc = "4 consecutive samples must agree."]
    _100,
    #[doc = "5 consecutive samples must agree."]
    _101,
    #[doc = "6 consecutive samples must agree."]
    _110,
    #[doc = "7 consecutive samples must agree."]
    _111,
}
impl FILTER_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FILTER_CNTR::_000 => 0,
            FILTER_CNTR::_001 => 1,
            FILTER_CNTR::_010 => 2,
            FILTER_CNTR::_011 => 3,
            FILTER_CNTR::_100 => 4,
            FILTER_CNTR::_101 => 5,
            FILTER_CNTR::_110 => 6,
            FILTER_CNTR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FILTER_CNTR {
        match value {
            0 => FILTER_CNTR::_000,
            1 => FILTER_CNTR::_001,
            2 => FILTER_CNTR::_010,
            3 => FILTER_CNTR::_011,
            4 => FILTER_CNTR::_100,
            5 => FILTER_CNTR::_101,
            6 => FILTER_CNTR::_110,
            7 => FILTER_CNTR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FILTER_CNTR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FILTER_CNTR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FILTER_CNTR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FILTER_CNTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FILTER_CNTR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FILTER_CNTR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FILTER_CNTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FILTER_CNTR::_111
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Analog Comparator is disabled."]
    _0,
    #[doc = "Analog Comparator is enabled."]
    _1,
}
impl ENR {
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
            ENR::_0 => false,
            ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::_0,
            true => ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENR::_1
    }
}
#[doc = "Possible values of the field `OPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPER {
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    _0,
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    _1,
}
impl OPER {
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
            OPER::_0 => false,
            OPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPER {
        match value {
            false => OPER::_0,
            true => OPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OPER::_1
    }
}
#[doc = "Possible values of the field `COS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COSR {
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    _0,
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    _1,
}
impl COSR {
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
            COSR::_0 => false,
            COSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COSR {
        match value {
            false => COSR::_0,
            true => COSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == COSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == COSR::_1
    }
}
#[doc = "Possible values of the field `INVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVTR {
    #[doc = "Does not invert the comparator output."]
    _0,
    #[doc = "Inverts the comparator output."]
    _1,
}
impl INVTR {
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
            INVTR::_0 => false,
            INVTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVTR {
        match value {
            false => INVTR::_0,
            true => INVTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVTR::_1
    }
}
#[doc = "Possible values of the field `PMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMODER {
    #[doc = "Low Speed (LS) comparison mode is selected."]
    _0,
    #[doc = "High Speed (HS) comparison mode is selected, in VLPx mode, or Stop mode switched to Low Speed (LS) mode."]
    _1,
}
impl PMODER {
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
            PMODER::_0 => false,
            PMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMODER {
        match value {
            false => PMODER::_0,
            true => PMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PMODER::_1
    }
}
#[doc = "Possible values of the field `WE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WER {
    #[doc = "Windowing mode is not selected."]
    _0,
    #[doc = "Windowing mode is selected."]
    _1,
}
impl WER {
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
            WER::_0 => false,
            WER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WER {
        match value {
            false => WER::_0,
            true => WER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WER::_1
    }
}
#[doc = "Possible values of the field `SE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SER {
    #[doc = "Sampling mode is not selected."]
    _0,
    #[doc = "Sampling mode is selected."]
    _1,
}
impl SER {
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
            SER::_0 => false,
            SER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SER {
        match value {
            false => SER::_0,
            true => SER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SER::_1
    }
}
#[doc = r" Value of the field"]
pub struct FPRR {
    bits: u8,
}
impl FPRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct COUTR {
    bits: bool,
}
impl COUTR {
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
#[doc = "Possible values of the field `CFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFFR {
    #[doc = "A falling edge has not been detected on COUT."]
    _0,
    #[doc = "A falling edge on COUT has occurred."]
    _1,
}
impl CFFR {
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
            CFFR::_0 => false,
            CFFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFFR {
        match value {
            false => CFFR::_0,
            true => CFFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CFFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CFFR::_1
    }
}
#[doc = "Possible values of the field `CFR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFRR {
    #[doc = "A rising edge has not been detected on COUT."]
    _0,
    #[doc = "A rising edge on COUT has occurred."]
    _1,
}
impl CFRR {
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
            CFRR::_0 => false,
            CFRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CFRR {
        match value {
            false => CFRR::_0,
            true => CFRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CFRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CFRR::_1
    }
}
#[doc = "Possible values of the field `IEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEFR {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IEFR {
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
            IEFR::_0 => false,
            IEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IEFR {
        match value {
            false => IEFR::_0,
            true => IEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IEFR::_1
    }
}
#[doc = "Possible values of the field `IER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IERR {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IERR {
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
            IERR::_0 => false,
            IERR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IERR {
        match value {
            false => IERR::_0,
            true => IERR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IERR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IERR::_1
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled."]
    _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = "Values that can be written to the field `HYSTCTR`"]
pub enum HYSTCTRW {
    #[doc = "The hard block output has level 0 hysteresis internally."]
    _00,
    #[doc = "The hard block output has level 1 hysteresis internally."]
    _01,
    #[doc = "The hard block output has level 2 hysteresis internally."]
    _10,
    #[doc = "The hard block output has level 3 hysteresis internally."]
    _11,
}
impl HYSTCTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HYSTCTRW::_00 => 0,
            HYSTCTRW::_01 => 1,
            HYSTCTRW::_10 => 2,
            HYSTCTRW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HYSTCTRW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTCTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HYSTCTRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The hard block output has level 0 hysteresis internally."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(HYSTCTRW::_00)
    }
    #[doc = "The hard block output has level 1 hysteresis internally."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(HYSTCTRW::_01)
    }
    #[doc = "The hard block output has level 2 hysteresis internally."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(HYSTCTRW::_10)
    }
    #[doc = "The hard block output has level 3 hysteresis internally."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(HYSTCTRW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OFFSET`"]
pub enum OFFSETW {
    #[doc = "The comparator hard block output has level 0 offset internally."]
    _0,
    #[doc = "The comparator hard block output has level 1 offset internally."]
    _1,
}
impl OFFSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFSETW::_0 => false,
            OFFSETW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The comparator hard block output has level 0 offset internally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OFFSETW::_0)
    }
    #[doc = "The comparator hard block output has level 1 offset internally."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OFFSETW::_1)
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
#[doc = "Values that can be written to the field `FILTER_CNT`"]
pub enum FILTER_CNTW {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    _000,
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    _001,
    #[doc = "2 consecutive samples must agree."]
    _010,
    #[doc = "3 consecutive samples must agree."]
    _011,
    #[doc = "4 consecutive samples must agree."]
    _100,
    #[doc = "5 consecutive samples must agree."]
    _101,
    #[doc = "6 consecutive samples must agree."]
    _110,
    #[doc = "7 consecutive samples must agree."]
    _111,
}
impl FILTER_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FILTER_CNTW::_000 => 0,
            FILTER_CNTW::_001 => 1,
            FILTER_CNTW::_010 => 2,
            FILTER_CNTW::_011 => 3,
            FILTER_CNTW::_100 => 4,
            FILTER_CNTW::_101 => 5,
            FILTER_CNTW::_110 => 6,
            FILTER_CNTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTER_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTER_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTER_CNTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_000)
    }
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_001)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_010)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_011)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_100)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_101)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_110)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FILTER_CNTW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Analog Comparator is disabled."]
    _0,
    #[doc = "Analog Comparator is enabled."]
    _1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::_0 => false,
            ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENW::_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENW::_1)
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
#[doc = "Values that can be written to the field `OPE`"]
pub enum OPEW {
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    _0,
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    _1,
}
impl OPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPEW::_0 => false,
            OPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPEW::_0)
    }
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPEW::_1)
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
#[doc = "Values that can be written to the field `COS`"]
pub enum COSW {
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    _0,
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    _1,
}
impl COSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COSW::_0 => false,
            COSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COSW<'a> {
    w: &'a mut W,
}
impl<'a> _COSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(COSW::_0)
    }
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(COSW::_1)
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
#[doc = "Values that can be written to the field `INVT`"]
pub enum INVTW {
    #[doc = "Does not invert the comparator output."]
    _0,
    #[doc = "Inverts the comparator output."]
    _1,
}
impl INVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVTW::_0 => false,
            INVTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVTW<'a> {
    w: &'a mut W,
}
impl<'a> _INVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Does not invert the comparator output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVTW::_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVTW::_1)
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
#[doc = "Values that can be written to the field `PMODE`"]
pub enum PMODEW {
    #[doc = "Low Speed (LS) comparison mode is selected."]
    _0,
    #[doc = "High Speed (HS) comparison mode is selected, in VLPx mode, or Stop mode switched to Low Speed (LS) mode."]
    _1,
}
impl PMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMODEW::_0 => false,
            PMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low Speed (LS) comparison mode is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PMODEW::_0)
    }
    #[doc = "High Speed (HS) comparison mode is selected, in VLPx mode, or Stop mode switched to Low Speed (LS) mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PMODEW::_1)
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
#[doc = "Values that can be written to the field `WE`"]
pub enum WEW {
    #[doc = "Windowing mode is not selected."]
    _0,
    #[doc = "Windowing mode is selected."]
    _1,
}
impl WEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WEW::_0 => false,
            WEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WEW<'a> {
    w: &'a mut W,
}
impl<'a> _WEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Windowing mode is not selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WEW::_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WEW::_1)
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
#[doc = "Values that can be written to the field `SE`"]
pub enum SEW {
    #[doc = "Sampling mode is not selected."]
    _0,
    #[doc = "Sampling mode is selected."]
    _1,
}
impl SEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEW::_0 => false,
            SEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEW<'a> {
    w: &'a mut W,
}
impl<'a> _SEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Sampling mode is not selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEW::_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEW::_1)
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
#[doc = r" Proxy"]
pub struct _FPRW<'a> {
    w: &'a mut W,
}
impl<'a> _FPRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CFF`"]
pub enum CFFW {
    #[doc = "A falling edge has not been detected on COUT."]
    _0,
    #[doc = "A falling edge on COUT has occurred."]
    _1,
}
impl CFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFFW::_0 => false,
            CFFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge has not been detected on COUT."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFFW::_0)
    }
    #[doc = "A falling edge on COUT has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFFW::_1)
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
#[doc = "Values that can be written to the field `CFR`"]
pub enum CFRW {
    #[doc = "A rising edge has not been detected on COUT."]
    _0,
    #[doc = "A rising edge on COUT has occurred."]
    _1,
}
impl CFRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CFRW::_0 => false,
            CFRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CFRW<'a> {
    w: &'a mut W,
}
impl<'a> _CFRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A rising edge has not been detected on COUT."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFRW::_0)
    }
    #[doc = "A rising edge on COUT has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFRW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IEF`"]
pub enum IEFW {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IEFW::_0 => false,
            IEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IEFW<'a> {
    w: &'a mut W,
}
impl<'a> _IEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEFW::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEFW::_1)
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
#[doc = "Values that can be written to the field `IER`"]
pub enum IERW {
    #[doc = "Interrupt is disabled."]
    _0,
    #[doc = "Interrupt is enabled."]
    _1,
}
impl IERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IERW::_0 => false,
            IERW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IERW<'a> {
    w: &'a mut W,
}
impl<'a> _IERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERW::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERW::_1)
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
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled."]
    _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline]
    pub fn hystctr(&self) -> HYSTCTRR {
        HYSTCTRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
    #[inline]
    pub fn offset(&self) -> OFFSETR {
        OFFSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&self) -> FILTER_CNTR {
        FILTER_CNTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline]
    pub fn ope(&self) -> OPER {
        OPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline]
    pub fn cos(&self) -> COSR {
        COSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline]
    pub fn invt(&self) -> INVTR {
        INVTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline]
    pub fn pmode(&self) -> PMODER {
        PMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline]
    pub fn we(&self) -> WER {
        WER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline]
    pub fn se(&self) -> SER {
        SER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline]
    pub fn fpr(&self) -> FPRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FPRR { bits }
    }
    #[doc = "Bit 24 - Analog Comparator Output"]
    #[inline]
    pub fn cout(&self) -> COUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COUTR { bits }
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&self) -> CFFR {
        CFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&self) -> CFRR {
        CFRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&self) -> IEFR {
        IEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&self) -> IERR {
        IERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control. See chip data sheet to get the actual hystersis value with each level"]
    #[inline]
    pub fn hystctr(&mut self) -> _HYSTCTRW {
        _HYSTCTRW { w: self }
    }
    #[doc = "Bit 2 - Comparator hard block offset control. See chip data sheet to get the actual offset value with each level"]
    #[inline]
    pub fn offset(&mut self) -> _OFFSETW {
        _OFFSETW { w: self }
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline]
    pub fn filter_cnt(&mut self) -> _FILTER_CNTW {
        _FILTER_CNTW { w: self }
    }
    #[doc = "Bit 8 - Comparator Module Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 9 - Comparator Output Pin Enable"]
    #[inline]
    pub fn ope(&mut self) -> _OPEW {
        _OPEW { w: self }
    }
    #[doc = "Bit 10 - Comparator Output Select"]
    #[inline]
    pub fn cos(&mut self) -> _COSW {
        _COSW { w: self }
    }
    #[doc = "Bit 11 - Comparator invert"]
    #[inline]
    pub fn invt(&mut self) -> _INVTW {
        _INVTW { w: self }
    }
    #[doc = "Bit 12 - Power Mode Select"]
    #[inline]
    pub fn pmode(&mut self) -> _PMODEW {
        _PMODEW { w: self }
    }
    #[doc = "Bit 14 - Windowing Enable"]
    #[inline]
    pub fn we(&mut self) -> _WEW {
        _WEW { w: self }
    }
    #[doc = "Bit 15 - Sample Enable"]
    #[inline]
    pub fn se(&mut self) -> _SEW {
        _SEW { w: self }
    }
    #[doc = "Bits 16:23 - Filter Sample Period"]
    #[inline]
    pub fn fpr(&mut self) -> _FPRW {
        _FPRW { w: self }
    }
    #[doc = "Bit 25 - Analog Comparator Flag Falling"]
    #[inline]
    pub fn cff(&mut self) -> _CFFW {
        _CFFW { w: self }
    }
    #[doc = "Bit 26 - Analog Comparator Flag Rising"]
    #[inline]
    pub fn cfr(&mut self) -> _CFRW {
        _CFRW { w: self }
    }
    #[doc = "Bit 27 - Comparator Interrupt Enable Falling"]
    #[inline]
    pub fn ief(&mut self) -> _IEFW {
        _IEFW { w: self }
    }
    #[doc = "Bit 28 - Comparator Interrupt Enable Rising"]
    #[inline]
    pub fn ier(&mut self) -> _IERW {
        _IERW { w: self }
    }
    #[doc = "Bit 30 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
