#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPXCFG3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPUR {
    #[doc = "FPU support is not included."]
    _0,
    #[doc = "FPU support is included."]
    _1,
}
impl FPUR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FPUR::_0 => false,
            FPUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPUR {
        match value {
            false => FPUR::_0,
            true => FPUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FPUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FPUR::_1
    }
}
#[doc = "Possible values of the field `SIMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIMDR {
    #[doc = "SIMD/NEON support is not included."]
    _0,
    #[doc = "SIMD/NEON support is included."]
    _1,
}
impl SIMDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SIMDR::_0 => false,
            SIMDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIMDR {
        match value {
            false => SIMDR::_0,
            true => SIMDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SIMDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SIMDR::_1
    }
}
#[doc = "Possible values of the field `JAZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JAZR {
    #[doc = "Jazelle support is not included."]
    _0,
    #[doc = "Jazelle support is included."]
    _1,
}
impl JAZR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            JAZR::_0 => false,
            JAZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JAZR {
        match value {
            false => JAZR::_0,
            true => JAZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == JAZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == JAZR::_1
    }
}
#[doc = "Possible values of the field `MMU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMUR {
    #[doc = "MMU support is not included."]
    _0,
    #[doc = "MMU support is included."]
    _1,
}
impl MMUR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MMUR::_0 => false,
            MMUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MMUR {
        match value {
            false => MMUR::_0,
            true => MMUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MMUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MMUR::_1
    }
}
#[doc = "Possible values of the field `TZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZR {
    #[doc = "Trust Zone support is not included."]
    _0,
    #[doc = "Trust Zone support is included."]
    _1,
}
impl TZR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TZR::_0 => false,
            TZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TZR {
        match value {
            false => TZR::_0,
            true => TZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TZR::_1
    }
}
#[doc = "Possible values of the field `CMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPR {
    #[doc = "Core Memory Protection is not included."]
    _0,
    #[doc = "Core Memory Protection is included."]
    _1,
}
impl CMPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CMPR::_0 => false,
            CMPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPR {
        match value {
            false => CMPR::_0,
            true => CMPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CMPR::_1
    }
}
#[doc = "Possible values of the field `BB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBR {
    #[doc = "Bit Banding is not supported."]
    _0,
    #[doc = "Bit Banding is supported."]
    _1,
}
impl BBR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BBR::_0 => false,
            BBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBR {
        match value {
            false => BBR::_0,
            true => BBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBR::_1
    }
}
#[doc = r" Value of the field"]
pub struct SBPR {
    bits: u8,
}
impl SBPR {
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
    #[doc = "Bit 0 - Floating Point Unit"]
    #[inline]
    pub fn fpu(&self) -> FPUR {
        FPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SIMD/NEON instruction support"]
    #[inline]
    pub fn simd(&self) -> SIMDR {
        SIMDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Jazelle support"]
    #[inline]
    pub fn jaz(&self) -> JAZR {
        JAZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Memory Management Unit"]
    #[inline]
    pub fn mmu(&self) -> MMUR {
        MMUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Trust Zone"]
    #[inline]
    pub fn tz(&self) -> TZR {
        TZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Core Memory Protection unit"]
    #[inline]
    pub fn cmp(&self) -> CMPR {
        CMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Bit Banding"]
    #[inline]
    pub fn bb(&self) -> BBR {
        BBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - System Bus Ports"]
    #[inline]
    pub fn sbp(&self) -> SBPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SBPR { bits }
    }
}
