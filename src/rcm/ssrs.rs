#[doc = "Reader of register SSRS"]
pub type R = crate::R<u32, super::SSRS>;
#[doc = "Writer for register SSRS"]
pub type W = crate::W<u32, super::SSRS>;
#[doc = "Register SSRS `reset()`'s with value 0x82"]
impl crate::ResetValue for super::SSRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x82
    }
}
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<SLVD_A> for bool {
    #[inline(always)]
    fn from(variant: SLVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLVD`"]
pub type SLVD_R = crate::R<bool, SLVD_A>;
impl SLVD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVD_A {
        match self.bits {
            false => SLVD_A::_0,
            true => SLVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLVD_A::_1
    }
}
#[doc = "Write proxy for field `SLVD`"]
pub struct SLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVD_A::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVD_A::_1)
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
#[doc = "Sticky Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1 = 1,
}
impl From<SLOC_A> for bool {
    #[inline(always)]
    fn from(variant: SLOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOC`"]
pub type SLOC_R = crate::R<bool, SLOC_A>;
impl SLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOC_A {
        match self.bits {
            false => SLOC_A::_0,
            true => SLOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOC_A::_1
    }
}
#[doc = "Write proxy for field `SLOC`"]
pub struct SLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOC_A::_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOC_A::_1)
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
#[doc = "Sticky Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL/FLL"]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL/FLL"]
    _1 = 1,
}
impl From<SLOL_A> for bool {
    #[inline(always)]
    fn from(variant: SLOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOL`"]
pub type SLOL_R = crate::R<bool, SLOL_A>;
impl SLOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOL_A {
        match self.bits {
            false => SLOL_A::_0,
            true => SLOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOL_A::_1
    }
}
#[doc = "Write proxy for field `SLOL`"]
pub struct SLOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOL_A::_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOL_A::_1)
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
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<SWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: SWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWDOG`"]
pub type SWDOG_R = crate::R<bool, SWDOG_A>;
impl SWDOG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDOG_A {
        match self.bits {
            false => SWDOG_A::_0,
            true => SWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWDOG_A::_1
    }
}
#[doc = "Write proxy for field `SWDOG`"]
pub struct SWDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDOG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOG_A::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOG_A::_1)
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
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<SPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIN`"]
pub type SPIN_R = crate::R<bool, SPIN_A>;
impl SPIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_A {
        match self.bits {
            false => SPIN_A::_0,
            true => SPIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIN_A::_1
    }
}
#[doc = "Write proxy for field `SPIN`"]
pub struct SPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIN_A::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIN_A::_1)
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
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR_A {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<SPOR_A> for bool {
    #[inline(always)]
    fn from(variant: SPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPOR`"]
pub type SPOR_R = crate::R<bool, SPOR_A>;
impl SPOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOR_A {
        match self.bits {
            false => SPOR_A::_0,
            true => SPOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOR_A::_1
    }
}
#[doc = "Write proxy for field `SPOR`"]
pub struct SPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOR_A::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOR_A::_1)
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
#[doc = "Sticky JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SJTAG_A {
    #[doc = "0: Reset not caused by JTAG"]
    _0 = 0,
    #[doc = "1: Reset caused by JTAG"]
    _1 = 1,
}
impl From<SJTAG_A> for bool {
    #[inline(always)]
    fn from(variant: SJTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SJTAG`"]
pub type SJTAG_R = crate::R<bool, SJTAG_A>;
impl SJTAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJTAG_A {
        match self.bits {
            false => SJTAG_A::_0,
            true => SJTAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SJTAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SJTAG_A::_1
    }
}
#[doc = "Write proxy for field `SJTAG`"]
pub struct SJTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SJTAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SJTAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SJTAG_A::_0)
    }
    #[doc = "Reset caused by JTAG"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SJTAG_A::_1)
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
#[doc = "Sticky Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    _0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    _1 = 1,
}
impl From<SLOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: SLOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLOCKUP`"]
pub type SLOCKUP_R = crate::R<bool, SLOCKUP_A>;
impl SLOCKUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOCKUP_A {
        match self.bits {
            false => SLOCKUP_A::_0,
            true => SLOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOCKUP_A::_1
    }
}
#[doc = "Write proxy for field `SLOCKUP`"]
pub struct SLOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOCKUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCKUP_A::_1)
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
#[doc = "Sticky Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    _0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1 = 1,
}
impl From<SSW_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSW`"]
pub type SSW_R = crate::R<bool, SSW_A>;
impl SSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_A {
        match self.bits {
            false => SSW_A::_0,
            true => SSW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSW_A::_1
    }
}
#[doc = "Write proxy for field `SSW`"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSW_A::_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSW_A::_1)
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
#[doc = "Sticky MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_AP_A {
    #[doc = "0: Reset was not caused by host debugger system setting of the System Reset Request bit"]
    _0 = 0,
    #[doc = "1: Reset was caused by host debugger system setting of the System Reset Request bit"]
    _1 = 1,
}
impl From<SMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: SMDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMDM_AP`"]
pub type SMDM_AP_R = crate::R<bool, SMDM_AP_A>;
impl SMDM_AP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMDM_AP_A {
        match self.bits {
            false => SMDM_AP_A::_0,
            true => SMDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SMDM_AP_A::_1
    }
}
#[doc = "Write proxy for field `SMDM_AP`"]
pub struct SMDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMDM_AP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_0)
    }
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMDM_AP_A::_1)
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
#[doc = "Sticky Stop Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1 = 1,
}
impl From<SSACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SSACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSACKERR`"]
pub type SSACKERR_R = crate::R<bool, SSACKERR_A>;
impl SSACKERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACKERR_A {
        match self.bits {
            false => SSACKERR_A::_0,
            true => SSACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SSACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SSACKERR_A::_1
    }
}
#[doc = "Write proxy for field `SSACKERR`"]
pub struct SSACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACKERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACKERR_A::_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACKERR_A::_1)
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
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SLVD_R {
        SLVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&self) -> SLOC_R {
        SLOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&self) -> SLOL_R {
        SLOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SWDOG_R {
        SWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SPIN_R {
        SPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SPOR_R {
        SPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline(always)]
    pub fn sjtag(&self) -> SJTAG_R {
        SJTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&self) -> SLOCKUP_R {
        SLOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&self) -> SMDM_AP_R {
        SMDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline(always)]
    pub fn ssackerr(&self) -> SSACKERR_R {
        SSACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&mut self) -> SLVD_W {
        SLVD_W { w: self }
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&mut self) -> SLOC_W {
        SLOC_W { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&mut self) -> SLOL_W {
        SLOL_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&mut self) -> SWDOG_W {
        SWDOG_W { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&mut self) -> SPIN_W {
        SPIN_W { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&mut self) -> SPOR_W {
        SPOR_W { w: self }
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline(always)]
    pub fn sjtag(&mut self) -> SJTAG_W {
        SJTAG_W { w: self }
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&mut self) -> SLOCKUP_W {
        SLOCKUP_W { w: self }
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&mut self) -> SMDM_AP_W {
        SMDM_AP_W { w: self }
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline(always)]
    pub fn ssackerr(&mut self) -> SSACKERR_W {
        SSACKERR_W { w: self }
    }
}
