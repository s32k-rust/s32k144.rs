#[doc = "Reader of register C1"]
pub type R = crate::R<u32, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u32, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VOSEL`"]
pub type VOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VOSEL`"]
pub struct VOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Minus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSEL`"]
pub type MSEL_R = crate::R<u8, MSEL_A>;
impl MSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::_000,
            1 => MSEL_A::_001,
            2 => MSEL_A::_010,
            3 => MSEL_A::_011,
            4 => MSEL_A::_100,
            5 => MSEL_A::_101,
            6 => MSEL_A::_110,
            7 => MSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MSEL_A::_111
    }
}
#[doc = "Write proxy for field `MSEL`"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Plus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    _000 = 0,
    #[doc = "1: IN1"]
    _001 = 1,
    #[doc = "2: IN2"]
    _010 = 2,
    #[doc = "3: IN3"]
    _011 = 3,
    #[doc = "4: IN4"]
    _100 = 4,
    #[doc = "5: IN5"]
    _101 = 5,
    #[doc = "6: IN6"]
    _110 = 6,
    #[doc = "7: IN7"]
    _111 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, PSEL_A>;
impl PSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::_000,
            1 => PSEL_A::_001,
            2 => PSEL_A::_010,
            3 => PSEL_A::_011,
            4 => PSEL_A::_100,
            5 => PSEL_A::_101,
            6 => PSEL_A::_110,
            7 => PSEL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PSEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PSEL_A::_111
    }
}
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSEL_A::_000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSEL_A::_001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSEL_A::_010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSEL_A::_011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSEL_A::_100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSEL_A::_101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSEL_A::_110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSEL_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSEL_A {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference Vin."]
    _0 = 0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference Vin."]
    _1 = 1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VRSEL`"]
pub type VRSEL_R = crate::R<bool, VRSEL_A>;
impl VRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::_0,
            true => VRSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VRSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VRSEL_A::_1
    }
}
#[doc = "Write proxy for field `VRSEL`"]
pub struct VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRSEL_A::_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRSEL_A::_1)
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
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    _0 = 0,
    #[doc = "1: DAC is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACEN`"]
pub type DACEN_R = crate::R<bool, DACEN_A>;
impl DACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACEN_A::_1
    }
}
#[doc = "Write proxy for field `DACEN`"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CHN0`"]
pub type CHN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN0`"]
pub struct CHN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN0_W<'a> {
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
#[doc = "Reader of field `CHN1`"]
pub type CHN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN1`"]
pub struct CHN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN1_W<'a> {
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
#[doc = "Reader of field `CHN2`"]
pub type CHN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN2`"]
pub struct CHN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN2_W<'a> {
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
#[doc = "Reader of field `CHN3`"]
pub type CHN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN3`"]
pub struct CHN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN3_W<'a> {
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
#[doc = "Reader of field `CHN4`"]
pub type CHN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN4`"]
pub struct CHN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN4_W<'a> {
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
#[doc = "Reader of field `CHN5`"]
pub type CHN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN5`"]
pub struct CHN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN5_W<'a> {
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
#[doc = "Reader of field `CHN6`"]
pub type CHN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN6`"]
pub struct CHN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN6_W<'a> {
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
#[doc = "Reader of field `CHN7`"]
pub type CHN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHN7`"]
pub struct CHN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN7_W<'a> {
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
#[doc = "Selection of the input to the negative port of the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INNSEL_A {
    #[doc = "0: IN0, from the 8-bit DAC output"]
    _00 = 0,
    #[doc = "1: IN1, from the analog 8-1 mux"]
    _01 = 1,
}
impl From<INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INNSEL`"]
pub type INNSEL_R = crate::R<u8, INNSEL_A>;
impl INNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INNSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INNSEL_A::_00),
            1 => Val(INNSEL_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == INNSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == INNSEL_A::_01
    }
}
#[doc = "Write proxy for field `INNSEL`"]
pub struct INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(INNSEL_A::_00)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(INNSEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Selection of the input to the positive port of the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: IN0, from the 8-bit DAC output"]
    _00 = 0,
    #[doc = "1: IN1, from the analog 8-1 mux"]
    _01 = 1,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPSEL`"]
pub type INPSEL_R = crate::R<u8, INPSEL_A>;
impl INPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPSEL_A::_00),
            1 => Val(INPSEL_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == INPSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == INPSEL_A::_01
    }
}
#[doc = "Write proxy for field `INPSEL`"]
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(INPSEL_A::_00)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(INPSEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&self) -> CHN0_R {
        CHN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&self) -> CHN1_R {
        CHN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&self) -> CHN2_R {
        CHN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&self) -> CHN3_R {
        CHN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&self) -> CHN4_R {
        CHN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&self) -> CHN5_R {
        CHN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline(always)]
    pub fn chn6(&self) -> CHN6_R {
        CHN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline(always)]
    pub fn chn7(&self) -> CHN7_R {
        CHN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline(always)]
    pub fn innsel(&self) -> INNSEL_R {
        INNSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&mut self) -> VOSEL_W {
        VOSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VRSEL_W {
        VRSEL_W { w: self }
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&mut self) -> CHN0_W {
        CHN0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&mut self) -> CHN1_W {
        CHN1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&mut self) -> CHN2_W {
        CHN2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&mut self) -> CHN3_W {
        CHN3_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&mut self) -> CHN4_W {
        CHN4_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&mut self) -> CHN5_W {
        CHN5_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline(always)]
    pub fn chn6(&mut self) -> CHN6_W {
        CHN6_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline(always)]
    pub fn chn7(&mut self) -> CHN7_W {
        CHN7_W { w: self }
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline(always)]
    pub fn innsel(&mut self) -> INNSEL_W {
        INNSEL_W { w: self }
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
}
