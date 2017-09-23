#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LMDR1 {
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
pub struct CF0R {
    bits: u8,
}
impl CF0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CF1R {
    bits: u8,
}
impl CF1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTR {
    #[doc = "SRAM_L"]
    _000,
    #[doc = "SRAM_U"]
    _001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MTR::_000 => 0,
            MTR::_001 => 1,
            MTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MTR {
        match value {
            0 => MTR::_000,
            1 => MTR::_001,
            i => MTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MTR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == MTR::_001
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "Writes to the LMDRn[7:0] are allowed."]
    _0,
    #[doc = "Writes to the LMDRn[7:0] are ignored."]
    _1,
}
impl LOCKR {
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
            LOCKR::_0 => false,
            LOCKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::_0,
            true => LOCKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCKR::_1
    }
}
#[doc = "Possible values of the field `DPW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPWR {
    #[doc = "LMEMn 32-bits wide"]
    _010,
    #[doc = "LMEMn 64-bits wide"]
    _011,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DPWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DPWR::_010 => 2,
            DPWR::_011 => 3,
            DPWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DPWR {
        match value {
            2 => DPWR::_010,
            3 => DPWR::_011,
            i => DPWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == DPWR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == DPWR::_011
    }
}
#[doc = "Possible values of the field `WY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WYR {
    #[doc = "No Cache"]
    _0000,
    #[doc = "2-Way Set Associative"]
    _0010,
    #[doc = "4-Way Set Associative"]
    _0100,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WYR::_0000 => 0,
            WYR::_0010 => 2,
            WYR::_0100 => 4,
            WYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WYR {
        match value {
            0 => WYR::_0000,
            2 => WYR::_0010,
            4 => WYR::_0100,
            i => WYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == WYR::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == WYR::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == WYR::_0100
    }
}
#[doc = "Possible values of the field `LMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSZR {
    #[doc = "no LMEMn (0 KB)"]
    _0000,
    #[doc = "1 KB LMEMn"]
    _0001,
    #[doc = "2 KB LMEMn"]
    _0010,
    #[doc = "4 KB LMEMn"]
    _0011,
    #[doc = "8 KB LMEMn"]
    _0100,
    #[doc = "16 KB LMEMn"]
    _0101,
    #[doc = "32 KB LMEMn"]
    _0110,
    #[doc = "64 KB LMEMn"]
    _0111,
    #[doc = "128 KB LMEMn"]
    _1000,
    #[doc = "256 KB LMEMn"]
    _1001,
    #[doc = "512 KB LMEMn"]
    _1010,
    #[doc = "1024 KB LMEMn"]
    _1011,
    #[doc = "2048 KB LMEMn"]
    _1100,
    #[doc = "4096 KB LMEMn"]
    _1101,
    #[doc = "8192 KB LMEMn"]
    _1110,
    #[doc = "16384 KB LMEMn"]
    _1111,
}
impl LMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LMSZR::_0000 => 0,
            LMSZR::_0001 => 1,
            LMSZR::_0010 => 2,
            LMSZR::_0011 => 3,
            LMSZR::_0100 => 4,
            LMSZR::_0101 => 5,
            LMSZR::_0110 => 6,
            LMSZR::_0111 => 7,
            LMSZR::_1000 => 8,
            LMSZR::_1001 => 9,
            LMSZR::_1010 => 10,
            LMSZR::_1011 => 11,
            LMSZR::_1100 => 12,
            LMSZR::_1101 => 13,
            LMSZR::_1110 => 14,
            LMSZR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LMSZR {
        match value {
            0 => LMSZR::_0000,
            1 => LMSZR::_0001,
            2 => LMSZR::_0010,
            3 => LMSZR::_0011,
            4 => LMSZR::_0100,
            5 => LMSZR::_0101,
            6 => LMSZR::_0110,
            7 => LMSZR::_0111,
            8 => LMSZR::_1000,
            9 => LMSZR::_1001,
            10 => LMSZR::_1010,
            11 => LMSZR::_1011,
            12 => LMSZR::_1100,
            13 => LMSZR::_1101,
            14 => LMSZR::_1110,
            15 => LMSZR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == LMSZR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == LMSZR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == LMSZR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == LMSZR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == LMSZR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == LMSZR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == LMSZR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == LMSZR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == LMSZR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == LMSZR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == LMSZR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == LMSZR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == LMSZR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == LMSZR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == LMSZR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == LMSZR::_1111
    }
}
#[doc = "Possible values of the field `LMSZH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LMSZHR {
    #[doc = "LMEMn is a power-of-2 capacity."]
    _0,
    #[doc = "LMEMn is not a power-of-2, with a capacity is 0.75 * LMSZ."]
    _1,
}
impl LMSZHR {
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
            LMSZHR::_0 => false,
            LMSZHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LMSZHR {
        match value {
            false => LMSZHR::_0,
            true => LMSZHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LMSZHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LMSZHR::_1
    }
}
#[doc = "Possible values of the field `V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR {
    #[doc = "LMEMn is not present."]
    _0,
    #[doc = "LMEMn is present."]
    _1,
}
impl VR {
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
            VR::_0 => false,
            VR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VR {
        match value {
            false => VR::_0,
            true => VR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VR::_1
    }
}
#[doc = r" Proxy"]
pub struct _CF0W<'a> {
    w: &'a mut W,
}
impl<'a> _CF0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CF1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "Writes to the LMDRn[7:0] are allowed."]
    _0,
    #[doc = "Writes to the LMDRn[7:0] are ignored."]
    _1,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::_0 => false,
            LOCKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the LMDRn[7:0] are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKW::_0)
    }
    #[doc = "Writes to the LMDRn[7:0] are ignored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Control Field 0"]
    #[inline]
    pub fn cf0(&self) -> CF0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CF0R { bits }
    }
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline]
    pub fn cf1(&self) -> CF1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CF1R { bits }
    }
    #[doc = "Bits 13:15 - Memory Type"]
    #[inline]
    pub fn mt(&self) -> MTR {
        MTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - LMEM Data Path Width. This field defines the width of the local memory."]
    #[inline]
    pub fn dpw(&self) -> DPWR {
        DPWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Level 1 Cache Ways"]
    #[inline]
    pub fn wy(&self) -> WYR {
        WYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - LMEM Size"]
    #[inline]
    pub fn lmsz(&self) -> LMSZR {
        LMSZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - LMEM Size Hole"]
    #[inline]
    pub fn lmszh(&self) -> LMSZHR {
        LMSZHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Local Memory Valid"]
    #[inline]
    pub fn v(&self) -> VR {
        VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - Control Field 0"]
    #[inline]
    pub fn cf0(&mut self) -> _CF0W {
        _CF0W { w: self }
    }
    #[doc = "Bits 4:7 - Control Field 1"]
    #[inline]
    pub fn cf1(&mut self) -> _CF1W {
        _CF1W { w: self }
    }
    #[doc = "Bit 16 - LOCK"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}
