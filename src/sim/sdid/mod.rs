#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SDID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FEATURESR {
    bits: u8,
}
impl FEATURESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PACKAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKAGER {
    #[doc = "48 LQFP"] _0010,
    #[doc = "64 LQFP"] _0011,
    #[doc = "100 LQFP"] _0100,
    #[doc = "144 LQFP"] _0110,
    #[doc = "176 LQFP"] _0111,
    #[doc = "100 MAP BGA"] _1000,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl PACKAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PACKAGER::_0010 => 2,
            PACKAGER::_0011 => 3,
            PACKAGER::_0100 => 4,
            PACKAGER::_0110 => 6,
            PACKAGER::_0111 => 7,
            PACKAGER::_1000 => 8,
            PACKAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PACKAGER {
        match value {
            2 => PACKAGER::_0010,
            3 => PACKAGER::_0011,
            4 => PACKAGER::_0100,
            6 => PACKAGER::_0110,
            7 => PACKAGER::_0111,
            8 => PACKAGER::_1000,
            i => PACKAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PACKAGER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PACKAGER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PACKAGER::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PACKAGER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PACKAGER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PACKAGER::_1000
    }
}
#[doc = r" Value of the field"]
pub struct REVIDR {
    bits: u8,
}
impl REVIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMSIZER {
    #[doc = "128 KB (S32K148), Reserved (others)"] _0111,
    #[doc = "160 KB (S32K148) , Reserved (others)"] _1001,
    #[doc = "192 KB (S32K148), 16 KB (S32K142), Reserved (others)"] _1011,
    #[doc = "48 KB (S32K144), 24 KB (S32K142), Reserved (others)"] _1101,
    #[doc = "256 KB (S32K148), 64 KB (S32K144), 32 KB (S32K142)"] _1111,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl RAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMSIZER::_0111 => 7,
            RAMSIZER::_1001 => 9,
            RAMSIZER::_1011 => 11,
            RAMSIZER::_1101 => 13,
            RAMSIZER::_1111 => 15,
            RAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMSIZER {
        match value {
            7 => RAMSIZER::_0111,
            9 => RAMSIZER::_1001,
            11 => RAMSIZER::_1011,
            13 => RAMSIZER::_1101,
            15 => RAMSIZER::_1111,
            i => RAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZER::_1011
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == RAMSIZER::_1101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == RAMSIZER::_1111
    }
}
#[doc = r" Value of the field"]
pub struct DERIVATER {
    bits: u8,
}
impl DERIVATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SUBSERIESR {
    bits: u8,
}
impl SUBSERIESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GENERATIONR {
    bits: u8,
}
impl GENERATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Features"]
    #[inline]
    pub fn features(&self) -> FEATURESR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FEATURESR { bits }
    }
    #[doc = "Bits 8:11 - Package"]
    #[inline]
    pub fn package(&self) -> PACKAGER {
        PACKAGER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline]
    pub fn revid(&self) -> REVIDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REVIDR { bits }
    }
    #[doc = "Bits 16:19 - RAM size"]
    #[inline]
    pub fn ramsize(&self) -> RAMSIZER {
        RAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Derivate"]
    #[inline]
    pub fn derivate(&self) -> DERIVATER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DERIVATER { bits }
    }
    #[doc = "Bits 24:27 - Subseries"]
    #[inline]
    pub fn subseries(&self) -> SUBSERIESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SUBSERIESR { bits }
    }
    #[doc = "Bits 28:31 - S32K product series generation"]
    #[inline]
    pub fn generation(&self) -> GENERATIONR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GENERATIONR { bits }
    }
}
