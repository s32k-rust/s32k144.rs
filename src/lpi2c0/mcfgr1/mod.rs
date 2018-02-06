#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFGR1 {
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
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 4."]
    _010,
    #[doc = "Divide by 8."]
    _011,
    #[doc = "Divide by 16."]
    _100,
    #[doc = "Divide by 32."]
    _101,
    #[doc = "Divide by 64."]
    _110,
    #[doc = "Divide by 128."]
    _111,
}
impl PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER::_000 => 0,
            PRESCALER::_001 => 1,
            PRESCALER::_010 => 2,
            PRESCALER::_011 => 3,
            PRESCALER::_100 => 4,
            PRESCALER::_101 => 5,
            PRESCALER::_110 => 6,
            PRESCALER::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER {
        match value {
            0 => PRESCALER::_000,
            1 => PRESCALER::_001,
            2 => PRESCALER::_010,
            3 => PRESCALER::_011,
            4 => PRESCALER::_100,
            5 => PRESCALER::_101,
            6 => PRESCALER::_110,
            7 => PRESCALER::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PRESCALER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PRESCALER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PRESCALER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PRESCALER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PRESCALER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PRESCALER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PRESCALER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PRESCALER::_111
    }
}
#[doc = "Possible values of the field `AUTOSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSTOPR {
    #[doc = "No effect."]
    _0,
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and LPI2C master is busy."]
    _1,
}
impl AUTOSTOPR {
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
            AUTOSTOPR::_0 => false,
            AUTOSTOPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOSTOPR {
        match value {
            false => AUTOSTOPR::_0,
            true => AUTOSTOPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AUTOSTOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AUTOSTOPR::_1
    }
}
#[doc = "Possible values of the field `IGNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACKR {
    #[doc = "LPI2C Master will receive ACK and NACK normally."]
    _0,
    #[doc = "LPI2C Master will treat a received NACK as if it was an ACK."]
    _1,
}
impl IGNACKR {
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
            IGNACKR::_0 => false,
            IGNACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IGNACKR {
        match value {
            false => IGNACKR::_0,
            true => IGNACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IGNACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IGNACKR::_1
    }
}
#[doc = "Possible values of the field `TIMECFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMECFGR {
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout."]
    _0,
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout."]
    _1,
}
impl TIMECFGR {
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
            TIMECFGR::_0 => false,
            TIMECFGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMECFGR {
        match value {
            false => TIMECFGR::_0,
            true => TIMECFGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMECFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMECFGR::_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Match disabled."]
    _000,
    #[doc = "Match enabled (1st data word equals MATCH0 OR MATCH1)."]
    _010,
    #[doc = "Match enabled (any data word equals MATCH0 OR MATCH1)."]
    _011,
    #[doc = "Match enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)."]
    _100,
    #[doc = "Match enabled (any data word equals MATCH0 AND next data word equals MATCH1)."]
    _101,
    #[doc = "Match enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _110,
    #[doc = "Match enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::_000 => 0,
            MATCFGR::_010 => 2,
            MATCFGR::_011 => 3,
            MATCFGR::_100 => 4,
            MATCFGR::_101 => 5,
            MATCFGR::_110 => 6,
            MATCFGR::_111 => 7,
            MATCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::_000,
            2 => MATCFGR::_010,
            3 => MATCFGR::_011,
            4 => MATCFGR::_100,
            5 => MATCFGR::_101,
            6 => MATCFGR::_110,
            7 => MATCFGR::_111,
            i => MATCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MATCFGR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MATCFGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MATCFGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == MATCFGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == MATCFGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == MATCFGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == MATCFGR::_111
    }
}
#[doc = "Possible values of the field `PINCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINCFGR {
    #[doc = "LPI2C configured for 2-pin open drain mode."]
    _000,
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode)."]
    _001,
    #[doc = "LPI2C configured for 2-pin push-pull mode."]
    _010,
    #[doc = "LPI2C configured for 4-pin push-pull mode."]
    _011,
    #[doc = "LPI2C configured for 2-pin open drain mode with separate LPI2C slave."]
    _100,
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode) with separate LPI2C slave."]
    _101,
    #[doc = "LPI2C configured for 2-pin push-pull mode with separate LPI2C slave."]
    _110,
    #[doc = "LPI2C configured for 4-pin push-pull mode (inverted outputs)."]
    _111,
}
impl PINCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PINCFGR::_000 => 0,
            PINCFGR::_001 => 1,
            PINCFGR::_010 => 2,
            PINCFGR::_011 => 3,
            PINCFGR::_100 => 4,
            PINCFGR::_101 => 5,
            PINCFGR::_110 => 6,
            PINCFGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PINCFGR {
        match value {
            0 => PINCFGR::_000,
            1 => PINCFGR::_001,
            2 => PINCFGR::_010,
            3 => PINCFGR::_011,
            4 => PINCFGR::_100,
            5 => PINCFGR::_101,
            6 => PINCFGR::_110,
            7 => PINCFGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PINCFGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PINCFGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PINCFGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PINCFGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PINCFGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PINCFGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PINCFGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PINCFGR::_111
    }
}
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 4."]
    _010,
    #[doc = "Divide by 8."]
    _011,
    #[doc = "Divide by 16."]
    _100,
    #[doc = "Divide by 32."]
    _101,
    #[doc = "Divide by 64."]
    _110,
    #[doc = "Divide by 128."]
    _111,
}
impl PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALEW::_000 => 0,
            PRESCALEW::_001 => 1,
            PRESCALEW::_010 => 2,
            PRESCALEW::_011 => 3,
            PRESCALEW::_100 => 4,
            PRESCALEW::_101 => 5,
            PRESCALEW::_110 => 6,
            PRESCALEW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALEW::_000)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALEW::_001)
    }
    #[doc = "Divide by 4."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALEW::_010)
    }
    #[doc = "Divide by 8."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALEW::_011)
    }
    #[doc = "Divide by 16."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALEW::_100)
    }
    #[doc = "Divide by 32."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALEW::_101)
    }
    #[doc = "Divide by 64."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALEW::_110)
    }
    #[doc = "Divide by 128."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALEW::_111)
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
#[doc = "Values that can be written to the field `AUTOSTOP`"]
pub enum AUTOSTOPW {
    #[doc = "No effect."]
    _0,
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and LPI2C master is busy."]
    _1,
}
impl AUTOSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOSTOPW::_0 => false,
            AUTOSTOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOSTOPW::_0)
    }
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and LPI2C master is busy."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOSTOPW::_1)
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
#[doc = "Values that can be written to the field `IGNACK`"]
pub enum IGNACKW {
    #[doc = "LPI2C Master will receive ACK and NACK normally."]
    _0,
    #[doc = "LPI2C Master will treat a received NACK as if it was an ACK."]
    _1,
}
impl IGNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IGNACKW::_0 => false,
            IGNACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IGNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IGNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPI2C Master will receive ACK and NACK normally."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGNACKW::_0)
    }
    #[doc = "LPI2C Master will treat a received NACK as if it was an ACK."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGNACKW::_1)
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
#[doc = "Values that can be written to the field `TIMECFG`"]
pub enum TIMECFGW {
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout."]
    _0,
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout."]
    _1,
}
impl TIMECFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMECFGW::_0 => false,
            TIMECFGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMECFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMECFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMECFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMECFGW::_0)
    }
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMECFGW::_1)
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
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Match disabled."]
    _000,
    #[doc = "Match enabled (1st data word equals MATCH0 OR MATCH1)."]
    _010,
    #[doc = "Match enabled (any data word equals MATCH0 OR MATCH1)."]
    _011,
    #[doc = "Match enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)."]
    _100,
    #[doc = "Match enabled (any data word equals MATCH0 AND next data word equals MATCH1)."]
    _101,
    #[doc = "Match enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _110,
    #[doc = "Match enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _111,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::_000 => 0,
            MATCFGW::_010 => 2,
            MATCFGW::_011 => 3,
            MATCFGW::_100 => 4,
            MATCFGW::_101 => 5,
            MATCFGW::_110 => 6,
            MATCFGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Match disabled."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(MATCFGW::_000)
    }
    #[doc = "Match enabled (1st data word equals MATCH0 OR MATCH1)."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(MATCFGW::_010)
    }
    #[doc = "Match enabled (any data word equals MATCH0 OR MATCH1)."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(MATCFGW::_011)
    }
    #[doc = "Match enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(MATCFGW::_100)
    }
    #[doc = "Match enabled (any data word equals MATCH0 AND next data word equals MATCH1)."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(MATCFGW::_101)
    }
    #[doc = "Match enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(MATCFGW::_110)
    }
    #[doc = "Match enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(MATCFGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINCFG`"]
pub enum PINCFGW {
    #[doc = "LPI2C configured for 2-pin open drain mode."]
    _000,
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode)."]
    _001,
    #[doc = "LPI2C configured for 2-pin push-pull mode."]
    _010,
    #[doc = "LPI2C configured for 4-pin push-pull mode."]
    _011,
    #[doc = "LPI2C configured for 2-pin open drain mode with separate LPI2C slave."]
    _100,
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode) with separate LPI2C slave."]
    _101,
    #[doc = "LPI2C configured for 2-pin push-pull mode with separate LPI2C slave."]
    _110,
    #[doc = "LPI2C configured for 4-pin push-pull mode (inverted outputs)."]
    _111,
}
impl PINCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PINCFGW::_000 => 0,
            PINCFGW::_001 => 1,
            PINCFGW::_010 => 2,
            PINCFGW::_011 => 3,
            PINCFGW::_100 => 4,
            PINCFGW::_101 => 5,
            PINCFGW::_110 => 6,
            PINCFGW::_111 => 7,
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
    #[doc = "LPI2C configured for 2-pin open drain mode."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PINCFGW::_000)
    }
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode)."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PINCFGW::_001)
    }
    #[doc = "LPI2C configured for 2-pin push-pull mode."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PINCFGW::_010)
    }
    #[doc = "LPI2C configured for 4-pin push-pull mode."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PINCFGW::_011)
    }
    #[doc = "LPI2C configured for 2-pin open drain mode with separate LPI2C slave."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PINCFGW::_100)
    }
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode) with separate LPI2C slave."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PINCFGW::_101)
    }
    #[doc = "LPI2C configured for 2-pin push-pull mode with separate LPI2C slave."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PINCFGW::_110)
    }
    #[doc = "LPI2C configured for 4-pin push-pull mode (inverted outputs)."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PINCFGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline]
    pub fn autostop(&self) -> AUTOSTOPR {
        AUTOSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline]
    pub fn ignack(&self) -> IGNACKR {
        IGNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline]
    pub fn timecfg(&self) -> TIMECFGR {
        TIMECFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&self) -> PINCFGR {
        PINCFGR::_from({
            const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline]
    pub fn autostop(&mut self) -> _AUTOSTOPW {
        _AUTOSTOPW { w: self }
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline]
    pub fn ignack(&mut self) -> _IGNACKW {
        _IGNACKW { w: self }
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline]
    pub fn timecfg(&mut self) -> _TIMECFGW {
        _TIMECFGW { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline]
    pub fn pincfg(&mut self) -> _PINCFGW {
        _PINCFGW { w: self }
    }
}
