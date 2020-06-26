#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Reader of field `FEATURES`"]
pub type FEATURES_R = crate::R<u8, u8>;
#[doc = "Package\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PACKAGE_A {
    #[doc = "2: 48 LQFP"]
    _0010 = 2,
    #[doc = "3: 64 LQFP"]
    _0011 = 3,
    #[doc = "4: 100 LQFP"]
    _0100 = 4,
    #[doc = "6: 144 LQFP"]
    _0110 = 6,
    #[doc = "7: 176 LQFP"]
    _0111 = 7,
    #[doc = "8: 100 MAP BGA"]
    _1000 = 8,
}
impl From<PACKAGE_A> for u8 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PACKAGE`"]
pub type PACKAGE_R = crate::R<u8, PACKAGE_A>;
impl PACKAGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PACKAGE_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(PACKAGE_A::_0010),
            3 => Val(PACKAGE_A::_0011),
            4 => Val(PACKAGE_A::_0100),
            6 => Val(PACKAGE_A::_0110),
            7 => Val(PACKAGE_A::_0111),
            8 => Val(PACKAGE_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PACKAGE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PACKAGE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PACKAGE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PACKAGE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PACKAGE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PACKAGE_A::_1000
    }
}
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMSIZE_A {
    #[doc = "7: 128 KB (S32K148), Reserved (others)"]
    _0111 = 7,
    #[doc = "9: 160 KB (S32K148) , Reserved (others)"]
    _1001 = 9,
    #[doc = "11: 192 KB (S32K148), 16 KB (S32K142), Reserved (others)"]
    _1011 = 11,
    #[doc = "13: 48 KB (S32K144), 24 KB (S32K142), Reserved (others)"]
    _1101 = 13,
    #[doc = "15: 256 KB (S32K148), 64 KB (S32K144), 32 KB (S32K142)"]
    _1111 = 15,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAMSIZE`"]
pub type RAMSIZE_R = crate::R<u8, RAMSIZE_A>;
impl RAMSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            7 => Val(RAMSIZE_A::_0111),
            9 => Val(RAMSIZE_A::_1001),
            11 => Val(RAMSIZE_A::_1011),
            13 => Val(RAMSIZE_A::_1101),
            15 => Val(RAMSIZE_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == RAMSIZE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == RAMSIZE_A::_1111
    }
}
#[doc = "Reader of field `DERIVATE`"]
pub type DERIVATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUBSERIES`"]
pub type SUBSERIES_R = crate::R<u8, u8>;
#[doc = "Reader of field `GENERATION`"]
pub type GENERATION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Features"]
    #[inline(always)]
    pub fn features(&self) -> FEATURES_R {
        FEATURES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Package"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Derivate"]
    #[inline(always)]
    pub fn derivate(&self) -> DERIVATE_R {
        DERIVATE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Subseries"]
    #[inline(always)]
    pub fn subseries(&self) -> SUBSERIES_R {
        SUBSERIES_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - S32K product series generation"]
    #[inline(always)]
    pub fn generation(&self) -> GENERATION_R {
        GENERATION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
