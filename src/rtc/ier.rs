#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `TIIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIIER {
    #[doc = "Time invalid flag does not generate an interrupt."]
    _0,
    #[doc = "Time invalid flag does generate an interrupt."]
    _1,
}
impl TIIER {
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
            TIIER::_0 => false,
            TIIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIIER {
        match value {
            false => TIIER::_0,
            true => TIIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIIER::_1
    }
}
#[doc = "Possible values of the field `TOIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIER {
    #[doc = "Time overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Time overflow flag does generate an interrupt."]
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
#[doc = "Possible values of the field `TAIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIER {
    #[doc = "Time alarm flag does not generate an interrupt."]
    _0,
    #[doc = "Time alarm flag does generate an interrupt."]
    _1,
}
impl TAIER {
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
            TAIER::_0 => false,
            TAIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAIER {
        match value {
            false => TAIER::_0,
            true => TAIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TAIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TAIER::_1
    }
}
#[doc = "Possible values of the field `TSIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIER {
    #[doc = "Seconds interrupt is disabled."]
    _0,
    #[doc = "Seconds interrupt is enabled."]
    _1,
}
impl TSIER {
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
            TSIER::_0 => false,
            TSIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSIER {
        match value {
            false => TSIER::_0,
            true => TSIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSIER::_1
    }
}
#[doc = "Possible values of the field `TSIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSICR {
    #[doc = "1 Hz."]
    _000,
    #[doc = "2 Hz."]
    _001,
    #[doc = "4 Hz."]
    _010,
    #[doc = "8 Hz."]
    _011,
    #[doc = "16 Hz."]
    _100,
    #[doc = "32 Hz."]
    _101,
    #[doc = "64 Hz."]
    _110,
    #[doc = "128 Hz."]
    _111,
}
impl TSICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSICR::_000 => 0,
            TSICR::_001 => 1,
            TSICR::_010 => 2,
            TSICR::_011 => 3,
            TSICR::_100 => 4,
            TSICR::_101 => 5,
            TSICR::_110 => 6,
            TSICR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSICR {
        match value {
            0 => TSICR::_000,
            1 => TSICR::_001,
            2 => TSICR::_010,
            3 => TSICR::_011,
            4 => TSICR::_100,
            5 => TSICR::_101,
            6 => TSICR::_110,
            7 => TSICR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TSICR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TSICR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TSICR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TSICR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TSICR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TSICR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TSICR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TSICR::_111
    }
}
#[doc = "Values that can be written to the field `TIIE`"]
pub enum TIIEW {
    #[doc = "Time invalid flag does not generate an interrupt."]
    _0,
    #[doc = "Time invalid flag does generate an interrupt."]
    _1,
}
impl TIIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIIEW::_0 => false,
            TIIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIIEW::_0)
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIIEW::_1)
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
#[doc = "Values that can be written to the field `TOIE`"]
pub enum TOIEW {
    #[doc = "Time overflow flag does not generate an interrupt."]
    _0,
    #[doc = "Time overflow flag does generate an interrupt."]
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
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIEW::_0)
    }
    #[doc = "Time overflow flag does generate an interrupt."]
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TAIE`"]
pub enum TAIEW {
    #[doc = "Time alarm flag does not generate an interrupt."]
    _0,
    #[doc = "Time alarm flag does generate an interrupt."]
    _1,
}
impl TAIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAIEW::_0 => false,
            TAIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TAIEW::_0)
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TAIEW::_1)
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
#[doc = "Values that can be written to the field `TSIE`"]
pub enum TSIEW {
    #[doc = "Seconds interrupt is disabled."]
    _0,
    #[doc = "Seconds interrupt is enabled."]
    _1,
}
impl TSIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSIEW::_0 => false,
            TSIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Seconds interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIEW::_0)
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIEW::_1)
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
#[doc = "Values that can be written to the field `TSIC`"]
pub enum TSICW {
    #[doc = "1 Hz."]
    _000,
    #[doc = "2 Hz."]
    _001,
    #[doc = "4 Hz."]
    _010,
    #[doc = "8 Hz."]
    _011,
    #[doc = "16 Hz."]
    _100,
    #[doc = "32 Hz."]
    _101,
    #[doc = "64 Hz."]
    _110,
    #[doc = "128 Hz."]
    _111,
}
impl TSICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSICW::_000 => 0,
            TSICW::_001 => 1,
            TSICW::_010 => 2,
            TSICW::_011 => 3,
            TSICW::_100 => 4,
            TSICW::_101 => 5,
            TSICW::_110 => 6,
            TSICW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSICW<'a> {
    w: &'a mut W,
}
impl<'a> _TSICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSICW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 Hz."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(TSICW::_000)
    }
    #[doc = "2 Hz."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(TSICW::_001)
    }
    #[doc = "4 Hz."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(TSICW::_010)
    }
    #[doc = "8 Hz."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(TSICW::_011)
    }
    #[doc = "16 Hz."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TSICW::_100)
    }
    #[doc = "32 Hz."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TSICW::_101)
    }
    #[doc = "64 Hz."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TSICW::_110)
    }
    #[doc = "128 Hz."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TSICW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline]
    pub fn tiie(&self) -> TIIER {
        TIIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&self) -> TOIER {
        TOIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline]
    pub fn taie(&self) -> TAIER {
        TAIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline]
    pub fn tsie(&self) -> TSIER {
        TSIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Timer Seconds Interrupt Configuration"]
    #[inline]
    pub fn tsic(&self) -> TSICR {
        TSICR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 7 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline]
    pub fn tiie(&mut self) -> _TIIEW {
        _TIIEW { w: self }
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline]
    pub fn toie(&mut self) -> _TOIEW {
        _TOIEW { w: self }
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline]
    pub fn taie(&mut self) -> _TAIEW {
        _TAIEW { w: self }
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline]
    pub fn tsie(&mut self) -> _TSIEW {
        _TSIEW { w: self }
    }
    #[doc = "Bits 16:18 - Timer Seconds Interrupt Configuration"]
    #[inline]
    pub fn tsic(&mut self) -> _TSICW {
        _TSICW { w: self }
    }
}
