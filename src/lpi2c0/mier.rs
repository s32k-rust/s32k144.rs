#[doc = "Reader of register MIER"]
pub type R = crate::R<u32, super::MIER>;
#[doc = "Writer for register MIER"]
pub type W = crate::W<u32, super::MIER>;
#[doc = "Register MIER `reset()`'s with value 0"]
impl crate::ResetValue for super::MIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled"]
    _1 = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDIE`"]
pub type TDIE_R = crate::R<bool, TDIE_A>;
impl TDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::_0,
            true => TDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDIE_A::_1
    }
}
#[doc = "Write proxy for field `TDIE`"]
pub struct TDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDIE_A::_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDIE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDIE`"]
pub type RDIE_R = crate::R<bool, RDIE_A>;
impl RDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::_0,
            true => RDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDIE_A::_1
    }
}
#[doc = "Write proxy for field `RDIE`"]
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "End Packet Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPIE`"]
pub type EPIE_R = crate::R<bool, EPIE_A>;
impl EPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::_0,
            true => EPIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPIE_A::_1
    }
}
#[doc = "Write proxy for field `EPIE`"]
pub struct EPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPIE_A::_1)
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
#[doc = "STOP Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<SDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDIE`"]
pub type SDIE_R = crate::R<bool, SDIE_A>;
impl SDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIE_A {
        match self.bits {
            false => SDIE_A::_0,
            true => SDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDIE_A::_1
    }
}
#[doc = "Write proxy for field `SDIE`"]
pub struct SDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDIE_A::_1)
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
#[doc = "NACK Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<NDIE_A> for bool {
    #[inline(always)]
    fn from(variant: NDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NDIE`"]
pub type NDIE_R = crate::R<bool, NDIE_A>;
impl NDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDIE_A {
        match self.bits {
            false => NDIE_A::_0,
            true => NDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NDIE_A::_1
    }
}
#[doc = "Write proxy for field `NDIE`"]
pub struct NDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NDIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NDIE_A::_1)
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
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALIE`"]
pub type ALIE_R = crate::R<bool, ALIE_A>;
impl ALIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::_0,
            true => ALIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALIE_A::_1
    }
}
#[doc = "Write proxy for field `ALIE`"]
pub struct ALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Interrupt enabled."]
    _0 = 0,
    #[doc = "1: Interrupt disabled."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FEIE`"]
pub type FEIE_R = crate::R<bool, FEIE_A>;
impl FEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Write proxy for field `FEIE`"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Pin Low Timeout Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<PLTIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLTIE`"]
pub type PLTIE_R = crate::R<bool, PLTIE_A>;
impl PLTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTIE_A {
        match self.bits {
            false => PLTIE_A::_0,
            true => PLTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLTIE_A::_1
    }
}
#[doc = "Write proxy for field `PLTIE`"]
pub struct PLTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLTIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLTIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Data Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIE_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<DMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMIE`"]
pub type DMIE_R = crate::R<bool, DMIE_A>;
impl DMIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIE_A {
        match self.bits {
            false => DMIE_A::_0,
            true => DMIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMIE_A::_1
    }
}
#[doc = "Write proxy for field `DMIE`"]
pub struct DMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMIE_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&self) -> SDIE_R {
        SDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ndie(&self) -> NDIE_R {
        NDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn pltie(&self) -> PLTIE_R {
        PLTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&self) -> DMIE_R {
        DMIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&mut self) -> TDIE_W {
        TDIE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W {
        EPIE_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&mut self) -> SDIE_W {
        SDIE_W { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ndie(&mut self) -> NDIE_W {
        NDIE_W { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W {
        ALIE_W { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn pltie(&mut self) -> PLTIE_W {
        PLTIE_W { w: self }
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&mut self) -> DMIE_W {
        DMIE_W { w: self }
    }
}
