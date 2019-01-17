#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCFG1 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct DEPARTR {
    bits: u8,
}
impl DEPARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EEERAMSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEERAMSIZER {
    #[doc = "4 KB"]
    _0010,
    #[doc = "2 KB"]
    _0011,
    #[doc = "1 KB"]
    _0100,
    #[doc = "512 Bytes"]
    _0101,
    #[doc = "256 Bytes"]
    _0110,
    #[doc = "128 Bytes"]
    _0111,
    #[doc = "64 Bytes"]
    _1000,
    #[doc = "32 Bytes"]
    _1001,
    #[doc = "0 Bytes"]
    _1111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EEERAMSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EEERAMSIZER::_0010 => 2,
            EEERAMSIZER::_0011 => 3,
            EEERAMSIZER::_0100 => 4,
            EEERAMSIZER::_0101 => 5,
            EEERAMSIZER::_0110 => 6,
            EEERAMSIZER::_0111 => 7,
            EEERAMSIZER::_1000 => 8,
            EEERAMSIZER::_1001 => 9,
            EEERAMSIZER::_1111 => 15,
            EEERAMSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EEERAMSIZER {
        match value {
            2 => EEERAMSIZER::_0010,
            3 => EEERAMSIZER::_0011,
            4 => EEERAMSIZER::_0100,
            5 => EEERAMSIZER::_0101,
            6 => EEERAMSIZER::_0110,
            7 => EEERAMSIZER::_0111,
            8 => EEERAMSIZER::_1000,
            9 => EEERAMSIZER::_1001,
            15 => EEERAMSIZER::_1111,
            i => EEERAMSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == EEERAMSIZER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == EEERAMSIZER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == EEERAMSIZER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == EEERAMSIZER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == EEERAMSIZER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == EEERAMSIZER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == EEERAMSIZER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == EEERAMSIZER::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == EEERAMSIZER::_1111
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 12:15 - FlexNVM partition"]
    #[inline]
    pub fn depart(&self) -> DEPARTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEPARTR { bits }
    }
    #[doc = "Bits 16:19 - EEE SRAM SIZE"]
    #[inline]
    pub fn eeeramsize(&self) -> EEERAMSIZER {
        EEERAMSIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
