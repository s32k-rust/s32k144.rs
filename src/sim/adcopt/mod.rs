#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCOPT {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Possible values of the field `ADC0TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0TRGSELR {
    #[doc = "PDB output"]
    _0,
    #[doc = "TRGMUX output"]
    _1,
}
impl ADC0TRGSELR {
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
            ADC0TRGSELR::_0 => false,
            ADC0TRGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC0TRGSELR {
        match value {
            false => ADC0TRGSELR::_0,
            true => ADC0TRGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC0TRGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC0TRGSELR::_1
    }
}
#[doc = "Possible values of the field `ADC0SWPRETRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0SWPRETRGR {
    #[doc = "Software pretrigger disabled"]
    _000,
    #[doc = "Reserved (do not use)"]
    _001,
    #[doc = "Reserved (do not use)"]
    _010,
    #[doc = "Reserved (do not use)"]
    _011,
    #[doc = "Software pretrigger 0"]
    _100,
    #[doc = "Software pretrigger 1"]
    _101,
    #[doc = "Software pretrigger 2"]
    _110,
    #[doc = "Software pretrigger 3"]
    _111,
}
impl ADC0SWPRETRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0SWPRETRGR::_000 => 0,
            ADC0SWPRETRGR::_001 => 1,
            ADC0SWPRETRGR::_010 => 2,
            ADC0SWPRETRGR::_011 => 3,
            ADC0SWPRETRGR::_100 => 4,
            ADC0SWPRETRGR::_101 => 5,
            ADC0SWPRETRGR::_110 => 6,
            ADC0SWPRETRGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0SWPRETRGR {
        match value {
            0 => ADC0SWPRETRGR::_000,
            1 => ADC0SWPRETRGR::_001,
            2 => ADC0SWPRETRGR::_010,
            3 => ADC0SWPRETRGR::_011,
            4 => ADC0SWPRETRGR::_100,
            5 => ADC0SWPRETRGR::_101,
            6 => ADC0SWPRETRGR::_110,
            7 => ADC0SWPRETRGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ADC0SWPRETRGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == ADC0SWPRETRGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == ADC0SWPRETRGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == ADC0SWPRETRGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == ADC0SWPRETRGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == ADC0SWPRETRGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == ADC0SWPRETRGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == ADC0SWPRETRGR::_111
    }
}
#[doc = "Possible values of the field `ADC0PRETRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0PRETRGSELR {
    #[doc = "PDB pretrigger (default)"]
    _00,
    #[doc = "TRGMUX pretrigger"]
    _01,
    #[doc = "Software pretrigger"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC0PRETRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0PRETRGSELR::_00 => 0,
            ADC0PRETRGSELR::_01 => 1,
            ADC0PRETRGSELR::_10 => 2,
            ADC0PRETRGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0PRETRGSELR {
        match value {
            0 => ADC0PRETRGSELR::_00,
            1 => ADC0PRETRGSELR::_01,
            2 => ADC0PRETRGSELR::_10,
            i => ADC0PRETRGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADC0PRETRGSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADC0PRETRGSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADC0PRETRGSELR::_10
    }
}
#[doc = "Possible values of the field `ADC1TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1TRGSELR {
    #[doc = "PDB output"]
    _0,
    #[doc = "TRGMUX output"]
    _1,
}
impl ADC1TRGSELR {
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
            ADC1TRGSELR::_0 => false,
            ADC1TRGSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC1TRGSELR {
        match value {
            false => ADC1TRGSELR::_0,
            true => ADC1TRGSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADC1TRGSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADC1TRGSELR::_1
    }
}
#[doc = "Possible values of the field `ADC1SWPRETRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1SWPRETRGR {
    #[doc = "Software pretrigger disabled"]
    _000,
    #[doc = "Reserved (do not use)"]
    _001,
    #[doc = "Reserved (do not use)"]
    _010,
    #[doc = "Reserved (do not use)"]
    _011,
    #[doc = "Software pretrigger 0"]
    _100,
    #[doc = "Software pretrigger 1"]
    _101,
    #[doc = "Software pretrigger 2"]
    _110,
    #[doc = "Software pretrigger 3"]
    _111,
}
impl ADC1SWPRETRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC1SWPRETRGR::_000 => 0,
            ADC1SWPRETRGR::_001 => 1,
            ADC1SWPRETRGR::_010 => 2,
            ADC1SWPRETRGR::_011 => 3,
            ADC1SWPRETRGR::_100 => 4,
            ADC1SWPRETRGR::_101 => 5,
            ADC1SWPRETRGR::_110 => 6,
            ADC1SWPRETRGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC1SWPRETRGR {
        match value {
            0 => ADC1SWPRETRGR::_000,
            1 => ADC1SWPRETRGR::_001,
            2 => ADC1SWPRETRGR::_010,
            3 => ADC1SWPRETRGR::_011,
            4 => ADC1SWPRETRGR::_100,
            5 => ADC1SWPRETRGR::_101,
            6 => ADC1SWPRETRGR::_110,
            7 => ADC1SWPRETRGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ADC1SWPRETRGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == ADC1SWPRETRGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == ADC1SWPRETRGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == ADC1SWPRETRGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == ADC1SWPRETRGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == ADC1SWPRETRGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == ADC1SWPRETRGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == ADC1SWPRETRGR::_111
    }
}
#[doc = "Possible values of the field `ADC1PRETRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1PRETRGSELR {
    #[doc = "PDB pretrigger (default)"]
    _00,
    #[doc = "TRGMUX pretrigger"]
    _01,
    #[doc = "Software pretrigger"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC1PRETRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC1PRETRGSELR::_00 => 0,
            ADC1PRETRGSELR::_01 => 1,
            ADC1PRETRGSELR::_10 => 2,
            ADC1PRETRGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC1PRETRGSELR {
        match value {
            0 => ADC1PRETRGSELR::_00,
            1 => ADC1PRETRGSELR::_01,
            2 => ADC1PRETRGSELR::_10,
            i => ADC1PRETRGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADC1PRETRGSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADC1PRETRGSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADC1PRETRGSELR::_10
    }
}
#[doc = "Values that can be written to the field `ADC0TRGSEL`"]
pub enum ADC0TRGSELW {
    #[doc = "PDB output"]
    _0,
    #[doc = "TRGMUX output"]
    _1,
}
impl ADC0TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC0TRGSELW::_0 => false,
            ADC0TRGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0TRGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_0)
    }
    #[doc = "TRGMUX output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0TRGSELW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC0SWPRETRG`"]
pub enum ADC0SWPRETRGW {
    #[doc = "Software pretrigger disabled"]
    _000,
    #[doc = "Reserved (do not use)"]
    _001,
    #[doc = "Reserved (do not use)"]
    _010,
    #[doc = "Reserved (do not use)"]
    _011,
    #[doc = "Software pretrigger 0"]
    _100,
    #[doc = "Software pretrigger 1"]
    _101,
    #[doc = "Software pretrigger 2"]
    _110,
    #[doc = "Software pretrigger 3"]
    _111,
}
impl ADC0SWPRETRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0SWPRETRGW::_000 => 0,
            ADC0SWPRETRGW::_001 => 1,
            ADC0SWPRETRGW::_010 => 2,
            ADC0SWPRETRGW::_011 => 3,
            ADC0SWPRETRGW::_100 => 4,
            ADC0SWPRETRGW::_101 => 5,
            ADC0SWPRETRGW::_110 => 6,
            ADC0SWPRETRGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0SWPRETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0SWPRETRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0SWPRETRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Software pretrigger disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_000)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_001)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_010)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_011)
    }
    #[doc = "Software pretrigger 0"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_100)
    }
    #[doc = "Software pretrigger 1"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_101)
    }
    #[doc = "Software pretrigger 2"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_110)
    }
    #[doc = "Software pretrigger 3"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(ADC0SWPRETRGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC0PRETRGSEL`"]
pub enum ADC0PRETRGSELW {
    #[doc = "PDB pretrigger (default)"]
    _00,
    #[doc = "TRGMUX pretrigger"]
    _01,
    #[doc = "Software pretrigger"]
    _10,
}
impl ADC0PRETRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0PRETRGSELW::_00 => 0,
            ADC0PRETRGSELW::_01 => 1,
            ADC0PRETRGSELW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0PRETRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0PRETRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0PRETRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB pretrigger (default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADC0PRETRGSELW::_00)
    }
    #[doc = "TRGMUX pretrigger"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADC0PRETRGSELW::_01)
    }
    #[doc = "Software pretrigger"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADC0PRETRGSELW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1TRGSEL`"]
pub enum ADC1TRGSELW {
    #[doc = "PDB output"]
    _0,
    #[doc = "TRGMUX output"]
    _1,
}
impl ADC1TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC1TRGSELW::_0 => false,
            ADC1TRGSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1TRGSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1TRGSELW::_0)
    }
    #[doc = "TRGMUX output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1TRGSELW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1SWPRETRG`"]
pub enum ADC1SWPRETRGW {
    #[doc = "Software pretrigger disabled"]
    _000,
    #[doc = "Reserved (do not use)"]
    _001,
    #[doc = "Reserved (do not use)"]
    _010,
    #[doc = "Reserved (do not use)"]
    _011,
    #[doc = "Software pretrigger 0"]
    _100,
    #[doc = "Software pretrigger 1"]
    _101,
    #[doc = "Software pretrigger 2"]
    _110,
    #[doc = "Software pretrigger 3"]
    _111,
}
impl ADC1SWPRETRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC1SWPRETRGW::_000 => 0,
            ADC1SWPRETRGW::_001 => 1,
            ADC1SWPRETRGW::_010 => 2,
            ADC1SWPRETRGW::_011 => 3,
            ADC1SWPRETRGW::_100 => 4,
            ADC1SWPRETRGW::_101 => 5,
            ADC1SWPRETRGW::_110 => 6,
            ADC1SWPRETRGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1SWPRETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1SWPRETRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1SWPRETRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Software pretrigger disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_000)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_001)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_010)
    }
    #[doc = "Reserved (do not use)"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_011)
    }
    #[doc = "Software pretrigger 0"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_100)
    }
    #[doc = "Software pretrigger 1"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_101)
    }
    #[doc = "Software pretrigger 2"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_110)
    }
    #[doc = "Software pretrigger 3"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(ADC1SWPRETRGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1PRETRGSEL`"]
pub enum ADC1PRETRGSELW {
    #[doc = "PDB pretrigger (default)"]
    _00,
    #[doc = "TRGMUX pretrigger"]
    _01,
    #[doc = "Software pretrigger"]
    _10,
}
impl ADC1PRETRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC1PRETRGSELW::_00 => 0,
            ADC1PRETRGSELW::_01 => 1,
            ADC1PRETRGSELW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1PRETRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1PRETRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1PRETRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB pretrigger (default)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADC1PRETRGSELW::_00)
    }
    #[doc = "TRGMUX pretrigger"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADC1PRETRGSELW::_01)
    }
    #[doc = "Software pretrigger"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADC1PRETRGSELW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - ADC0 trigger source select"]
    #[inline]
    pub fn adc0trgsel(&self) -> ADC0TRGSELR {
        ADC0TRGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - ADC0 software pretrigger sources"]
    #[inline]
    pub fn adc0swpretrg(&self) -> ADC0SWPRETRGR {
        ADC0SWPRETRGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ADC0 pretrigger source select"]
    #[inline]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSELR {
        ADC0PRETRGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - ADC1 trigger source select"]
    #[inline]
    pub fn adc1trgsel(&self) -> ADC1TRGSELR {
        ADC1TRGSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 9:11 - ADC1 software pretrigger sources"]
    #[inline]
    pub fn adc1swpretrg(&self) -> ADC1SWPRETRGR {
        ADC1SWPRETRGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - ADC1 pretrigger source select"]
    #[inline]
    pub fn adc1pretrgsel(&self) -> ADC1PRETRGSELR {
        ADC1PRETRGSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - ADC0 trigger source select"]
    #[inline]
    pub fn adc0trgsel(&mut self) -> _ADC0TRGSELW {
        _ADC0TRGSELW { w: self }
    }
    #[doc = "Bits 1:3 - ADC0 software pretrigger sources"]
    #[inline]
    pub fn adc0swpretrg(&mut self) -> _ADC0SWPRETRGW {
        _ADC0SWPRETRGW { w: self }
    }
    #[doc = "Bits 4:5 - ADC0 pretrigger source select"]
    #[inline]
    pub fn adc0pretrgsel(&mut self) -> _ADC0PRETRGSELW {
        _ADC0PRETRGSELW { w: self }
    }
    #[doc = "Bit 8 - ADC1 trigger source select"]
    #[inline]
    pub fn adc1trgsel(&mut self) -> _ADC1TRGSELW {
        _ADC1TRGSELW { w: self }
    }
    #[doc = "Bits 9:11 - ADC1 software pretrigger sources"]
    #[inline]
    pub fn adc1swpretrg(&mut self) -> _ADC1SWPRETRGW {
        _ADC1SWPRETRGW { w: self }
    }
    #[doc = "Bits 12:13 - ADC1 pretrigger source select"]
    #[inline]
    pub fn adc1pretrgsel(&mut self) -> _ADC1PRETRGSELW {
        _ADC1PRETRGSELW { w: self }
    }
}
