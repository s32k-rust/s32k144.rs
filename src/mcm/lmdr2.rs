#[doc = "Reader of register LMDR2"]
pub type R = crate::R<u32, super::LMDR2>;
#[doc = "Writer for register LMDR2"]
pub type W = crate::W<u32, super::LMDR2>;
#[doc = "Register LMDR2 `reset()`'s with value 0x8424_40a0"]
impl crate::ResetValue for super::LMDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8424_40a0
    }
}
#[doc = "Reader of field `CF1`"]
pub type CF1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CF1`"]
pub struct CF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Memory Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MT_A {
    #[doc = "2: PC Cache"]
    _010 = 2,
}
impl From<MT_A> for u8 {
    #[inline(always)]
    fn from(variant: MT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MT`"]
pub type MT_R = crate::R<u8, MT_A>;
impl MT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MT_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(MT_A::_010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MT_A::_010
    }
}
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Writes to the LMDRn\\[7:0\\]
are allowed."]
    _0 = 0,
    #[doc = "1: Writes to the LMDRn\\[7:0\\]
are ignored."]
    _1 = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::_0,
            true => LOCK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCK_A::_1
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writes to the LMDRn\\[7:0\\]
are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCK_A::_0)
    }
    #[doc = "Writes to the LMDRn\\[7:0\\]
are ignored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCK_A::_1)
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
#[doc = "LMEM Data Path Width. This field defines the width of the local memory.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DPW_A {
    #[doc = "2: LMEMn 32-bits wide"]
    _010 = 2,
    #[doc = "3: LMEMn 64-bits wide"]
    _011 = 3,
}
impl From<DPW_A> for u8 {
    #[inline(always)]
    fn from(variant: DPW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DPW`"]
pub type DPW_R = crate::R<u8, DPW_A>;
impl DPW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DPW_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(DPW_A::_010),
            3 => Val(DPW_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DPW_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DPW_A::_011
    }
}
#[doc = "Level 1 Cache Ways\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WY_A {
    #[doc = "0: No Cache"]
    _0000 = 0,
    #[doc = "2: 2-Way Set Associative"]
    _0010 = 2,
    #[doc = "4: 4-Way Set Associative"]
    _0100 = 4,
}
impl From<WY_A> for u8 {
    #[inline(always)]
    fn from(variant: WY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WY`"]
pub type WY_R = crate::R<u8, WY_A>;
impl WY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WY_A::_0000),
            2 => Val(WY_A::_0010),
            4 => Val(WY_A::_0100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == WY_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == WY_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == WY_A::_0100
    }
}
#[doc = "LMEM Size\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LMSZ_A {
    #[doc = "4: 4 KB LMEMn"]
    _0100 = 4,
}
impl From<LMSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: LMSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LMSZ`"]
pub type LMSZ_R = crate::R<u8, LMSZ_A>;
impl LMSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LMSZ_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(LMSZ_A::_0100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == LMSZ_A::_0100
    }
}
#[doc = "LMEM Size Hole\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSZH_A {
    #[doc = "0: LMEMn is a power-of-2 capacity."]
    _0 = 0,
    #[doc = "1: LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ."]
    _1 = 1,
}
impl From<LMSZH_A> for bool {
    #[inline(always)]
    fn from(variant: LMSZH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LMSZH`"]
pub type LMSZH_R = crate::R<bool, LMSZH_A>;
impl LMSZH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LMSZH_A {
        match self.bits {
            false => LMSZH_A::_0,
            true => LMSZH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LMSZH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LMSZH_A::_1
    }
}
#[doc = "Local Memory Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: LMEMn is not present."]
    _0 = 0,
    #[doc = "1: LMEMn is present."]
    _1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::_0,
            true => V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == V_A::_1
    }
}
impl R {
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Memory Type"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - LMEM Data Path Width. This field defines the width of the local memory."]
    #[inline(always)]
    pub fn dpw(&self) -> DPW_R {
        DPW_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Level 1 Cache Ways"]
    #[inline(always)]
    pub fn wy(&self) -> WY_R {
        WY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LMEM Size"]
    #[inline(always)]
    pub fn lmsz(&self) -> LMSZ_R {
        LMSZ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - LMEM Size Hole"]
    #[inline(always)]
    pub fn lmszh(&self) -> LMSZH_R {
        LMSZH_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Local Memory Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline(always)]
    pub fn cf1(&mut self) -> CF1_W {
        CF1_W { w: self }
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
