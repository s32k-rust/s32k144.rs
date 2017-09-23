#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIRCDIV {
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
#[doc = "Possible values of the field `FIRCDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCDIV1R {
    #[doc = "Output disabled"] _000,
    #[doc = "Divide by 1"] _001,
    #[doc = "Divide by 2"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 8"] _100,
    #[doc = "Divide by 16"] _101,
    #[doc = "Divide by 32"] _110,
    #[doc = "Divide by 64"] _111,
}
impl FIRCDIV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIRCDIV1R::_000 => 0,
            FIRCDIV1R::_001 => 1,
            FIRCDIV1R::_010 => 2,
            FIRCDIV1R::_011 => 3,
            FIRCDIV1R::_100 => 4,
            FIRCDIV1R::_101 => 5,
            FIRCDIV1R::_110 => 6,
            FIRCDIV1R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIRCDIV1R {
        match value {
            0 => FIRCDIV1R::_000,
            1 => FIRCDIV1R::_001,
            2 => FIRCDIV1R::_010,
            3 => FIRCDIV1R::_011,
            4 => FIRCDIV1R::_100,
            5 => FIRCDIV1R::_101,
            6 => FIRCDIV1R::_110,
            7 => FIRCDIV1R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FIRCDIV1R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FIRCDIV1R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FIRCDIV1R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FIRCDIV1R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FIRCDIV1R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FIRCDIV1R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FIRCDIV1R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FIRCDIV1R::_111
    }
}
#[doc = "Possible values of the field `FIRCDIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRCDIV2R {
    #[doc = "Output disabled"] _000,
    #[doc = "Divide by 1"] _001,
    #[doc = "Divide by 2"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 8"] _100,
    #[doc = "Divide by 16"] _101,
    #[doc = "Divide by 32"] _110,
    #[doc = "Divide by 64"] _111,
}
impl FIRCDIV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIRCDIV2R::_000 => 0,
            FIRCDIV2R::_001 => 1,
            FIRCDIV2R::_010 => 2,
            FIRCDIV2R::_011 => 3,
            FIRCDIV2R::_100 => 4,
            FIRCDIV2R::_101 => 5,
            FIRCDIV2R::_110 => 6,
            FIRCDIV2R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIRCDIV2R {
        match value {
            0 => FIRCDIV2R::_000,
            1 => FIRCDIV2R::_001,
            2 => FIRCDIV2R::_010,
            3 => FIRCDIV2R::_011,
            4 => FIRCDIV2R::_100,
            5 => FIRCDIV2R::_101,
            6 => FIRCDIV2R::_110,
            7 => FIRCDIV2R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == FIRCDIV2R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == FIRCDIV2R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == FIRCDIV2R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == FIRCDIV2R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == FIRCDIV2R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == FIRCDIV2R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == FIRCDIV2R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == FIRCDIV2R::_111
    }
}
#[doc = "Values that can be written to the field `FIRCDIV1`"]
pub enum FIRCDIV1W {
    #[doc = "Output disabled"] _000,
    #[doc = "Divide by 1"] _001,
    #[doc = "Divide by 2"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 8"] _100,
    #[doc = "Divide by 16"] _101,
    #[doc = "Divide by 32"] _110,
    #[doc = "Divide by 64"] _111,
}
impl FIRCDIV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIRCDIV1W::_000 => 0,
            FIRCDIV1W::_001 => 1,
            FIRCDIV1W::_010 => 2,
            FIRCDIV1W::_011 => 3,
            FIRCDIV1W::_100 => 4,
            FIRCDIV1W::_101 => 5,
            FIRCDIV1W::_110 => 6,
            FIRCDIV1W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIRCDIV1W<'a> {
    w: &'a mut W,
}
impl<'a> _FIRCDIV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRCDIV1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FIRCDIV1W::_111)
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
#[doc = "Values that can be written to the field `FIRCDIV2`"]
pub enum FIRCDIV2W {
    #[doc = "Output disabled"] _000,
    #[doc = "Divide by 1"] _001,
    #[doc = "Divide by 2"] _010,
    #[doc = "Divide by 4"] _011,
    #[doc = "Divide by 8"] _100,
    #[doc = "Divide by 16"] _101,
    #[doc = "Divide by 32"] _110,
    #[doc = "Divide by 64"] _111,
}
impl FIRCDIV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIRCDIV2W::_000 => 0,
            FIRCDIV2W::_001 => 1,
            FIRCDIV2W::_010 => 2,
            FIRCDIV2W::_011 => 3,
            FIRCDIV2W::_100 => 4,
            FIRCDIV2W::_101 => 5,
            FIRCDIV2W::_110 => 6,
            FIRCDIV2W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIRCDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _FIRCDIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRCDIV2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(FIRCDIV2W::_111)
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
    #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
    #[inline]
    pub fn fircdiv1(&self) -> FIRCDIV1R {
        FIRCDIV1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
    #[inline]
    pub fn fircdiv2(&self) -> FIRCDIV2R {
        FIRCDIV2R::_from({
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
    #[doc = "Bits 0:2 - Fast IRC Clock Divide 1"]
    #[inline]
    pub fn fircdiv1(&mut self) -> _FIRCDIV1W {
        _FIRCDIV1W { w: self }
    }
    #[doc = "Bits 8:10 - Fast IRC Clock Divide 2"]
    #[inline]
    pub fn fircdiv2(&mut self) -> _FIRCDIV2W {
        _FIRCDIV2W { w: self }
    }
}
