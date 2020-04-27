#[doc = "Reader of register C2"]
pub type R = crate::R<u32, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u32, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACOn`"]
pub type ACON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACOn`"]
pub struct ACON_W<'a> {
    w: &'a mut W,
}
impl<'a> ACON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Comparator and DAC initialization delay modulus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INITMOD_A {
    #[doc = "0: The modulus is set to 64(same with 111111)."]
    _000000 = 0,
}
impl From<INITMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: INITMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INITMOD`"]
pub type INITMOD_R = crate::R<u8, INITMOD_A>;
impl INITMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INITMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INITMOD_A::_000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000000`"]
    #[inline(always)]
    pub fn is_000000(&self) -> bool {
        *self == INITMOD_A::_000000
    }
}
#[doc = "Write proxy for field `INITMOD`"]
pub struct INITMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> INITMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The modulus is set to 64(same with 111111)."]
    #[inline(always)]
    pub fn _000000(self) -> &'a mut W {
        self.variant(INITMOD_A::_000000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Number of sample clocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSAM_A {
    #[doc = "0: The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    _00 = 0,
    #[doc = "1: The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    _01 = 1,
    #[doc = "2: The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    _10 = 2,
    #[doc = "3: The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    _11 = 3,
}
impl From<NSAM_A> for u8 {
    #[inline(always)]
    fn from(variant: NSAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NSAM`"]
pub type NSAM_R = crate::R<u8, NSAM_A>;
impl NSAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSAM_A {
        match self.bits {
            0 => NSAM_A::_00,
            1 => NSAM_A::_01,
            2 => NSAM_A::_10,
            3 => NSAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == NSAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == NSAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == NSAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == NSAM_A::_11
    }
}
#[doc = "Write proxy for field `NSAM`"]
pub struct NSAM_W<'a> {
    w: &'a mut W,
}
impl<'a> NSAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSAM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(NSAM_A::_00)
    }
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(NSAM_A::_01)
    }
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(NSAM_A::_10)
    }
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(NSAM_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `CH0F`"]
pub type CH0F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0F`"]
pub struct CH0F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0F_W<'a> {
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
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1F`"]
pub struct CH1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1F_W<'a> {
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
#[doc = "Reader of field `CH2F`"]
pub type CH2F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2F`"]
pub struct CH2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2F_W<'a> {
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
#[doc = "Reader of field `CH3F`"]
pub type CH3F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3F`"]
pub struct CH3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3F_W<'a> {
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
#[doc = "Reader of field `CH4F`"]
pub type CH4F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4F`"]
pub struct CH4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4F_W<'a> {
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
#[doc = "Reader of field `CH5F`"]
pub type CH5F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5F`"]
pub struct CH5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5F_W<'a> {
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
#[doc = "Reader of field `CH6F`"]
pub type CH6F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6F`"]
pub struct CH6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6F_W<'a> {
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
#[doc = "Reader of field `CH7F`"]
pub type CH7F_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7F`"]
pub struct CH7F_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7F_W<'a> {
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
#[doc = "Fixed channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FXMXCH_A {
    #[doc = "0: Channel 0 is selected as the fixed reference input for the fixed mux port."]
    _000 = 0,
    #[doc = "1: Channel 1 is selected as the fixed reference input for the fixed mux port."]
    _001 = 1,
    #[doc = "2: Channel 2 is selected as the fixed reference input for the fixed mux port."]
    _010 = 2,
    #[doc = "3: Channel 3 is selected as the fixed reference input for the fixed mux port."]
    _011 = 3,
    #[doc = "4: Channel 4 is selected as the fixed reference input for the fixed mux port."]
    _100 = 4,
    #[doc = "5: Channel 5 is selected as the fixed reference input for the fixed mux port."]
    _101 = 5,
    #[doc = "6: Channel 6 is selected as the fixed reference input for the fixed mux port."]
    _110 = 6,
    #[doc = "7: Channel 7 is selected as the fixed reference input for the fixed mux port."]
    _111 = 7,
}
impl From<FXMXCH_A> for u8 {
    #[inline(always)]
    fn from(variant: FXMXCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FXMXCH`"]
pub type FXMXCH_R = crate::R<u8, FXMXCH_A>;
impl FXMXCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXMXCH_A {
        match self.bits {
            0 => FXMXCH_A::_000,
            1 => FXMXCH_A::_001,
            2 => FXMXCH_A::_010,
            3 => FXMXCH_A::_011,
            4 => FXMXCH_A::_100,
            5 => FXMXCH_A::_101,
            6 => FXMXCH_A::_110,
            7 => FXMXCH_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FXMXCH_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FXMXCH_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FXMXCH_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FXMXCH_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FXMXCH_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FXMXCH_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FXMXCH_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FXMXCH_A::_111
    }
}
#[doc = "Write proxy for field `FXMXCH`"]
pub struct FXMXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> FXMXCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXMXCH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FXMXCH_A::_000)
    }
    #[doc = "Channel 1 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FXMXCH_A::_001)
    }
    #[doc = "Channel 2 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FXMXCH_A::_010)
    }
    #[doc = "Channel 3 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FXMXCH_A::_011)
    }
    #[doc = "Channel 4 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FXMXCH_A::_100)
    }
    #[doc = "Channel 5 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FXMXCH_A::_101)
    }
    #[doc = "Channel 6 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FXMXCH_A::_110)
    }
    #[doc = "Channel 7 is selected as the fixed reference input for the fixed mux port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FXMXCH_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Fixed MUX Port\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXMP_A {
    #[doc = "0: The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    _0 = 0,
    #[doc = "1: The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    _1 = 1,
}
impl From<FXMP_A> for bool {
    #[inline(always)]
    fn from(variant: FXMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FXMP`"]
pub type FXMP_R = crate::R<bool, FXMP_A>;
impl FXMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXMP_A {
        match self.bits {
            false => FXMP_A::_0,
            true => FXMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FXMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FXMP_A::_1
    }
}
#[doc = "Write proxy for field `FXMP`"]
pub struct FXMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FXMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FXMP_A::_0)
    }
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FXMP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Round-Robin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIE_A {
    #[doc = "0: The round-robin interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    _1 = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRIE`"]
pub type RRIE_R = crate::R<bool, RRIE_A>;
impl RRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::_0,
            true => RRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRIE_A::_1
    }
}
#[doc = "Write proxy for field `RRIE`"]
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The round-robin interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRIE_A::_0)
    }
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Round-Robin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRE_A {
    #[doc = "0: Round-robin operation is disabled."]
    _0 = 0,
    #[doc = "1: Round-robin operation is enabled."]
    _1 = 1,
}
impl From<RRE_A> for bool {
    #[inline(always)]
    fn from(variant: RRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRE`"]
pub type RRE_R = crate::R<bool, RRE_A>;
impl RRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRE_A {
        match self.bits {
            false => RRE_A::_0,
            true => RRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRE_A::_1
    }
}
#[doc = "Write proxy for field `RRE`"]
pub struct RRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Round-robin operation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRE_A::_0)
    }
    #[doc = "Round-robin operation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline(always)]
    pub fn acon(&self) -> ACON_R {
        ACON_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&self) -> INITMOD_R {
        INITMOD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&self) -> NSAM_R {
        NSAM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline(always)]
    pub fn ch7f(&self) -> CH7F_R {
        CH7F_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&self) -> FXMXCH_R {
        FXMXCH_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&self) -> FXMP_R {
        FXMP_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline(always)]
    pub fn rre(&self) -> RRE_R {
        RRE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - The result of the input comparison for channel n"]
    #[inline(always)]
    pub fn acon(&mut self) -> ACON_W {
        ACON_W { w: self }
    }
    #[doc = "Bits 8:13 - Comparator and DAC initialization delay modulus."]
    #[inline(always)]
    pub fn initmod(&mut self) -> INITMOD_W {
        INITMOD_W { w: self }
    }
    #[doc = "Bits 14:15 - Number of sample clocks"]
    #[inline(always)]
    pub fn nsam(&mut self) -> NSAM_W {
        NSAM_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input changed flag"]
    #[inline(always)]
    pub fn ch0f(&mut self) -> CH0F_W {
        CH0F_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input changed flag"]
    #[inline(always)]
    pub fn ch1f(&mut self) -> CH1F_W {
        CH1F_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input changed flag"]
    #[inline(always)]
    pub fn ch2f(&mut self) -> CH2F_W {
        CH2F_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input changed flag"]
    #[inline(always)]
    pub fn ch3f(&mut self) -> CH3F_W {
        CH3F_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input changed flag"]
    #[inline(always)]
    pub fn ch4f(&mut self) -> CH4F_W {
        CH4F_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input changed flag"]
    #[inline(always)]
    pub fn ch5f(&mut self) -> CH5F_W {
        CH5F_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input changed flag"]
    #[inline(always)]
    pub fn ch6f(&mut self) -> CH6F_W {
        CH6F_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input changed flag"]
    #[inline(always)]
    pub fn ch7f(&mut self) -> CH7F_W {
        CH7F_W { w: self }
    }
    #[doc = "Bits 25:27 - Fixed channel selection"]
    #[inline(always)]
    pub fn fxmxch(&mut self) -> FXMXCH_W {
        FXMXCH_W { w: self }
    }
    #[doc = "Bit 29 - Fixed MUX Port"]
    #[inline(always)]
    pub fn fxmp(&mut self) -> FXMP_W {
        FXMP_W { w: self }
    }
    #[doc = "Bit 30 - Round-Robin interrupt enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    #[doc = "Bit 31 - Round-Robin Enable"]
    #[inline(always)]
    pub fn rre(&mut self) -> RRE_W {
        RRE_W { w: self }
    }
}
