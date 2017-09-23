#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCTL2 {
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
#[doc = "Possible values of the field `TIMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMODR {
    #[doc = "Timer Disabled."] _0,
    #[doc = "Dual 8-bit counters baud/bit mode."] _1,
    #[doc = "Dual 8-bit counters PWM mode."] _10,
    #[doc = "Single 16-bit counter mode."] _11,
}
impl TIMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMODR::_0 => 0,
            TIMODR::_1 => 1,
            TIMODR::_10 => 2,
            TIMODR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMODR {
        match value {
            0 => TIMODR::_0,
            1 => TIMODR::_1,
            2 => TIMODR::_10,
            3 => TIMODR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMODR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMODR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMODR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMODR::_11
    }
}
#[doc = "Possible values of the field `PINPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOLR {
    #[doc = "Pin is active high"] _0,
    #[doc = "Pin is active low"] _1,
}
impl PINPOLR {
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
            PINPOLR::_0 => false,
            PINPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINPOLR {
        match value {
            false => PINPOLR::_0,
            true => PINPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PINPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PINPOLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PINSELR {
    bits: u8,
}
impl PINSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "Timer pin output disabled"] _0,
    #[doc = "Timer pin open drain or bidirectional output enable"] _1,
    #[doc = "Timer pin bidirectional output data"] _10,
    #[doc = "Timer pin output"] _11,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::_0 => 0,
            PINCFGR::_1 => 1,
            PINCFGR::_10 => 2,
            PINCFGR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::_0,
            1 => PINCFGR::_1,
            2 => PINCFGR::_10,
            3 => PINCFGR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PINCFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PINCFGR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PINCFGR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PINCFGR::_11
    }
}
#[doc = "Possible values of the field `TRGSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRCR {
    #[doc = "External trigger selected"] _0,
    #[doc = "Internal trigger selected"] _1,
}
impl TRGSRCR {
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
            TRGSRCR::_0 => false,
            TRGSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGSRCR {
        match value {
            false => TRGSRCR::_0,
            true => TRGSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGSRCR::_1
    }
}
#[doc = "Possible values of the field `TRGPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOLR {
    #[doc = "Trigger active high"] _0,
    #[doc = "Trigger active low"] _1,
}
impl TRGPOLR {
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
            TRGPOLR::_0 => false,
            TRGPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGPOLR {
        match value {
            false => TRGPOLR::_0,
            true => TRGPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGPOLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TRGSELR {
    bits: u8,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TIMOD`"]
pub enum TIMODW {
    #[doc = "Timer Disabled."] _0,
    #[doc = "Dual 8-bit counters baud/bit mode."] _1,
    #[doc = "Dual 8-bit counters PWM mode."] _10,
    #[doc = "Single 16-bit counter mode."] _11,
}
impl TIMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMODW::_0 => 0,
            TIMODW::_1 => 1,
            TIMODW::_10 => 2,
            TIMODW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMODW::_0)
    }
    #[doc = "Dual 8-bit counters baud/bit mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMODW::_1)
    }
    #[doc = "Dual 8-bit counters PWM mode."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMODW::_10)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMODW::_11)
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
#[doc = "Values that can be written to the field `PINPOL`"]
pub enum PINPOLW {
    #[doc = "Pin is active high"] _0,
    #[doc = "Pin is active low"] _1,
}
impl PINPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINPOLW::_0 => false,
            PINPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _PINPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin is active high"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINPOLW::_0)
    }
    #[doc = "Pin is active low"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINPOLW::_1)
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
#[doc = r" Proxy"]
pub struct _PINSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PINSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "Timer pin output disabled"] _0,
    #[doc = "Timer pin open drain or bidirectional output enable"] _1,
    #[doc = "Timer pin bidirectional output data"] _10,
    #[doc = "Timer pin output"] _11,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::_0 => 0,
            PINCFGW::_1 => 1,
            PINCFGW::_10 => 2,
            PINCFGW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer pin output disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINCFGW::_0)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINCFGW::_1)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINCFGW::_10)
    }
    #[doc = "Timer pin output"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PINCFGW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGSRC`"]
pub enum TRGSRCW {
    #[doc = "External trigger selected"] _0,
    #[doc = "Internal trigger selected"] _1,
}
impl TRGSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGSRCW::_0 => false,
            TRGSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger selected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSRCW::_0)
    }
    #[doc = "Internal trigger selected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSRCW::_1)
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
#[doc = "Values that can be written to the field `TRGPOL`"]
pub enum TRGPOLW {
    #[doc = "Trigger active high"] _0,
    #[doc = "Trigger active low"] _1,
}
impl TRGPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGPOLW::_0 => false,
            TRGPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger active high"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPOLW::_0)
    }
    #[doc = "Trigger active low"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPOLW::_1)
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
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn timod(&self) -> TIMODR {
        TIMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline]
    pub fn pinpol(&self) -> PINPOLR {
        PINPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline]
    pub fn pinsel(&self) -> PINSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PINSELR { bits }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&self) -> TRGSRCR {
        TRGSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&self) -> TRGPOLR {
        TRGPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGSELR { bits }
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
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline]
    pub fn timod(&mut self) -> _TIMODW {
        _TIMODW { w: self }
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline]
    pub fn pinpol(&mut self) -> _PINPOLW {
        _PINPOLW { w: self }
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline]
    pub fn pinsel(&mut self) -> _PINSELW {
        _PINSELW { w: self }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&mut self) -> _TRGSRCW {
        _TRGSRCW { w: self }
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline]
    pub fn trgpol(&mut self) -> _TRGPOLW {
        _TRGPOLW { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
