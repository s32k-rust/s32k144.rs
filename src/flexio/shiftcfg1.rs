#[doc = "Reader of register SHIFTCFG1"]
pub type R = crate::R<u32, super::SHIFTCFG1>;
#[doc = "Writer for register SHIFTCFG1"]
pub type W = crate::W<u32, super::SHIFTCFG1>;
#[doc = "Register SHIFTCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSTART_A {
    #[doc = "0: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    _0 = 0,
    #[doc = "1: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    _1 = 1,
    #[doc = "2: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    _10 = 2,
    #[doc = "3: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    _11 = 3,
}
impl From<SSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTART_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSTART`"]
pub type SSTART_R = crate::R<u8, SSTART_A>;
impl SSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTART_A {
        match self.bits {
            0 => SSTART_A::_0,
            1 => SSTART_A::_1,
            2 => SSTART_A::_10,
            3 => SSTART_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSTART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSTART_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSTART_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SSTART_A::_11
    }
}
#[doc = "Write proxy for field `SSTART`"]
pub struct SSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTART_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTART_A::_0)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTART_A::_1)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTART_A::_10)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTART_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Shifter Stop bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSTOP_A {
    #[doc = "0: Stop bit disabled for transmitter/receiver/match store"]
    _0 = 0,
    #[doc = "1: Reserved for transmitter/receiver/match store"]
    _1 = 1,
    #[doc = "2: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    _10 = 2,
    #[doc = "3: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    _11 = 3,
}
impl From<SSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSTOP`"]
pub type SSTOP_R = crate::R<u8, SSTOP_A>;
impl SSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTOP_A {
        match self.bits {
            0 => SSTOP_A::_0,
            1 => SSTOP_A::_1,
            2 => SSTOP_A::_10,
            3 => SSTOP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSTOP_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SSTOP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SSTOP_A::_11
    }
}
#[doc = "Write proxy for field `SSTOP`"]
pub struct SSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTOP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTOP_A::_0)
    }
    #[doc = "Reserved for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTOP_A::_1)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTOP_A::_10)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTOP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSRC_A {
    #[doc = "0: Pin"]
    _0 = 0,
    #[doc = "1: Shifter N+1 Output"]
    _1 = 1,
}
impl From<INSRC_A> for bool {
    #[inline(always)]
    fn from(variant: INSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INSRC`"]
pub type INSRC_R = crate::R<bool, INSRC_A>;
impl INSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSRC_A {
        match self.bits {
            false => INSRC_A::_0,
            true => INSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INSRC_A::_1
    }
}
#[doc = "Write proxy for field `INSRC`"]
pub struct INSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> INSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INSRC_A::_0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INSRC_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&self) -> INSRC_R {
        INSRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&mut self) -> SSTART_W {
        SSTART_W { w: self }
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SSTOP_W {
        SSTOP_W { w: self }
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&mut self) -> INSRC_W {
        INSRC_W { w: self }
    }
}
