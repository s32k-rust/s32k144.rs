#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VERID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `FEATURE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEATURER {
    #[doc = "Standard feature set."]
    _0000000000000001,
    #[doc = "Standard feature set with MODEM/IrDA support."]
    _0000000000000011,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FEATURER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FEATURER::_0000000000000001 => 1,
            FEATURER::_0000000000000011 => 3,
            FEATURER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FEATURER {
        match value {
            1 => FEATURER::_0000000000000001,
            3 => FEATURER::_0000000000000011,
            i => FEATURER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000000000000001`"]
    #[inline]
    pub fn is_0000000000000001(&self) -> bool {
        *self == FEATURER::_0000000000000001
    }
    #[doc = "Checks if the value of the field is `_0000000000000011`"]
    #[inline]
    pub fn is_0000000000000011(&self) -> bool {
        *self == FEATURER::_0000000000000011
    }
}
#[doc = r" Value of the field"]
pub struct MINORR {
    bits: u8,
}
impl MINORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAJORR {
    bits: u8,
}
impl MAJORR {
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
    #[doc = "Bits 0:15 - Feature Identification Number"]
    #[inline]
    pub fn feature(&self) -> FEATURER {
        FEATURER::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline]
    pub fn minor(&self) -> MINORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MINORR { bits }
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline]
    pub fn major(&self) -> MAJORR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAJORR { bits }
    }
}
