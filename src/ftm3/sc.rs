#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u32, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 4"]
    _010 = 2,
    #[doc = "3: Divide by 8"]
    _011 = 3,
    #[doc = "4: Divide by 16"]
    _100 = 4,
    #[doc = "5: Divide by 32"]
    _101 = 5,
    #[doc = "6: Divide by 64"]
    _110 = 6,
    #[doc = "7: Divide by 128"]
    _111 = 7,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PS_A::_111
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: No clock selected. This in effect disables the FTM counter."]
    _00 = 0,
    #[doc = "1: FTM input clock"]
    _01 = 1,
    #[doc = "2: Fixed frequency clock"]
    _10 = 2,
    #[doc = "3: External clock"]
    _11 = 3,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKS`"]
pub type CLKS_R = crate::R<u8, CLKS_A>;
impl CLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::_00,
            1 => CLKS_A::_01,
            2 => CLKS_A::_10,
            3 => CLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKS_A::_11
    }
}
#[doc = "Write proxy for field `CLKS`"]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKS_A::_00)
    }
    #[doc = "FTM input clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKS_A::_01)
    }
    #[doc = "Fixed frequency clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKS_A::_10)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMS_A {
    #[doc = "0: FTM counter operates in Up Counting mode."]
    _0 = 0,
    #[doc = "1: FTM counter operates in Up-Down Counting mode."]
    _1 = 1,
}
impl From<CPWMS_A> for bool {
    #[inline(always)]
    fn from(variant: CPWMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPWMS`"]
pub type CPWMS_R = crate::R<bool, CPWMS_A>;
impl CPWMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPWMS_A {
        match self.bits {
            false => CPWMS_A::_0,
            true => CPWMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPWMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPWMS_A::_1
    }
}
#[doc = "Write proxy for field `CPWMS`"]
pub struct CPWMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPWMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPWMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FTM counter operates in Up Counting mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMS_A::_0)
    }
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMS_A::_1)
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
#[doc = "Reload Point Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: Reload point interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Reload point interrupt is enabled."]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIE`"]
pub type RIE_R = crate::R<bool, RIE_A>;
impl RIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Write proxy for field `RIE`"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reload point interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "Reload point interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
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
#[doc = "Reload Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_A {
    #[doc = "0: A selected reload point did not happen."]
    _0 = 0,
    #[doc = "1: A selected reload point happened."]
    _1 = 1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF`"]
pub type RF_R = crate::R<bool, RF_A>;
impl RF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_A {
        match self.bits {
            false => RF_A::_0,
            true => RF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RF_A::_1
    }
}
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
    #[doc = "0: Disable TOF interrupts. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOIE`"]
pub type TOIE_R = crate::R<bool, TOIE_A>;
impl TOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::_0,
            true => TOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIE_A::_1
    }
}
#[doc = "Write proxy for field `TOIE`"]
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIE_A::_1)
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
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: FTM counter has not overflowed."]
    _0 = 0,
    #[doc = "1: FTM counter has overflowed."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOF`"]
pub type TOF_R = crate::R<bool, TOF_A>;
impl TOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOF_A::_1
    }
}
#[doc = "Channel 0 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN0`"]
pub type PWMEN0_R = crate::R<bool, PWMEN0_A>;
impl PWMEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN0_A {
        match self.bits {
            false => PWMEN0_A::_0,
            true => PWMEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN0_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN0`"]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN0_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Channel 1 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN1`"]
pub type PWMEN1_R = crate::R<bool, PWMEN1_A>;
impl PWMEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN1_A {
        match self.bits {
            false => PWMEN1_A::_0,
            true => PWMEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN1_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN1`"]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN1_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Channel 2 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN2`"]
pub type PWMEN2_R = crate::R<bool, PWMEN2_A>;
impl PWMEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN2_A {
        match self.bits {
            false => PWMEN2_A::_0,
            true => PWMEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN2_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN2`"]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN2_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Channel 3 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN3`"]
pub type PWMEN3_R = crate::R<bool, PWMEN3_A>;
impl PWMEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN3_A {
        match self.bits {
            false => PWMEN3_A::_0,
            true => PWMEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN3_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN3`"]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN3_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Channel 4 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN4_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN4`"]
pub type PWMEN4_R = crate::R<bool, PWMEN4_A>;
impl PWMEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN4_A {
        match self.bits {
            false => PWMEN4_A::_0,
            true => PWMEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN4_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN4`"]
pub struct PWMEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN4_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Channel 5 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN5_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN5`"]
pub type PWMEN5_R = crate::R<bool, PWMEN5_A>;
impl PWMEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN5_A {
        match self.bits {
            false => PWMEN5_A::_0,
            true => PWMEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN5_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN5`"]
pub struct PWMEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN5_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Channel 6 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN6_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN6`"]
pub type PWMEN6_R = crate::R<bool, PWMEN6_A>;
impl PWMEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN6_A {
        match self.bits {
            false => PWMEN6_A::_0,
            true => PWMEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN6_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN6`"]
pub struct PWMEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN6_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Channel 7 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN7_A {
    #[doc = "0: Channel output port is disabled"]
    _0 = 0,
    #[doc = "1: Channel output port is enabled"]
    _1 = 1,
}
impl From<PWMEN7_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMEN7`"]
pub type PWMEN7_R = crate::R<bool, PWMEN7_A>;
impl PWMEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN7_A {
        match self.bits {
            false => PWMEN7_A::_0,
            true => PWMEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMEN7_A::_1
    }
}
#[doc = "Write proxy for field `PWMEN7`"]
pub struct PWMEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel output port is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMEN7_A::_0)
    }
    #[doc = "Channel output port is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMEN7_A::_1)
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
#[doc = "Filter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTPS_A {
    #[doc = "0: Divide by 1"]
    _0000 = 0,
    #[doc = "1: Divide by 2"]
    _0001 = 1,
    #[doc = "2: Divide by 3"]
    _0010 = 2,
    #[doc = "3: Divide by 4"]
    _0011 = 3,
    #[doc = "4: Divide by 5"]
    _0100 = 4,
    #[doc = "5: Divide by 6"]
    _0101 = 5,
    #[doc = "6: Divide by 7"]
    _0110 = 6,
    #[doc = "7: Divide by 8"]
    _0111 = 7,
    #[doc = "8: Divide by 9"]
    _1000 = 8,
    #[doc = "9: Divide by 10"]
    _1001 = 9,
    #[doc = "10: Divide by 11"]
    _1010 = 10,
    #[doc = "11: Divide by 12"]
    _1011 = 11,
    #[doc = "12: Divide by 13"]
    _1100 = 12,
    #[doc = "13: Divide by 14"]
    _1101 = 13,
    #[doc = "14: Divide by 15"]
    _1110 = 14,
    #[doc = "15: Divide by 16"]
    _1111 = 15,
}
impl From<FLTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTPS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLTPS`"]
pub type FLTPS_R = crate::R<u8, FLTPS_A>;
impl FLTPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPS_A {
        match self.bits {
            0 => FLTPS_A::_0000,
            1 => FLTPS_A::_0001,
            2 => FLTPS_A::_0010,
            3 => FLTPS_A::_0011,
            4 => FLTPS_A::_0100,
            5 => FLTPS_A::_0101,
            6 => FLTPS_A::_0110,
            7 => FLTPS_A::_0111,
            8 => FLTPS_A::_1000,
            9 => FLTPS_A::_1001,
            10 => FLTPS_A::_1010,
            11 => FLTPS_A::_1011,
            12 => FLTPS_A::_1100,
            13 => FLTPS_A::_1101,
            14 => FLTPS_A::_1110,
            15 => FLTPS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FLTPS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FLTPS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == FLTPS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FLTPS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FLTPS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == FLTPS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == FLTPS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == FLTPS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == FLTPS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == FLTPS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == FLTPS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == FLTPS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == FLTPS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == FLTPS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == FLTPS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == FLTPS_A::_1111
    }
}
#[doc = "Write proxy for field `FLTPS`"]
pub struct FLTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTPS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FLTPS_A::_0000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(FLTPS_A::_0001)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(FLTPS_A::_0010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(FLTPS_A::_0011)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(FLTPS_A::_0100)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(FLTPS_A::_0101)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(FLTPS_A::_0110)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(FLTPS_A::_0111)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(FLTPS_A::_1000)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(FLTPS_A::_1001)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(FLTPS_A::_1010)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(FLTPS_A::_1011)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(FLTPS_A::_1100)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(FLTPS_A::_1101)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(FLTPS_A::_1110)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(FLTPS_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CPWMS_R {
        CPWMS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen4(&self) -> PWMEN4_R {
        PWMEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen5(&self) -> PWMEN5_R {
        PWMEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen6(&self) -> PWMEN6_R {
        PWMEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen7(&self) -> PWMEN7_R {
        PWMEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline(always)]
    pub fn fltps(&self) -> FLTPS_R {
        FLTPS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&mut self) -> CPWMS_W {
        CPWMS_W { w: self }
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> PWMEN0_W {
        PWMEN0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> PWMEN1_W {
        PWMEN1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> PWMEN2_W {
        PWMEN2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen4(&mut self) -> PWMEN4_W {
        PWMEN4_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen5(&mut self) -> PWMEN5_W {
        PWMEN5_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen6(&mut self) -> PWMEN6_W {
        PWMEN6_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen7(&mut self) -> PWMEN7_W {
        PWMEN7_W { w: self }
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline(always)]
    pub fn fltps(&mut self) -> FLTPS_W {
        FLTPS_W { w: self }
    }
}
