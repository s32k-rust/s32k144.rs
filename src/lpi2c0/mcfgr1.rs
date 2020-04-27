#[doc = "Reader of register MCFGR1"]
pub type R = crate::R<u32, super::MCFGR1>;
#[doc = "Writer for register MCFGR1"]
pub type W = crate::W<u32, super::MCFGR1>;
#[doc = "Register MCFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MCFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Divide by 1."]
    _000 = 0,
    #[doc = "1: Divide by 2."]
    _001 = 1,
    #[doc = "2: Divide by 4."]
    _010 = 2,
    #[doc = "3: Divide by 8."]
    _011 = 3,
    #[doc = "4: Divide by 16."]
    _100 = 4,
    #[doc = "5: Divide by 32."]
    _101 = 5,
    #[doc = "6: Divide by 64."]
    _110 = 6,
    #[doc = "7: Divide by 128."]
    _111 = 7,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::_000,
            1 => PRESCALE_A::_001,
            2 => PRESCALE_A::_010,
            3 => PRESCALE_A::_011,
            4 => PRESCALE_A::_100,
            5 => PRESCALE_A::_101,
            6 => PRESCALE_A::_110,
            7 => PRESCALE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PRESCALE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PRESCALE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PRESCALE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PRESCALE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PRESCALE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PRESCALE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PRESCALE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PRESCALE_A::_111
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALE_A::_000)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALE_A::_001)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALE_A::_010)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALE_A::_011)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALE_A::_100)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALE_A::_101)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALE_A::_110)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALE_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Automatic STOP Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOSTOP_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: STOP condition is automatically generated whenever the transmit FIFO is empty and LPI2C master is busy."]
    _1 = 1,
}
impl From<AUTOSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOSTOP`"]
pub type AUTOSTOP_R = crate::R<bool, AUTOSTOP_A>;
impl AUTOSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOSTOP_A {
        match self.bits {
            false => AUTOSTOP_A::_0,
            true => AUTOSTOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AUTOSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AUTOSTOP_A::_1
    }
}
#[doc = "Write proxy for field `AUTOSTOP`"]
pub struct AUTOSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::_0)
    }
    #[doc = "STOP condition is automatically generated whenever the transmit FIFO is empty and LPI2C master is busy."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AUTOSTOP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "IGNACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACK_A {
    #[doc = "0: LPI2C Master will receive ACK and NACK normally."]
    _0 = 0,
    #[doc = "1: LPI2C Master will treat a received NACK as if it was an ACK."]
    _1 = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IGNACK`"]
pub type IGNACK_R = crate::R<bool, IGNACK_A>;
impl IGNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::_0,
            true => IGNACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGNACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGNACK_A::_1
    }
}
#[doc = "Write proxy for field `IGNACK`"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPI2C Master will receive ACK and NACK normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGNACK_A::_0)
    }
    #[doc = "LPI2C Master will treat a received NACK as if it was an ACK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGNACK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Timeout Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMECFG_A {
    #[doc = "0: Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout."]
    _0 = 0,
    #[doc = "1: Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout."]
    _1 = 1,
}
impl From<TIMECFG_A> for bool {
    #[inline(always)]
    fn from(variant: TIMECFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMECFG`"]
pub type TIMECFG_R = crate::R<bool, TIMECFG_A>;
impl TIMECFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMECFG_A {
        match self.bits {
            false => TIMECFG_A::_0,
            true => TIMECFG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMECFG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMECFG_A::_1
    }
}
#[doc = "Write proxy for field `TIMECFG`"]
pub struct TIMECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMECFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin Low Timeout Flag will set if SCL is low for longer than the configured timeout."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMECFG_A::_0)
    }
    #[doc = "Pin Low Timeout Flag will set if either SCL or SDA is low for longer than the configured timeout."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMECFG_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MATCFG_A {
    #[doc = "0: Match disabled."]
    _000 = 0,
    #[doc = "2: Match enabled (1st data word equals MATCH0 OR MATCH1)."]
    _010 = 2,
    #[doc = "3: Match enabled (any data word equals MATCH0 OR MATCH1)."]
    _011 = 3,
    #[doc = "4: Match enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)."]
    _100 = 4,
    #[doc = "5: Match enabled (any data word equals MATCH0 AND next data word equals MATCH1)."]
    _101 = 5,
    #[doc = "6: Match enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _110 = 6,
    #[doc = "7: Match enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    _111 = 7,
}
impl From<MATCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: MATCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MATCFG`"]
pub type MATCFG_R = crate::R<u8, MATCFG_A>;
impl MATCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MATCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MATCFG_A::_000),
            2 => Val(MATCFG_A::_010),
            3 => Val(MATCFG_A::_011),
            4 => Val(MATCFG_A::_100),
            5 => Val(MATCFG_A::_101),
            6 => Val(MATCFG_A::_110),
            7 => Val(MATCFG_A::_111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MATCFG_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MATCFG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MATCFG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MATCFG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MATCFG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MATCFG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MATCFG_A::_111
    }
}
#[doc = "Write proxy for field `MATCFG`"]
pub struct MATCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Match disabled."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MATCFG_A::_000)
    }
    #[doc = "Match enabled (1st data word equals MATCH0 OR MATCH1)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MATCFG_A::_010)
    }
    #[doc = "Match enabled (any data word equals MATCH0 OR MATCH1)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MATCFG_A::_011)
    }
    #[doc = "Match enabled (1st data word equals MATCH0 AND 2nd data word equals MATCH1)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MATCFG_A::_100)
    }
    #[doc = "Match enabled (any data word equals MATCH0 AND next data word equals MATCH1)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MATCFG_A::_101)
    }
    #[doc = "Match enabled (1st data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MATCFG_A::_110)
    }
    #[doc = "Match enabled (any data word AND MATCH1 equals MATCH0 AND MATCH1)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MATCFG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: LPI2C configured for 2-pin open drain mode."]
    _000 = 0,
    #[doc = "1: LPI2C configured for 2-pin output only mode (ultra-fast mode)."]
    _001 = 1,
    #[doc = "2: LPI2C configured for 2-pin push-pull mode."]
    _010 = 2,
    #[doc = "3: LPI2C configured for 4-pin push-pull mode."]
    _011 = 3,
    #[doc = "4: LPI2C configured for 2-pin open drain mode with separate LPI2C slave."]
    _100 = 4,
    #[doc = "5: LPI2C configured for 2-pin output only mode (ultra-fast mode) with separate LPI2C slave."]
    _101 = 5,
    #[doc = "6: LPI2C configured for 2-pin push-pull mode with separate LPI2C slave."]
    _110 = 6,
    #[doc = "7: LPI2C configured for 4-pin push-pull mode (inverted outputs)."]
    _111 = 7,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINCFG`"]
pub type PINCFG_R = crate::R<u8, PINCFG_A>;
impl PINCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::_000,
            1 => PINCFG_A::_001,
            2 => PINCFG_A::_010,
            3 => PINCFG_A::_011,
            4 => PINCFG_A::_100,
            5 => PINCFG_A::_101,
            6 => PINCFG_A::_110,
            7 => PINCFG_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PINCFG_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PINCFG_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PINCFG_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PINCFG_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PINCFG_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PINCFG_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PINCFG_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PINCFG_A::_111
    }
}
#[doc = "Write proxy for field `PINCFG`"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPI2C configured for 2-pin open drain mode."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PINCFG_A::_000)
    }
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PINCFG_A::_001)
    }
    #[doc = "LPI2C configured for 2-pin push-pull mode."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PINCFG_A::_010)
    }
    #[doc = "LPI2C configured for 4-pin push-pull mode."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PINCFG_A::_011)
    }
    #[doc = "LPI2C configured for 2-pin open drain mode with separate LPI2C slave."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PINCFG_A::_100)
    }
    #[doc = "LPI2C configured for 2-pin output only mode (ultra-fast mode) with separate LPI2C slave."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PINCFG_A::_101)
    }
    #[doc = "LPI2C configured for 2-pin push-pull mode with separate LPI2C slave."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PINCFG_A::_110)
    }
    #[doc = "LPI2C configured for 4-pin push-pull mode (inverted outputs)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PINCFG_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    pub fn autostop(&self) -> AUTOSTOP_R {
        AUTOSTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    pub fn timecfg(&self) -> TIMECFG_R {
        TIMECFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MATCFG_R {
        MATCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 8 - Automatic STOP Generation"]
    #[inline(always)]
    pub fn autostop(&mut self) -> AUTOSTOP_W {
        AUTOSTOP_W { w: self }
    }
    #[doc = "Bit 9 - IGNACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Bit 10 - Timeout Configuration"]
    #[inline(always)]
    pub fn timecfg(&mut self) -> TIMECFG_W {
        TIMECFG_W { w: self }
    }
    #[doc = "Bits 16:18 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&mut self) -> MATCFG_W {
        MATCFG_W { w: self }
    }
    #[doc = "Bits 24:26 - Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
}
