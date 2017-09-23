#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC3 {
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
#[doc = "Possible values of the field `AVGS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGSR {
    #[doc = "4 samples averaged."]
    _00,
    #[doc = "8 samples averaged."]
    _01,
    #[doc = "16 samples averaged."]
    _10,
    #[doc = "32 samples averaged."]
    _11,
}
impl AVGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AVGSR::_00 => 0,
            AVGSR::_01 => 1,
            AVGSR::_10 => 2,
            AVGSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AVGSR {
        match value {
            0 => AVGSR::_00,
            1 => AVGSR::_01,
            2 => AVGSR::_10,
            3 => AVGSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == AVGSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == AVGSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == AVGSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == AVGSR::_11
    }
}
#[doc = "Possible values of the field `AVGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVGER {
    #[doc = "Hardware average function disabled."]
    _0,
    #[doc = "Hardware average function enabled."]
    _1,
}
impl AVGER {
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
            AVGER::_0 => false,
            AVGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVGER {
        match value {
            false => AVGER::_0,
            true => AVGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVGER::_1
    }
}
#[doc = "Possible values of the field `ADCO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCOR {
    #[doc = "One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated."]
    _0,
    #[doc = "Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated."]
    _1,
}
impl ADCOR {
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
            ADCOR::_0 => false,
            ADCOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADCOR {
        match value {
            false => ADCOR::_0,
            true => ADCOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADCOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADCOR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CALR {
    bits: bool,
}
impl CALR {
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
#[doc = "Values that can be written to the field `AVGS`"]
pub enum AVGSW {
    #[doc = "4 samples averaged."]
    _00,
    #[doc = "8 samples averaged."]
    _01,
    #[doc = "16 samples averaged."]
    _10,
    #[doc = "32 samples averaged."]
    _11,
}
impl AVGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AVGSW::_00 => 0,
            AVGSW::_01 => 1,
            AVGSW::_10 => 2,
            AVGSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGSW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4 samples averaged."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(AVGSW::_00)
    }
    #[doc = "8 samples averaged."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(AVGSW::_01)
    }
    #[doc = "16 samples averaged."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(AVGSW::_10)
    }
    #[doc = "32 samples averaged."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(AVGSW::_11)
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
#[doc = "Values that can be written to the field `AVGE`"]
pub enum AVGEW {
    #[doc = "Hardware average function disabled."]
    _0,
    #[doc = "Hardware average function enabled."]
    _1,
}
impl AVGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVGEW::_0 => false,
            AVGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVGEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware average function disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVGEW::_0)
    }
    #[doc = "Hardware average function enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVGEW::_1)
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
#[doc = "Values that can be written to the field `ADCO`"]
pub enum ADCOW {
    #[doc = "One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated."]
    _0,
    #[doc = "Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated."]
    _1,
}
impl ADCOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADCOW::_0 => false,
            ADCOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADCOW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADCOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One conversion will be performed (or one set of conversions, if AVGE is set) after a conversion is initiated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCOW::_0)
    }
    #[doc = "Continuous conversions will be performed (or continuous sets of conversions, if AVGE is set) after a conversion is initiated."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCOW::_1)
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
#[doc = r" Proxy"]
pub struct _CALW<'a> {
    w: &'a mut W,
}
impl<'a> _CALW<'a> {
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
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline]
    pub fn avgs(&self) -> AVGSR {
        AVGSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline]
    pub fn avge(&self) -> AVGER {
        AVGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline]
    pub fn adco(&self) -> ADCOR {
        ADCOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline]
    pub fn cal(&self) -> CALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CALR { bits }
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
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline]
    pub fn avgs(&mut self) -> _AVGSW {
        _AVGSW { w: self }
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline]
    pub fn avge(&mut self) -> _AVGEW {
        _AVGEW { w: self }
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline]
    pub fn adco(&mut self) -> _ADCOW {
        _ADCOW { w: self }
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline]
    pub fn cal(&mut self) -> _CALW {
        _CALW { w: self }
    }
}
