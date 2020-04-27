#[doc = "Reader of register FTMOPT0"]
pub type R = crate::R<u32, super::FTMOPT0>;
#[doc = "Writer for register FTMOPT0"]
pub type W = crate::W<u32, super::FTMOPT0>;
#[doc = "Register FTMOPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FTMOPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM0 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0FLTXSEL_A {
    #[doc = "0: FTM0_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM0 out"]
    _001 = 1,
}
impl From<FTM0FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM0FLTxSEL`"]
pub type FTM0FLTXSEL_R = crate::R<u8, FTM0FLTXSEL_A>;
impl FTM0FLTXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM0FLTXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM0FLTXSEL_A::_000),
            1 => Val(FTM0FLTXSEL_A::_001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FTM0FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FTM0FLTXSEL_A::_001
    }
}
#[doc = "Write proxy for field `FTM0FLTxSEL`"]
pub struct FTM0FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM0_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM0FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM0 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM0FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "FTM1 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1FLTXSEL_A {
    #[doc = "0: FTM1_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM1 out"]
    _001 = 1,
}
impl From<FTM1FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM1FLTxSEL`"]
pub type FTM1FLTXSEL_R = crate::R<u8, FTM1FLTXSEL_A>;
impl FTM1FLTXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM1FLTXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM1FLTXSEL_A::_000),
            1 => Val(FTM1FLTXSEL_A::_001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FTM1FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FTM1FLTXSEL_A::_001
    }
}
#[doc = "Write proxy for field `FTM1FLTxSEL`"]
pub struct FTM1FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM1_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM1FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM1 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM1FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "FTM2 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2FLTXSEL_A {
    #[doc = "0: FTM2_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM2 out"]
    _001 = 1,
}
impl From<FTM2FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2FLTxSEL`"]
pub type FTM2FLTXSEL_R = crate::R<u8, FTM2FLTXSEL_A>;
impl FTM2FLTXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM2FLTXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM2FLTXSEL_A::_000),
            1 => Val(FTM2FLTXSEL_A::_001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FTM2FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FTM2FLTXSEL_A::_001
    }
}
#[doc = "Write proxy for field `FTM2FLTxSEL`"]
pub struct FTM2FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM2FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM2 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM2FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "FTM3 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM3FLTXSEL_A {
    #[doc = "0: FTM3_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM3 out"]
    _001 = 1,
}
impl From<FTM3FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM3FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM3FLTxSEL`"]
pub type FTM3FLTXSEL_R = crate::R<u8, FTM3FLTXSEL_A>;
impl FTM3FLTXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTM3FLTXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTM3FLTXSEL_A::_000),
            1 => Val(FTM3FLTXSEL_A::_001),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FTM3FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FTM3FLTXSEL_A::_001
    }
}
#[doc = "Write proxy for field `FTM3FLTxSEL`"]
pub struct FTM3FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM3_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM3FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM3 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM3FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "FTM0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0CLKSEL_A {
    #[doc = "0: FTM0 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM0 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM0 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM0CLKSEL`"]
pub type FTM0CLKSEL_R = crate::R<u8, FTM0CLKSEL_A>;
impl FTM0CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0CLKSEL_A {
        match self.bits {
            0 => FTM0CLKSEL_A::_00,
            1 => FTM0CLKSEL_A::_01,
            2 => FTM0CLKSEL_A::_10,
            3 => FTM0CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM0CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM0CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM0CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM0CLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `FTM0CLKSEL`"]
pub struct FTM0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FTM0 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_00)
    }
    #[doc = "FTM0 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_01)
    }
    #[doc = "FTM0 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "FTM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1CLKSEL_A {
    #[doc = "0: FTM1 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM1 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM1 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM1CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM1CLKSEL`"]
pub type FTM1CLKSEL_R = crate::R<u8, FTM1CLKSEL_A>;
impl FTM1CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CLKSEL_A {
        match self.bits {
            0 => FTM1CLKSEL_A::_00,
            1 => FTM1CLKSEL_A::_01,
            2 => FTM1CLKSEL_A::_10,
            3 => FTM1CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM1CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM1CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM1CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM1CLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `FTM1CLKSEL`"]
pub struct FTM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FTM1 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_00)
    }
    #[doc = "FTM1 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_01)
    }
    #[doc = "FTM1 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "FTM2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2CLKSEL_A {
    #[doc = "0: FTM2 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM2 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM2 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM2CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM2CLKSEL`"]
pub type FTM2CLKSEL_R = crate::R<u8, FTM2CLKSEL_A>;
impl FTM2CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CLKSEL_A {
        match self.bits {
            0 => FTM2CLKSEL_A::_00,
            1 => FTM2CLKSEL_A::_01,
            2 => FTM2CLKSEL_A::_10,
            3 => FTM2CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM2CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM2CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM2CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM2CLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `FTM2CLKSEL`"]
pub struct FTM2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FTM2 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_00)
    }
    #[doc = "FTM2 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_01)
    }
    #[doc = "FTM2 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "FTM3 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM3CLKSEL_A {
    #[doc = "0: FTM3 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM3 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM3 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM3CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM3CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTM3CLKSEL`"]
pub type FTM3CLKSEL_R = crate::R<u8, FTM3CLKSEL_A>;
impl FTM3CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3CLKSEL_A {
        match self.bits {
            0 => FTM3CLKSEL_A::_00,
            1 => FTM3CLKSEL_A::_01,
            2 => FTM3CLKSEL_A::_10,
            3 => FTM3CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FTM3CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FTM3CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FTM3CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FTM3CLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `FTM3CLKSEL`"]
pub struct FTM3CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3CLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FTM3 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_00)
    }
    #[doc = "FTM3 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_01)
    }
    #[doc = "FTM3 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline(always)]
    pub fn ftm0fltx_sel(&self) -> FTM0FLTXSEL_R {
        FTM0FLTXSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline(always)]
    pub fn ftm1fltx_sel(&self) -> FTM1FLTXSEL_R {
        FTM1FLTXSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline(always)]
    pub fn ftm2fltx_sel(&self) -> FTM2FLTXSEL_R {
        FTM2FLTXSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline(always)]
    pub fn ftm3fltx_sel(&self) -> FTM3FLTXSEL_R {
        FTM3FLTXSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&self) -> FTM0CLKSEL_R {
        FTM0CLKSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&self) -> FTM1CLKSEL_R {
        FTM1CLKSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&self) -> FTM2CLKSEL_R {
        FTM2CLKSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&self) -> FTM3CLKSEL_R {
        FTM3CLKSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline(always)]
    pub fn ftm0fltx_sel(&mut self) -> FTM0FLTXSEL_W {
        FTM0FLTXSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline(always)]
    pub fn ftm1fltx_sel(&mut self) -> FTM1FLTXSEL_W {
        FTM1FLTXSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline(always)]
    pub fn ftm2fltx_sel(&mut self) -> FTM2FLTXSEL_W {
        FTM2FLTXSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline(always)]
    pub fn ftm3fltx_sel(&mut self) -> FTM3FLTXSEL_W {
        FTM3FLTXSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&mut self) -> FTM0CLKSEL_W {
        FTM0CLKSEL_W { w: self }
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&mut self) -> FTM1CLKSEL_W {
        FTM1CLKSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&mut self) -> FTM2CLKSEL_W {
        FTM2CLKSEL_W { w: self }
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&mut self) -> FTM3CLKSEL_W {
        FTM3CLKSEL_W { w: self }
    }
}
