#[doc = "Reader of register SRIE"]
pub type R = crate::R<u32, super::SRIE>;
#[doc = "Writer for register SRIE"]
pub type W = crate::W<u32, super::SRIE>;
#[doc = "Register SRIE `reset()`'s with value 0"]
impl crate::ResetValue for super::SRIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset Delay Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DELAY_A {
    #[doc = "0: 10 LPO cycles"]
    _00 = 0,
    #[doc = "1: 34 LPO cycles"]
    _01 = 1,
    #[doc = "2: 130 LPO cycles"]
    _10 = 2,
    #[doc = "3: 514 LPO cycles"]
    _11 = 3,
}
impl From<DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, DELAY_A>;
impl DELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELAY_A {
        match self.bits {
            0 => DELAY_A::_00,
            1 => DELAY_A::_01,
            2 => DELAY_A::_10,
            3 => DELAY_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DELAY_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DELAY_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DELAY_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DELAY_A::_11
    }
}
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "10 LPO cycles"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DELAY_A::_00)
    }
    #[doc = "34 LPO cycles"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DELAY_A::_01)
    }
    #[doc = "130 LPO cycles"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DELAY_A::_10)
    }
    #[doc = "514 LPO cycles"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DELAY_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Loss-of-Clock Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOC`"]
pub type LOC_R = crate::R<bool, LOC_A>;
impl LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::_0,
            true => LOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOC_A::_1
    }
}
#[doc = "Write proxy for field `LOC`"]
pub struct LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOC_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Loss-of-Lock Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOL_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<LOL_A> for bool {
    #[inline(always)]
    fn from(variant: LOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOL`"]
pub type LOL_R = crate::R<bool, LOL_A>;
impl LOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOL_A {
        match self.bits {
            false => LOL_A::_0,
            true => LOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOL_A::_1
    }
}
#[doc = "Write proxy for field `LOL`"]
pub struct LOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOL_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Watchdog Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDOG`"]
pub type WDOG_R = crate::R<bool, WDOG_A>;
impl WDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::_0,
            true => WDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDOG_A::_1
    }
}
#[doc = "Write proxy for field `WDOG`"]
pub struct WDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDOG_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDOG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "External Reset Pin Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<bool, PIN_A>;
impl PIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::_0,
            true => PIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIN_A::_1
    }
}
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIN_A::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Global Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIE_A {
    #[doc = "0: All interrupt sources disabled."]
    _0 = 0,
    #[doc = "1: All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    _1 = 1,
}
impl From<GIE_A> for bool {
    #[inline(always)]
    fn from(variant: GIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIE`"]
pub type GIE_R = crate::R<bool, GIE_A>;
impl GIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIE_A {
        match self.bits {
            false => GIE_A::_0,
            true => GIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GIE_A::_1
    }
}
#[doc = "Write proxy for field `GIE`"]
pub struct GIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All interrupt sources disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GIE_A::_0)
    }
    #[doc = "All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GIE_A::_1)
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
#[doc = "JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<JTAG_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JTAG`"]
pub type JTAG_R = crate::R<bool, JTAG_A>;
impl JTAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_A {
        match self.bits {
            false => JTAG_A::_0,
            true => JTAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JTAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JTAG_A::_1
    }
}
#[doc = "Write proxy for field `JTAG`"]
pub struct JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(JTAG_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(JTAG_A::_1)
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
#[doc = "Core Lockup Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKUP`"]
pub type LOCKUP_R = crate::R<bool, LOCKUP_A>;
impl LOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::_0,
            true => LOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCKUP_A::_1
    }
}
#[doc = "Write proxy for field `LOCKUP`"]
pub struct LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKUP_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKUP_A::_1)
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
#[doc = "Software Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<bool, SW_A>;
impl SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::_0,
            true => SW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SW_A::_1
    }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SW_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SW_A::_1)
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
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_AP_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<MDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: MDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDM_AP`"]
pub type MDM_AP_R = crate::R<bool, MDM_AP_A>;
impl MDM_AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDM_AP_A {
        match self.bits {
            false => MDM_AP_A::_0,
            true => MDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDM_AP_A::_1
    }
}
#[doc = "Write proxy for field `MDM_AP`"]
pub struct MDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> MDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDM_AP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDM_AP_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDM_AP_A::_1)
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
#[doc = "Stop Acknowledge Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERR_A {
    #[doc = "0: Interrupt disabled."]
    _0 = 0,
    #[doc = "1: Interrupt enabled."]
    _1 = 1,
}
impl From<SACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SACKERR`"]
pub type SACKERR_R = crate::R<bool, SACKERR_A>;
impl SACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKERR_A {
        match self.bits {
            false => SACKERR_A::_0,
            true => SACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACKERR_A::_1
    }
}
#[doc = "Write proxy for field `SACKERR`"]
pub struct SACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SACKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SACKERR_A::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SACKERR_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline(always)]
    pub fn lol(&self) -> LOL_R {
        LOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MDM_AP_R {
        MDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn sackerr(&self) -> SACKERR_R {
        SACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline(always)]
    pub fn loc(&mut self) -> LOC_W {
        LOC_W { w: self }
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline(always)]
    pub fn lol(&mut self) -> LOL_W {
        LOL_W { w: self }
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WDOG_W {
        WDOG_W { w: self }
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W {
        GIE_W { w: self }
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&mut self) -> JTAG_W {
        JTAG_W { w: self }
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W {
        LOCKUP_W { w: self }
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&mut self) -> MDM_AP_W {
        MDM_AP_W { w: self }
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn sackerr(&mut self) -> SACKERR_W {
        SACKERR_W { w: self }
    }
}
