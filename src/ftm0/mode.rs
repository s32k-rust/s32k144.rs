#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0x04"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "FTM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTMEN_A {
    #[doc = "0: TPM compatibility. Free running counter and synchronization compatible with TPM."]
    _0 = 0,
    #[doc = "1: Free running counter and synchronization are different from TPM behavior."]
    _1 = 1,
}
impl From<FTMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FTMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTMEN`"]
pub type FTMEN_R = crate::R<bool, FTMEN_A>;
impl FTMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTMEN_A {
        match self.bits {
            false => FTMEN_A::_0,
            true => FTMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTMEN_A::_1
    }
}
#[doc = "Write proxy for field `FTMEN`"]
pub struct FTMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FTMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TPM compatibility. Free running counter and synchronization compatible with TPM."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMEN_A::_0)
    }
    #[doc = "Free running counter and synchronization are different from TPM behavior."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMEN_A::_1)
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
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
#[doc = "Write Protection Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPDIS_A {
    #[doc = "0: Write protection is enabled."]
    _0 = 0,
    #[doc = "1: Write protection is disabled."]
    _1 = 1,
}
impl From<WPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: WPDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPDIS`"]
pub type WPDIS_R = crate::R<bool, WPDIS_A>;
impl WPDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPDIS_A {
        match self.bits {
            false => WPDIS_A::_0,
            true => WPDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPDIS_A::_1
    }
}
#[doc = "Write proxy for field `WPDIS`"]
pub struct WPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WPDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPDIS_A::_0)
    }
    #[doc = "Write protection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPDIS_A::_1)
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
#[doc = "PWM Synchronization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSYNC_A {
    #[doc = "0: No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    _1 = 1,
}
impl From<PWMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMSYNC`"]
pub type PWMSYNC_R = crate::R<bool, PWMSYNC_A>;
impl PWMSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSYNC_A {
        match self.bits {
            false => PWMSYNC_A::_0,
            true => PWMSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMSYNC_A::_1
    }
}
#[doc = "Write proxy for field `PWMSYNC`"]
pub struct PWMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMSYNC_A::_0)
    }
    #[doc = "Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMSYNC_A::_1)
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
#[doc = "Capture Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPTEST_A {
    #[doc = "0: Capture test mode is disabled."]
    _0 = 0,
    #[doc = "1: Capture test mode is enabled."]
    _1 = 1,
}
impl From<CAPTEST_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPTEST`"]
pub type CAPTEST_R = crate::R<bool, CAPTEST_A>;
impl CAPTEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTEST_A {
        match self.bits {
            false => CAPTEST_A::_0,
            true => CAPTEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CAPTEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CAPTEST_A::_1
    }
}
#[doc = "Write proxy for field `CAPTEST`"]
pub struct CAPTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture test mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPTEST_A::_0)
    }
    #[doc = "Capture test mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPTEST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Fault Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAULTM_A {
    #[doc = "0: Fault control is disabled for all channels."]
    _00 = 0,
    #[doc = "1: Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    _01 = 1,
    #[doc = "2: Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    _10 = 2,
    #[doc = "3: Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    _11 = 3,
}
impl From<FAULTM_A> for u8 {
    #[inline(always)]
    fn from(variant: FAULTM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAULTM`"]
pub type FAULTM_R = crate::R<u8, FAULTM_A>;
impl FAULTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTM_A {
        match self.bits {
            0 => FAULTM_A::_00,
            1 => FAULTM_A::_01,
            2 => FAULTM_A::_10,
            3 => FAULTM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FAULTM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FAULTM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FAULTM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FAULTM_A::_11
    }
}
#[doc = "Write proxy for field `FAULTM`"]
pub struct FAULTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Fault control is disabled for all channels."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FAULTM_A::_00)
    }
    #[doc = "Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FAULTM_A::_01)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FAULTM_A::_10)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FAULTM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Fault Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTIE_A {
    #[doc = "0: Fault control interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Fault control interrupt is enabled."]
    _1 = 1,
}
impl From<FAULTIE_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTIE`"]
pub type FAULTIE_R = crate::R<bool, FAULTIE_A>;
impl FAULTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTIE_A {
        match self.bits {
            false => FAULTIE_A::_0,
            true => FAULTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTIE_A::_1
    }
}
#[doc = "Write proxy for field `FAULTIE`"]
pub struct FAULTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fault control interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTIE_A::_0)
    }
    #[doc = "Fault control interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTIE_A::_1)
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
impl R {
    #[doc = "Bit 0 - FTM Enable"]
    #[inline(always)]
    pub fn ftmen(&self) -> FTMEN_R {
        FTMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline(always)]
    pub fn wpdis(&self) -> WPDIS_R {
        WPDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline(always)]
    pub fn pwmsync(&self) -> PWMSYNC_R {
        PWMSYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline(always)]
    pub fn captest(&self) -> CAPTEST_R {
        CAPTEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline(always)]
    pub fn faultm(&self) -> FAULTM_R {
        FAULTM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline(always)]
    pub fn faultie(&self) -> FAULTIE_R {
        FAULTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM Enable"]
    #[inline(always)]
    pub fn ftmen(&mut self) -> FTMEN_W {
        FTMEN_W { w: self }
    }
    #[doc = "Bit 1 - Initialize The Channels Output"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline(always)]
    pub fn wpdis(&mut self) -> WPDIS_W {
        WPDIS_W { w: self }
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline(always)]
    pub fn pwmsync(&mut self) -> PWMSYNC_W {
        PWMSYNC_W { w: self }
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline(always)]
    pub fn captest(&mut self) -> CAPTEST_W {
        CAPTEST_W { w: self }
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline(always)]
    pub fn faultm(&mut self) -> FAULTM_W {
        FAULTM_W { w: self }
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline(always)]
    pub fn faultie(&mut self) -> FAULTIE_W {
        FAULTIE_W { w: self }
    }
}
