#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIRCDIV {
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
#[doc = "Possible values of the field `SIRCDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCDIV1R {
    #[doc = "Output disabled"]
    _000,
    #[doc = "Divide by 1"]
    _001,
    #[doc = "Divide by 2"]
    _010,
    #[doc = "Divide by 4"]
    _011,
    #[doc = "Divide by 8"]
    _100,
    #[doc = "Divide by 16"]
    _101,
    #[doc = "Divide by 32"]
    _110,
    #[doc = "Divide by 64"]
    _111,
}
impl SIRCDIV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIRCDIV1R::_000 => 0,
            SIRCDIV1R::_001 => 1,
            SIRCDIV1R::_010 => 2,
            SIRCDIV1R::_011 => 3,
            SIRCDIV1R::_100 => 4,
            SIRCDIV1R::_101 => 5,
            SIRCDIV1R::_110 => 6,
            SIRCDIV1R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIRCDIV1R {
        match value {
            0 => SIRCDIV1R::_000,
            1 => SIRCDIV1R::_001,
            2 => SIRCDIV1R::_010,
            3 => SIRCDIV1R::_011,
            4 => SIRCDIV1R::_100,
            5 => SIRCDIV1R::_101,
            6 => SIRCDIV1R::_110,
            7 => SIRCDIV1R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SIRCDIV1R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SIRCDIV1R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SIRCDIV1R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == SIRCDIV1R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SIRCDIV1R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SIRCDIV1R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == SIRCDIV1R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == SIRCDIV1R::_111
    }
}
#[doc = "Possible values of the field `SIRCDIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIRCDIV2R {
    #[doc = "Output disabled"]
    _000,
    #[doc = "Divide by 1"]
    _001,
    #[doc = "Divide by 2"]
    _010,
    #[doc = "Divide by 4"]
    _011,
    #[doc = "Divide by 8"]
    _100,
    #[doc = "Divide by 16"]
    _101,
    #[doc = "Divide by 32"]
    _110,
    #[doc = "Divide by 64"]
    _111,
}
impl SIRCDIV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIRCDIV2R::_000 => 0,
            SIRCDIV2R::_001 => 1,
            SIRCDIV2R::_010 => 2,
            SIRCDIV2R::_011 => 3,
            SIRCDIV2R::_100 => 4,
            SIRCDIV2R::_101 => 5,
            SIRCDIV2R::_110 => 6,
            SIRCDIV2R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIRCDIV2R {
        match value {
            0 => SIRCDIV2R::_000,
            1 => SIRCDIV2R::_001,
            2 => SIRCDIV2R::_010,
            3 => SIRCDIV2R::_011,
            4 => SIRCDIV2R::_100,
            5 => SIRCDIV2R::_101,
            6 => SIRCDIV2R::_110,
            7 => SIRCDIV2R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SIRCDIV2R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SIRCDIV2R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SIRCDIV2R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == SIRCDIV2R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SIRCDIV2R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SIRCDIV2R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == SIRCDIV2R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == SIRCDIV2R::_111
    }
}
#[doc = "Values that can be written to the field `SIRCDIV1`"]
pub enum SIRCDIV1W {
    #[doc = "Output disabled"]
    _000,
    #[doc = "Divide by 1"]
    _001,
    #[doc = "Divide by 2"]
    _010,
    #[doc = "Divide by 4"]
    _011,
    #[doc = "Divide by 8"]
    _100,
    #[doc = "Divide by 16"]
    _101,
    #[doc = "Divide by 32"]
    _110,
    #[doc = "Divide by 64"]
    _111,
}
impl SIRCDIV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIRCDIV1W::_000 => 0,
            SIRCDIV1W::_001 => 1,
            SIRCDIV1W::_010 => 2,
            SIRCDIV1W::_011 => 3,
            SIRCDIV1W::_100 => 4,
            SIRCDIV1W::_101 => 5,
            SIRCDIV1W::_110 => 6,
            SIRCDIV1W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIRCDIV1W<'a> {
    w: &'a mut W,
}
impl<'a> _SIRCDIV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIRCDIV1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV1W::_111)
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
#[doc = "Values that can be written to the field `SIRCDIV2`"]
pub enum SIRCDIV2W {
    #[doc = "Output disabled"]
    _000,
    #[doc = "Divide by 1"]
    _001,
    #[doc = "Divide by 2"]
    _010,
    #[doc = "Divide by 4"]
    _011,
    #[doc = "Divide by 8"]
    _100,
    #[doc = "Divide by 16"]
    _101,
    #[doc = "Divide by 32"]
    _110,
    #[doc = "Divide by 64"]
    _111,
}
impl SIRCDIV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIRCDIV2W::_000 => 0,
            SIRCDIV2W::_001 => 1,
            SIRCDIV2W::_010 => 2,
            SIRCDIV2W::_011 => 3,
            SIRCDIV2W::_100 => 4,
            SIRCDIV2W::_101 => 5,
            SIRCDIV2W::_110 => 6,
            SIRCDIV2W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIRCDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _SIRCDIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIRCDIV2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV2W::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline]
    pub fn sircdiv1(&self) -> SIRCDIV1R {
        SIRCDIV1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline]
    pub fn sircdiv2(&self) -> SIRCDIV2R {
        SIRCDIV2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline]
    pub fn sircdiv1(&mut self) -> _SIRCDIV1W {
        _SIRCDIV1W { w: self }
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline]
    pub fn sircdiv2(&mut self) -> _SIRCDIV2W {
        _SIRCDIV2W { w: self }
    }
}
