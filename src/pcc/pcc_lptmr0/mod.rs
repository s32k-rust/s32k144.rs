#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCC_LPTMR0 {
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
#[doc = "Possible values of the field `PCD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCDR {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 3."]
    _010,
    #[doc = "Divide by 4."]
    _011,
    #[doc = "Divide by 5."]
    _100,
    #[doc = "Divide by 6."]
    _101,
    #[doc = "Divide by 7."]
    _110,
    #[doc = "Divide by 8."]
    _111,
}
impl PCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCDR::_000 => 0,
            PCDR::_001 => 1,
            PCDR::_010 => 2,
            PCDR::_011 => 3,
            PCDR::_100 => 4,
            PCDR::_101 => 5,
            PCDR::_110 => 6,
            PCDR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCDR {
        match value {
            0 => PCDR::_000,
            1 => PCDR::_001,
            2 => PCDR::_010,
            3 => PCDR::_011,
            4 => PCDR::_100,
            5 => PCDR::_101,
            6 => PCDR::_110,
            7 => PCDR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PCDR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PCDR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PCDR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PCDR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PCDR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PCDR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PCDR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PCDR::_111
    }
}
#[doc = "Possible values of the field `FRAC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRACR {
    #[doc = "Fractional value is 0."]
    _0,
    #[doc = "Fractional value is 1."]
    _1,
}
impl FRACR {
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
            FRACR::_0 => false,
            FRACR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRACR {
        match value {
            false => FRACR::_0,
            true => FRACR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FRACR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FRACR::_1
    }
}
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "Clock is off."]
    _000,
    #[doc = "Clock option 1"]
    _001,
    #[doc = "Clock option 2"]
    _010,
    #[doc = "Clock option 3"]
    _011,
    #[doc = "Clock option 4"]
    _100,
    #[doc = "Clock option 5"]
    _101,
    #[doc = "Clock option 6"]
    _110,
    #[doc = "Clock option 7"]
    _111,
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSR::_000 => 0,
            PCSR::_001 => 1,
            PCSR::_010 => 2,
            PCSR::_011 => 3,
            PCSR::_100 => 4,
            PCSR::_101 => 5,
            PCSR::_110 => 6,
            PCSR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSR {
        match value {
            0 => PCSR::_000,
            1 => PCSR::_001,
            2 => PCSR::_010,
            3 => PCSR::_011,
            4 => PCSR::_100,
            5 => PCSR::_101,
            6 => PCSR::_110,
            7 => PCSR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PCSR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PCSR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PCSR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PCSR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PCSR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PCSR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PCSR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PCSR::_111
    }
}
#[doc = "Possible values of the field `CGC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled. The current clock selection and divider options are locked."]
    _1,
}
impl CGCR {
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
            CGCR::_0 => false,
            CGCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCR {
        match value {
            false => CGCR::_0,
            true => CGCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCR::_1
    }
}
#[doc = "Possible values of the field `PR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRR {
    #[doc = "Peripheral is not present."]
    _0,
    #[doc = "Peripheral is present."]
    _1,
}
impl PRR {
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
            PRR::_0 => false,
            PRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRR {
        match value {
            false => PRR::_0,
            true => PRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRR::_1
    }
}
#[doc = "Values that can be written to the field `PCD`"]
pub enum PCDW {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 3."]
    _010,
    #[doc = "Divide by 4."]
    _011,
    #[doc = "Divide by 5."]
    _100,
    #[doc = "Divide by 6."]
    _101,
    #[doc = "Divide by 7."]
    _110,
    #[doc = "Divide by 8."]
    _111,
}
impl PCDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCDW::_000 => 0,
            PCDW::_001 => 1,
            PCDW::_010 => 2,
            PCDW::_011 => 3,
            PCDW::_100 => 4,
            PCDW::_101 => 5,
            PCDW::_110 => 6,
            PCDW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCDW<'a> {
    w: &'a mut W,
}
impl<'a> _PCDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide by 1."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCDW::_000)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCDW::_001)
    }
    #[doc = "Divide by 3."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCDW::_010)
    }
    #[doc = "Divide by 4."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCDW::_011)
    }
    #[doc = "Divide by 5."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCDW::_100)
    }
    #[doc = "Divide by 6."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCDW::_101)
    }
    #[doc = "Divide by 7."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCDW::_110)
    }
    #[doc = "Divide by 8."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PCDW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRAC`"]
pub enum FRACW {
    #[doc = "Fractional value is 0."]
    _0,
    #[doc = "Fractional value is 1."]
    _1,
}
impl FRACW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRACW::_0 => false,
            FRACW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRACW<'a> {
    w: &'a mut W,
}
impl<'a> _FRACW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRACW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fractional value is 0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRACW::_0)
    }
    #[doc = "Fractional value is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRACW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS`"]
pub enum PCSW {
    #[doc = "Clock is off."]
    _000,
    #[doc = "Clock option 1"]
    _001,
    #[doc = "Clock option 2"]
    _010,
    #[doc = "Clock option 3"]
    _011,
    #[doc = "Clock option 4"]
    _100,
    #[doc = "Clock option 5"]
    _101,
    #[doc = "Clock option 6"]
    _110,
    #[doc = "Clock option 7"]
    _111,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::_000 => 0,
            PCSW::_001 => 1,
            PCSW::_010 => 2,
            PCSW::_011 => 3,
            PCSW::_100 => 4,
            PCSW::_101 => 5,
            PCSW::_110 => 6,
            PCSW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock is off."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCSW::_000)
    }
    #[doc = "Clock option 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCSW::_001)
    }
    #[doc = "Clock option 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCSW::_010)
    }
    #[doc = "Clock option 3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCSW::_011)
    }
    #[doc = "Clock option 4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCSW::_100)
    }
    #[doc = "Clock option 5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCSW::_101)
    }
    #[doc = "Clock option 6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCSW::_110)
    }
    #[doc = "Clock option 7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PCSW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CGC`"]
pub enum CGCW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled. The current clock selection and divider options are locked."]
    _1,
}
impl CGCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCW::_0 => false,
            CGCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCW::_0)
    }
    #[doc = "Clock enabled. The current clock selection and divider options are locked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCW::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - Peripheral Clock Divider Select"]
    #[inline]
    pub fn pcd(&self) -> PCDR {
        PCDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Peripheral Clock Divider Fraction"]
    #[inline]
    pub fn frac(&self) -> FRACR {
        FRACR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline]
    pub fn cgc(&self) -> CGCR {
        CGCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Present"]
    #[inline]
    pub fn pr(&self) -> PRR {
        PRR::_from({
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
        W { bits: 2147483648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Peripheral Clock Divider Select"]
    #[inline]
    pub fn pcd(&mut self) -> _PCDW {
        _PCDW { w: self }
    }
    #[doc = "Bit 3 - Peripheral Clock Divider Fraction"]
    #[inline]
    pub fn frac(&mut self) -> _FRACW {
        _FRACW { w: self }
    }
    #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline]
    pub fn cgc(&mut self) -> _CGCW {
        _CGCW { w: self }
    }
}
