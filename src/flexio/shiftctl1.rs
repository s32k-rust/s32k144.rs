#[doc = "Reader of register SHIFTCTL1"]
pub type R = crate::R<u32, super::SHIFTCTL1>;
#[doc = "Writer for register SHIFTCTL1"]
pub type W = crate::W<u32, super::SHIFTCTL1>;
#[doc = "Register SHIFTCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shifter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    _1 = 1,
    #[doc = "2: Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    _10 = 2,
    #[doc = "4: Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    _100 = 4,
    #[doc = "5: Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    _101 = 5,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD`"]
pub type SMOD_R = crate::R<u8, SMOD_A>;
impl SMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD_A::_0),
            1 => Val(SMOD_A::_1),
            2 => Val(SMOD_A::_10),
            4 => Val(SMOD_A::_100),
            5 => Val(SMOD_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMOD_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SMOD_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SMOD_A::_101
    }
}
#[doc = "Write proxy for field `SMOD`"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMOD_A::_0)
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMOD_A::_1)
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMOD_A::_10)
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SMOD_A::_100)
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SMOD_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Shifter Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOL_A {
    #[doc = "0: Pin is active high"]
    _0 = 0,
    #[doc = "1: Pin is active low"]
    _1 = 1,
}
impl From<PINPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PINPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PINPOL`"]
pub type PINPOL_R = crate::R<bool, PINPOL_A>;
impl PINPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPOL_A {
        match self.bits {
            false => PINPOL_A::_0,
            true => PINPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PINPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PINPOL_A::_1
    }
}
#[doc = "Write proxy for field `PINPOL`"]
pub struct PINPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINPOL_A::_0)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PINSEL`"]
pub type PINSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PINSEL`"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Shifter Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Shifter pin output disabled"]
    _0 = 0,
    #[doc = "1: Shifter pin open drain or bidirectional output enable"]
    _1 = 1,
    #[doc = "2: Shifter pin bidirectional output data"]
    _10 = 2,
    #[doc = "3: Shifter pin output"]
    _11 = 3,
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
            0 => PINCFG_A::_0,
            1 => PINCFG_A::_1,
            2 => PINCFG_A::_10,
            3 => PINCFG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PINCFG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PINCFG_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PINCFG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PINCFG_A::_11
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
    #[doc = "Shifter pin output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINCFG_A::_0)
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINCFG_A::_1)
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINCFG_A::_10)
    }
    #[doc = "Shifter pin output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PINCFG_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Timer Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPOL_A {
    #[doc = "0: Shift on posedge of Shift clock"]
    _0 = 0,
    #[doc = "1: Shift on negedge of Shift clock"]
    _1 = 1,
}
impl From<TIMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMPOL`"]
pub type TIMPOL_R = crate::R<bool, TIMPOL_A>;
impl TIMPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPOL_A {
        match self.bits {
            false => TIMPOL_A::_0,
            true => TIMPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMPOL_A::_1
    }
}
#[doc = "Write proxy for field `TIMPOL`"]
pub struct TIMPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shift on posedge of Shift clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMPOL_A::_0)
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TIMSEL`"]
pub type TIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMSEL`"]
pub struct TIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&self) -> TIMPOL_R {
        TIMPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&self) -> TIMSEL_R {
        TIMSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&mut self) -> PINPOL_W {
        PINPOL_W { w: self }
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&mut self) -> TIMPOL_W {
        TIMPOL_W { w: self }
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&mut self) -> TIMSEL_W {
        TIMSEL_W { w: self }
    }
}
