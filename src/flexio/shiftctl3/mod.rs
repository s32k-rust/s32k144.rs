#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHIFTCTL3 {
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
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Disabled."] _0,
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    _1,
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    _10,
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    _100,
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    _101,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::_0 => 0,
            SMODR::_1 => 1,
            SMODR::_10 => 2,
            SMODR::_100 => 4,
            SMODR::_101 => 5,
            SMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::_0,
            1 => SMODR::_1,
            2 => SMODR::_10,
            4 => SMODR::_100,
            5 => SMODR::_101,
            i => SMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMODR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMODR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SMODR::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SMODR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SMODR::_101
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
    #[doc = "Shifter pin output disabled"] _0,
    #[doc = "Shifter pin open drain or bidirectional output enable"] _1,
    #[doc = "Shifter pin bidirectional output data"] _10,
    #[doc = "Shifter pin output"] _11,
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
#[doc = "Possible values of the field `TIMPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPOLR {
    #[doc = "Shift on posedge of Shift clock"] _0,
    #[doc = "Shift on negedge of Shift clock"] _1,
}
impl TIMPOLR {
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
            TIMPOLR::_0 => false,
            TIMPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMPOLR {
        match value {
            false => TIMPOLR::_0,
            true => TIMPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMPOLR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TIMSELR {
    bits: u8,
}
impl TIMSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
pub enum SMODW {
    #[doc = "Disabled."] _0,
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    _1,
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    _10,
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    _100,
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    _101,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::_0 => 0,
            SMODW::_1 => 1,
            SMODW::_10 => 2,
            SMODW::_100 => 4,
            SMODW::_101 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMODW::_0)
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMODW::_1)
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMODW::_10)
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SMODW::_100)
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SMODW::_101)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Shifter pin output disabled"] _0,
    #[doc = "Shifter pin open drain or bidirectional output enable"] _1,
    #[doc = "Shifter pin bidirectional output data"] _10,
    #[doc = "Shifter pin output"] _11,
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
    #[doc = "Shifter pin output disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINCFGW::_0)
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINCFGW::_1)
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINCFGW::_10)
    }
    #[doc = "Shifter pin output"]
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
#[doc = "Values that can be written to the field `TIMPOL`"]
pub enum TIMPOLW {
    #[doc = "Shift on posedge of Shift clock"] _0,
    #[doc = "Shift on negedge of Shift clock"] _1,
}
impl TIMPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMPOLW::_0 => false,
            TIMPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shift on posedge of Shift clock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMPOLW::_0)
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMPOLW::_1)
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
pub struct _TIMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline]
    pub fn pinpol(&self) -> PINPOLR {
        PINPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline]
    pub fn pinsel(&self) -> PINSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PINSELR { bits }
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline]
    pub fn timpol(&self) -> TIMPOLR {
        TIMPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline]
    pub fn timsel(&self) -> TIMSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMSELR { bits }
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
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline]
    pub fn pinpol(&mut self) -> _PINPOLW {
        _PINPOLW { w: self }
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline]
    pub fn pinsel(&mut self) -> _PINSELW {
        _PINSELW { w: self }
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline]
    pub fn timpol(&mut self) -> _TIMPOLW {
        _TIMPOLW { w: self }
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline]
    pub fn timsel(&mut self) -> _TIMSELW {
        _TIMSELW { w: self }
    }
}
