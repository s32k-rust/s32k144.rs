#[doc = "Reader of register FCFG1"]
pub type R = crate::R<u32, super::FCFG1>;
#[doc = "Writer for register FCFG1"]
pub type W = crate::W<u32, super::FCFG1>;
#[doc = "Register FCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEPART`"]
pub type DEPART_R = crate::R<u8, u8>;
#[doc = "EEE SRAM SIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEERAMSIZE_A {
    #[doc = "2: 4 KB"]
    _0010 = 2,
    #[doc = "3: 2 KB"]
    _0011 = 3,
    #[doc = "4: 1 KB"]
    _0100 = 4,
    #[doc = "5: 512 Bytes"]
    _0101 = 5,
    #[doc = "6: 256 Bytes"]
    _0110 = 6,
    #[doc = "7: 128 Bytes"]
    _0111 = 7,
    #[doc = "8: 64 Bytes"]
    _1000 = 8,
    #[doc = "9: 32 Bytes"]
    _1001 = 9,
    #[doc = "15: 0 Bytes"]
    _1111 = 15,
}
impl From<EEERAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EEERAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EEERAMSIZE`"]
pub type EEERAMSIZE_R = crate::R<u8, EEERAMSIZE_A>;
impl EEERAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EEERAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(EEERAMSIZE_A::_0010),
            3 => Val(EEERAMSIZE_A::_0011),
            4 => Val(EEERAMSIZE_A::_0100),
            5 => Val(EEERAMSIZE_A::_0101),
            6 => Val(EEERAMSIZE_A::_0110),
            7 => Val(EEERAMSIZE_A::_0111),
            8 => Val(EEERAMSIZE_A::_1000),
            9 => Val(EEERAMSIZE_A::_1001),
            15 => Val(EEERAMSIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == EEERAMSIZE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == EEERAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == EEERAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == EEERAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == EEERAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == EEERAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == EEERAMSIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == EEERAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == EEERAMSIZE_A::_1111
    }
}
impl R {
    #[doc = "Bits 12:15 - FlexNVM partition"]
    #[inline(always)]
    pub fn depart(&self) -> DEPART_R {
        DEPART_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEE SRAM SIZE"]
    #[inline(always)]
    pub fn eeeramsize(&self) -> EEERAMSIZE_R {
        EEERAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {}
