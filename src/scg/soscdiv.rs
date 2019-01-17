#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOSCDIV {
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
#[doc = "Possible values of the field `SOSCDIV1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCDIV1R {
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
impl SOSCDIV1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOSCDIV1R::_000 => 0,
            SOSCDIV1R::_001 => 1,
            SOSCDIV1R::_010 => 2,
            SOSCDIV1R::_011 => 3,
            SOSCDIV1R::_100 => 4,
            SOSCDIV1R::_101 => 5,
            SOSCDIV1R::_110 => 6,
            SOSCDIV1R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOSCDIV1R {
        match value {
            0 => SOSCDIV1R::_000,
            1 => SOSCDIV1R::_001,
            2 => SOSCDIV1R::_010,
            3 => SOSCDIV1R::_011,
            4 => SOSCDIV1R::_100,
            5 => SOSCDIV1R::_101,
            6 => SOSCDIV1R::_110,
            7 => SOSCDIV1R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SOSCDIV1R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SOSCDIV1R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SOSCDIV1R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == SOSCDIV1R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SOSCDIV1R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SOSCDIV1R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == SOSCDIV1R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == SOSCDIV1R::_111
    }
}
#[doc = "Possible values of the field `SOSCDIV2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCDIV2R {
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
impl SOSCDIV2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SOSCDIV2R::_000 => 0,
            SOSCDIV2R::_001 => 1,
            SOSCDIV2R::_010 => 2,
            SOSCDIV2R::_011 => 3,
            SOSCDIV2R::_100 => 4,
            SOSCDIV2R::_101 => 5,
            SOSCDIV2R::_110 => 6,
            SOSCDIV2R::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SOSCDIV2R {
        match value {
            0 => SOSCDIV2R::_000,
            1 => SOSCDIV2R::_001,
            2 => SOSCDIV2R::_010,
            3 => SOSCDIV2R::_011,
            4 => SOSCDIV2R::_100,
            5 => SOSCDIV2R::_101,
            6 => SOSCDIV2R::_110,
            7 => SOSCDIV2R::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == SOSCDIV2R::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == SOSCDIV2R::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == SOSCDIV2R::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == SOSCDIV2R::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == SOSCDIV2R::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == SOSCDIV2R::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == SOSCDIV2R::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == SOSCDIV2R::_111
    }
}
#[doc = "Values that can be written to the field `SOSCDIV1`"]
pub enum SOSCDIV1W {
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
impl SOSCDIV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOSCDIV1W::_000 => 0,
            SOSCDIV1W::_001 => 1,
            SOSCDIV1W::_010 => 2,
            SOSCDIV1W::_011 => 3,
            SOSCDIV1W::_100 => 4,
            SOSCDIV1W::_101 => 5,
            SOSCDIV1W::_110 => 6,
            SOSCDIV1W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCDIV1W<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCDIV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCDIV1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(SOSCDIV1W::_111)
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
#[doc = "Values that can be written to the field `SOSCDIV2`"]
pub enum SOSCDIV2W {
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
impl SOSCDIV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SOSCDIV2W::_000 => 0,
            SOSCDIV2W::_001 => 1,
            SOSCDIV2W::_010 => 2,
            SOSCDIV2W::_011 => 3,
            SOSCDIV2W::_100 => 4,
            SOSCDIV2W::_101 => 5,
            SOSCDIV2W::_110 => 6,
            SOSCDIV2W::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOSCDIV2W<'a> {
    w: &'a mut W,
}
impl<'a> _SOSCDIV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOSCDIV2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Output disabled"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_000)
    }
    #[doc = "Divide by 1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_001)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_010)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_011)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_100)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_101)
    }
    #[doc = "Divide by 32"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_110)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(SOSCDIV2W::_111)
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
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline]
    pub fn soscdiv1(&self) -> SOSCDIV1R {
        SOSCDIV1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline]
    pub fn soscdiv2(&self) -> SOSCDIV2R {
        SOSCDIV2R::_from({
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
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline]
    pub fn soscdiv1(&mut self) -> _SOSCDIV1W {
        _SOSCDIV1W { w: self }
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline]
    pub fn soscdiv2(&mut self) -> _SOSCDIV2W {
        _SOSCDIV2W { w: self }
    }
}
