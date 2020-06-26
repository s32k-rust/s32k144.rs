#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TCRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRC_A {
    #[doc = "0: 16-bit CRC protocol."]
    _0 = 0,
    #[doc = "1: 32-bit CRC protocol."]
    _1 = 1,
}
impl From<TCRC_A> for bool {
    #[inline(always)]
    fn from(variant: TCRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCRC`"]
pub type TCRC_R = crate::R<bool, TCRC_A>;
impl TCRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRC_A {
        match self.bits {
            false => TCRC_A::_0,
            true => TCRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCRC_A::_1
    }
}
#[doc = "Write proxy for field `TCRC`"]
pub struct TCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16-bit CRC protocol."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRC_A::_0)
    }
    #[doc = "32-bit CRC protocol."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write CRC Data Register As Seed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAS_A {
    #[doc = "0: Writes to the CRC data register are data values."]
    _0 = 0,
    #[doc = "1: Writes to the CRC data register are seed values."]
    _1 = 1,
}
impl From<WAS_A> for bool {
    #[inline(always)]
    fn from(variant: WAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAS`"]
pub type WAS_R = crate::R<bool, WAS_A>;
impl WAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAS_A {
        match self.bits {
            false => WAS_A::_0,
            true => WAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAS_A::_1
    }
}
#[doc = "Write proxy for field `WAS`"]
pub struct WAS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to the CRC data register are data values."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAS_A::_0)
    }
    #[doc = "Writes to the CRC data register are seed values."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Complement Read Of CRC Data Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FXOR_A {
    #[doc = "0: No XOR on reading."]
    _0 = 0,
    #[doc = "1: Invert or complement the read value of the CRC Data register."]
    _1 = 1,
}
impl From<FXOR_A> for bool {
    #[inline(always)]
    fn from(variant: FXOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FXOR`"]
pub type FXOR_R = crate::R<bool, FXOR_A>;
impl FXOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FXOR_A {
        match self.bits {
            false => FXOR_A::_0,
            true => FXOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FXOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FXOR_A::_1
    }
}
#[doc = "Write proxy for field `FXOR`"]
pub struct FXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FXOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FXOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No XOR on reading."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FXOR_A::_0)
    }
    #[doc = "Invert or complement the read value of the CRC Data register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FXOR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Type Of Transpose For Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOTR_A {
    #[doc = "0: No transposition."]
    _00 = 0,
    #[doc = "1: Bits in bytes are transposed; bytes are not transposed."]
    _01 = 1,
    #[doc = "2: Both bits in bytes and bytes are transposed."]
    _10 = 2,
    #[doc = "3: Only bytes are transposed; no bits in a byte are transposed."]
    _11 = 3,
}
impl From<TOTR_A> for u8 {
    #[inline(always)]
    fn from(variant: TOTR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TOTR`"]
pub type TOTR_R = crate::R<u8, TOTR_A>;
impl TOTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOTR_A {
        match self.bits {
            0 => TOTR_A::_00,
            1 => TOTR_A::_01,
            2 => TOTR_A::_10,
            3 => TOTR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOTR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOTR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOTR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOTR_A::_11
    }
}
#[doc = "Write proxy for field `TOTR`"]
pub struct TOTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOTR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No transposition."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOTR_A::_00)
    }
    #[doc = "Bits in bytes are transposed; bytes are not transposed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOTR_A::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOTR_A::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOTR_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Type Of Transpose For Writes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOT_A {
    #[doc = "0: No transposition."]
    _00 = 0,
    #[doc = "1: Bits in bytes are transposed; bytes are not transposed."]
    _01 = 1,
    #[doc = "2: Both bits in bytes and bytes are transposed."]
    _10 = 2,
    #[doc = "3: Only bytes are transposed; no bits in a byte are transposed."]
    _11 = 3,
}
impl From<TOT_A> for u8 {
    #[inline(always)]
    fn from(variant: TOT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TOT`"]
pub type TOT_R = crate::R<u8, TOT_A>;
impl TOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOT_A {
        match self.bits {
            0 => TOT_A::_00,
            1 => TOT_A::_01,
            2 => TOT_A::_10,
            3 => TOT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TOT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TOT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TOT_A::_11
    }
}
#[doc = "Write proxy for field `TOT`"]
pub struct TOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No transposition."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TOT_A::_00)
    }
    #[doc = "Bits in bytes are transposed; bytes are not transposed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TOT_A::_01)
    }
    #[doc = "Both bits in bytes and bytes are transposed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TOT_A::_10)
    }
    #[doc = "Only bytes are transposed; no bits in a byte are transposed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TOT_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - TCRC"]
    #[inline(always)]
    pub fn tcrc(&self) -> TCRC_R {
        TCRC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write CRC Data Register As Seed"]
    #[inline(always)]
    pub fn was(&self) -> WAS_R {
        WAS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Complement Read Of CRC Data Register"]
    #[inline(always)]
    pub fn fxor(&self) -> FXOR_R {
        FXOR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Type Of Transpose For Read"]
    #[inline(always)]
    pub fn totr(&self) -> TOTR_R {
        TOTR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Type Of Transpose For Writes"]
    #[inline(always)]
    pub fn tot(&self) -> TOT_R {
        TOT_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - TCRC"]
    #[inline(always)]
    pub fn tcrc(&mut self) -> TCRC_W {
        TCRC_W { w: self }
    }
    #[doc = "Bit 25 - Write CRC Data Register As Seed"]
    #[inline(always)]
    pub fn was(&mut self) -> WAS_W {
        WAS_W { w: self }
    }
    #[doc = "Bit 26 - Complement Read Of CRC Data Register"]
    #[inline(always)]
    pub fn fxor(&mut self) -> FXOR_W {
        FXOR_W { w: self }
    }
    #[doc = "Bits 28:29 - Type Of Transpose For Read"]
    #[inline(always)]
    pub fn totr(&mut self) -> TOTR_W {
        TOTR_W { w: self }
    }
    #[doc = "Bits 30:31 - Type Of Transpose For Writes"]
    #[inline(always)]
    pub fn tot(&mut self) -> TOT_W {
        TOT_W { w: self }
    }
}
