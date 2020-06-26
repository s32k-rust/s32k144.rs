#[doc = "Reader of register CSR"]
pub type R = crate::R<u32, super::CSR>;
#[doc = "Slow Clock Divide Ratio\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVSLOW_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
}
impl From<DIVSLOW_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVSLOW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVSLOW`"]
pub type DIVSLOW_R = crate::R<u8, DIVSLOW_A>;
impl DIVSLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVSLOW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVSLOW_A::_0000),
            1 => Val(DIVSLOW_A::_0001),
            2 => Val(DIVSLOW_A::_0010),
            3 => Val(DIVSLOW_A::_0011),
            4 => Val(DIVSLOW_A::_0100),
            5 => Val(DIVSLOW_A::_0101),
            6 => Val(DIVSLOW_A::_0110),
            7 => Val(DIVSLOW_A::_0111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DIVSLOW_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DIVSLOW_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DIVSLOW_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DIVSLOW_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DIVSLOW_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DIVSLOW_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DIVSLOW_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DIVSLOW_A::_0111
    }
}
#[doc = "Bus Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVBUS_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
    #[doc = "8: Divide-by-9"]
    _1000 = 8,
    #[doc = "9: Divide-by-10"]
    _1001 = 9,
    #[doc = "10: Divide-by-11"]
    _1010 = 10,
    #[doc = "11: Divide-by-12"]
    _1011 = 11,
    #[doc = "12: Divide-by-13"]
    _1100 = 12,
    #[doc = "13: Divide-by-14"]
    _1101 = 13,
    #[doc = "14: Divide-by-15"]
    _1110 = 14,
    #[doc = "15: Divide-by-16"]
    _1111 = 15,
}
impl From<DIVBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVBUS`"]
pub type DIVBUS_R = crate::R<u8, DIVBUS_A>;
impl DIVBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVBUS_A {
        match self.bits {
            0 => DIVBUS_A::_0000,
            1 => DIVBUS_A::_0001,
            2 => DIVBUS_A::_0010,
            3 => DIVBUS_A::_0011,
            4 => DIVBUS_A::_0100,
            5 => DIVBUS_A::_0101,
            6 => DIVBUS_A::_0110,
            7 => DIVBUS_A::_0111,
            8 => DIVBUS_A::_1000,
            9 => DIVBUS_A::_1001,
            10 => DIVBUS_A::_1010,
            11 => DIVBUS_A::_1011,
            12 => DIVBUS_A::_1100,
            13 => DIVBUS_A::_1101,
            14 => DIVBUS_A::_1110,
            15 => DIVBUS_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DIVBUS_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DIVBUS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DIVBUS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DIVBUS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DIVBUS_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DIVBUS_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DIVBUS_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DIVBUS_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DIVBUS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DIVBUS_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DIVBUS_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DIVBUS_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == DIVBUS_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == DIVBUS_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DIVBUS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DIVBUS_A::_1111
    }
}
#[doc = "Core Clock Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVCORE_A {
    #[doc = "0: Divide-by-1"]
    _0000 = 0,
    #[doc = "1: Divide-by-2"]
    _0001 = 1,
    #[doc = "2: Divide-by-3"]
    _0010 = 2,
    #[doc = "3: Divide-by-4"]
    _0011 = 3,
    #[doc = "4: Divide-by-5"]
    _0100 = 4,
    #[doc = "5: Divide-by-6"]
    _0101 = 5,
    #[doc = "6: Divide-by-7"]
    _0110 = 6,
    #[doc = "7: Divide-by-8"]
    _0111 = 7,
    #[doc = "8: Divide-by-9"]
    _1000 = 8,
    #[doc = "9: Divide-by-10"]
    _1001 = 9,
    #[doc = "10: Divide-by-11"]
    _1010 = 10,
    #[doc = "11: Divide-by-12"]
    _1011 = 11,
    #[doc = "12: Divide-by-13"]
    _1100 = 12,
    #[doc = "13: Divide-by-14"]
    _1101 = 13,
    #[doc = "14: Divide-by-15"]
    _1110 = 14,
    #[doc = "15: Divide-by-16"]
    _1111 = 15,
}
impl From<DIVCORE_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVCORE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVCORE`"]
pub type DIVCORE_R = crate::R<u8, DIVCORE_A>;
impl DIVCORE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVCORE_A {
        match self.bits {
            0 => DIVCORE_A::_0000,
            1 => DIVCORE_A::_0001,
            2 => DIVCORE_A::_0010,
            3 => DIVCORE_A::_0011,
            4 => DIVCORE_A::_0100,
            5 => DIVCORE_A::_0101,
            6 => DIVCORE_A::_0110,
            7 => DIVCORE_A::_0111,
            8 => DIVCORE_A::_1000,
            9 => DIVCORE_A::_1001,
            10 => DIVCORE_A::_1010,
            11 => DIVCORE_A::_1011,
            12 => DIVCORE_A::_1100,
            13 => DIVCORE_A::_1101,
            14 => DIVCORE_A::_1110,
            15 => DIVCORE_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DIVCORE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DIVCORE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DIVCORE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DIVCORE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DIVCORE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DIVCORE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DIVCORE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DIVCORE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == DIVCORE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == DIVCORE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == DIVCORE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == DIVCORE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == DIVCORE_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == DIVCORE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DIVCORE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DIVCORE_A::_1111
    }
}
#[doc = "System Clock Source\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCS_A {
    #[doc = "1: System OSC (SOSC_CLK)"]
    _0001 = 1,
    #[doc = "2: Slow IRC (SIRC_CLK)"]
    _0010 = 2,
    #[doc = "3: Fast IRC (FIRC_CLK)"]
    _0011 = 3,
    #[doc = "6: System PLL (SPLL_CLK)"]
    _0110 = 6,
}
impl From<SCS_A> for u8 {
    #[inline(always)]
    fn from(variant: SCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCS`"]
pub type SCS_R = crate::R<u8, SCS_A>;
impl SCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SCS_A::_0001),
            2 => Val(SCS_A::_0010),
            3 => Val(SCS_A::_0011),
            6 => Val(SCS_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SCS_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SCS_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SCS_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SCS_A::_0110
    }
}
impl R {
    #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
    #[inline(always)]
    pub fn divslow(&self) -> DIVSLOW_R {
        DIVSLOW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
    #[inline(always)]
    pub fn divbus(&self) -> DIVBUS_R {
        DIVBUS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
    #[inline(always)]
    pub fn divcore(&self) -> DIVCORE_R {
        DIVCORE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - System Clock Source"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
