#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LMFATR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct PEFPRTR {
    bits: u8,
}
impl PEFPRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PEFSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEFSIZER {
    #[doc = "8-bit access"]
    _000,
    #[doc = "16-bit access"]
    _001,
    #[doc = "32-bit access"]
    _010,
    #[doc = "64-bit access"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PEFSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PEFSIZER::_000 => 0,
            PEFSIZER::_001 => 1,
            PEFSIZER::_010 => 2,
            PEFSIZER::_011 => 3,
            PEFSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PEFSIZER {
        match value {
            0 => PEFSIZER::_000,
            1 => PEFSIZER::_001,
            2 => PEFSIZER::_010,
            3 => PEFSIZER::_011,
            i => PEFSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PEFSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PEFSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PEFSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PEFSIZER::_011
    }
}
#[doc = r" Value of the field"]
pub struct PEFWR {
    bits: bool,
}
impl PEFWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PEFMSTR {
    bits: u8,
}
impl PEFMSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OVRR {
    bits: bool,
}
impl OVRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Parity/ECC Fault Protection"]
    #[inline]
    pub fn pefprt(&self) -> PEFPRTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PEFPRTR { bits }
    }
    #[doc = "Bits 4:6 - Parity/ECC Fault Master Size"]
    #[inline]
    pub fn pefsize(&self) -> PEFSIZER {
        PEFSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Parity/ECC Fault Write"]
    #[inline]
    pub fn pefw(&self) -> PEFWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PEFWR { bits }
    }
    #[doc = "Bits 8:15 - Parity/ECC Fault Master Number"]
    #[inline]
    pub fn pefmst(&self) -> PEFMSTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PEFMSTR { bits }
    }
    #[doc = "Bit 31 - Overrun"]
    #[inline]
    pub fn ovr(&self) -> OVRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVRR { bits }
    }
}
