#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OCMDR3 {
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
pub struct OCM0R {
    bits: u8,
}
impl OCM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OCM1R {
    bits: u8,
}
impl OCM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OCM2R {
    bits: u8,
}
impl OCM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OCMPUR {
    bits: bool,
}
impl OCMPUR {
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
#[doc = "Possible values of the field `OCMT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMTR {
    #[doc = "OCMEMn is a System RAM."]
    _000,
    #[doc = "OCMEMn is a Graphics RAM."]
    _001,
    #[doc = "OCMEMn is a ROM."]
    _011,
    #[doc = "OCMEMn is a Program Flash."]
    _100,
    #[doc = "OCMEMn is a Data Flash."]
    _101,
    #[doc = "OCMEMn is an EEE."]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OCMTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OCMTR::_000 => 0,
            OCMTR::_001 => 1,
            OCMTR::_011 => 3,
            OCMTR::_100 => 4,
            OCMTR::_101 => 5,
            OCMTR::_110 => 6,
            OCMTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OCMTR {
        match value {
            0 => OCMTR::_000,
            1 => OCMTR::_001,
            3 => OCMTR::_011,
            4 => OCMTR::_100,
            5 => OCMTR::_101,
            6 => OCMTR::_110,
            i => OCMTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == OCMTR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == OCMTR::_001
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == OCMTR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == OCMTR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == OCMTR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == OCMTR::_110
    }
}
#[doc = "Possible values of the field `RO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROR {
    #[doc = "Writes to the OCMDRn\\[11:0\\] are allowed"]
    _0,
    #[doc = "Writes to the OCMDRn\\[11:0\\] are ignored"]
    _1,
}
impl ROR {
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
            ROR::_0 => false,
            ROR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROR {
        match value {
            false => ROR::_0,
            true => ROR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ROR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ROR::_1
    }
}
#[doc = "Possible values of the field `OCMW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMWR {
    #[doc = "OCMEMn 32-bits wide"]
    _010,
    #[doc = "OCMEMn 64-bits wide"]
    _011,
    #[doc = "OCMEMn 128-bits wide"]
    _100,
    #[doc = "OCMEMn 256-bits wide"]
    _101,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OCMWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OCMWR::_010 => 2,
            OCMWR::_011 => 3,
            OCMWR::_100 => 4,
            OCMWR::_101 => 5,
            OCMWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OCMWR {
        match value {
            2 => OCMWR::_010,
            3 => OCMWR::_011,
            4 => OCMWR::_100,
            5 => OCMWR::_101,
            i => OCMWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == OCMWR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == OCMWR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == OCMWR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == OCMWR::_101
    }
}
#[doc = "Possible values of the field `OCMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMSZR {
    #[doc = "no OCMEMn"]
    _0000,
    #[doc = "1KB OCMEMn"]
    _0001,
    #[doc = "2KB OCMEMn"]
    _0010,
    #[doc = "4KB OCMEMn"]
    _0011,
    #[doc = "8KB OCMEMn"]
    _0100,
    #[doc = "16KB OCMEMn"]
    _0101,
    #[doc = "32KB OCMEMn"]
    _0110,
    #[doc = "64KB OCMEMn"]
    _0111,
    #[doc = "128KB OCMEMn"]
    _1000,
    #[doc = "256KB OCMEMn"]
    _1001,
    #[doc = "512KB OCMEMn"]
    _1010,
    #[doc = "1MB OCMEMn"]
    _1011,
    #[doc = "2MB OCMEMn"]
    _1100,
    #[doc = "4MB OCMEMn"]
    _1101,
    #[doc = "8MB OCMEMn"]
    _1110,
    #[doc = "16MB OCMEMn"]
    _1111,
}
impl OCMSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OCMSZR::_0000 => 0,
            OCMSZR::_0001 => 1,
            OCMSZR::_0010 => 2,
            OCMSZR::_0011 => 3,
            OCMSZR::_0100 => 4,
            OCMSZR::_0101 => 5,
            OCMSZR::_0110 => 6,
            OCMSZR::_0111 => 7,
            OCMSZR::_1000 => 8,
            OCMSZR::_1001 => 9,
            OCMSZR::_1010 => 10,
            OCMSZR::_1011 => 11,
            OCMSZR::_1100 => 12,
            OCMSZR::_1101 => 13,
            OCMSZR::_1110 => 14,
            OCMSZR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OCMSZR {
        match value {
            0 => OCMSZR::_0000,
            1 => OCMSZR::_0001,
            2 => OCMSZR::_0010,
            3 => OCMSZR::_0011,
            4 => OCMSZR::_0100,
            5 => OCMSZR::_0101,
            6 => OCMSZR::_0110,
            7 => OCMSZR::_0111,
            8 => OCMSZR::_1000,
            9 => OCMSZR::_1001,
            10 => OCMSZR::_1010,
            11 => OCMSZR::_1011,
            12 => OCMSZR::_1100,
            13 => OCMSZR::_1101,
            14 => OCMSZR::_1110,
            15 => OCMSZR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == OCMSZR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == OCMSZR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == OCMSZR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == OCMSZR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == OCMSZR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == OCMSZR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == OCMSZR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == OCMSZR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == OCMSZR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == OCMSZR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == OCMSZR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == OCMSZR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == OCMSZR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == OCMSZR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == OCMSZR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == OCMSZR::_1111
    }
}
#[doc = "Possible values of the field `OCMSZH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCMSZHR {
    #[doc = "OCMEMn is a power-of-2 capacity."]
    _0,
    #[doc = "OCMEMn is not a power-of-2, with a capacity is 0.75 * OCMSZ."]
    _1,
}
impl OCMSZHR {
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
            OCMSZHR::_0 => false,
            OCMSZHR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCMSZHR {
        match value {
            false => OCMSZHR::_0,
            true => OCMSZHR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == OCMSZHR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == OCMSZHR::_1
    }
}
#[doc = "Possible values of the field `V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR {
    #[doc = "OCMEMn is not present."]
    _0,
    #[doc = "OCMEMn is present."]
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
pub struct _OCM0W<'a> {
    w: &'a mut W,
}
impl<'a> _OCM0W<'a> {
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
pub struct _OCM1W<'a> {
    w: &'a mut W,
}
impl<'a> _OCM1W<'a> {
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
#[doc = r" Proxy"]
pub struct _OCM2W<'a> {
    w: &'a mut W,
}
impl<'a> _OCM2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RO`"]
pub enum ROW {
    #[doc = "Writes to the OCMDRn\\[11:0\\] are allowed"]
    _0,
    #[doc = "Writes to the OCMDRn\\[11:0\\] are ignored"]
    _1,
}
impl ROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ROW::_0 => false,
            ROW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ROW<'a> {
    w: &'a mut W,
}
impl<'a> _ROW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ROW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Writes to the OCMDRn\\[11:0\\] are allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROW::_0)
    }
    #[doc = "Writes to the OCMDRn\\[11:0\\] are ignored"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROW::_1)
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
    #[doc = "Bits 0:3 - OCMEM Control Field 0"]
    #[inline]
    pub fn ocm0(&self) -> OCM0R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OCM0R { bits }
    }
    #[doc = "Bits 4:7 - OCMEM Control Field 1"]
    #[inline]
    pub fn ocm1(&self) -> OCM1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OCM1R { bits }
    }
    #[doc = "Bits 8:11 - OCMEM Control Field 2"]
    #[inline]
    pub fn ocm2(&self) -> OCM2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OCM2R { bits }
    }
    #[doc = "Bit 12 - OCMPU"]
    #[inline]
    pub fn ocmpu(&self) -> OCMPUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OCMPUR { bits }
    }
    #[doc = "Bits 13:15 - OCMT"]
    #[inline]
    pub fn ocmt(&self) -> OCMTR {
        OCMTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - RO"]
    #[inline]
    pub fn ro(&self) -> ROR {
        ROR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:19 - OCMW"]
    #[inline]
    pub fn ocmw(&self) -> OCMWR {
        OCMWR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - OCMSZ"]
    #[inline]
    pub fn ocmsz(&self) -> OCMSZR {
        OCMSZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - OCMSZH"]
    #[inline]
    pub fn ocmszh(&self) -> OCMSZHR {
        OCMSZHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - V"]
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
        W { bits: 1073741824 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - OCMEM Control Field 0"]
    #[inline]
    pub fn ocm0(&mut self) -> _OCM0W {
        _OCM0W { w: self }
    }
    #[doc = "Bits 4:7 - OCMEM Control Field 1"]
    #[inline]
    pub fn ocm1(&mut self) -> _OCM1W {
        _OCM1W { w: self }
    }
    #[doc = "Bits 8:11 - OCMEM Control Field 2"]
    #[inline]
    pub fn ocm2(&mut self) -> _OCM2W {
        _OCM2W { w: self }
    }
    #[doc = "Bit 16 - RO"]
    #[inline]
    pub fn ro(&mut self) -> _ROW {
        _ROW { w: self }
    }
}
