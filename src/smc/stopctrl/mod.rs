#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STOPCTRL {
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
#[doc = "Possible values of the field `STOPO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPOR {
    #[doc = "STOP1 - Stop with both system and bus clocks disabled"]
    _01,
    #[doc = "STOP2 - Stop with system clock disabled and bus clock enabled"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STOPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPOR::_01 => 1,
            STOPOR::_10 => 2,
            STOPOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPOR {
        match value {
            1 => STOPOR::_01,
            2 => STOPOR::_10,
            i => STOPOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == STOPOR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == STOPOR::_10
    }
}
#[doc = "Values that can be written to the field `STOPO`"]
pub enum STOPOW {
    #[doc = "STOP1 - Stop with both system and bus clocks disabled"]
    _01,
    #[doc = "STOP2 - Stop with system clock disabled and bus clock enabled"]
    _10,
}
impl STOPOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPOW::_01 => 1,
            STOPOW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPOW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "STOP1 - Stop with both system and bus clocks disabled"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(STOPOW::_01)
    }
    #[doc = "STOP2 - Stop with system clock disabled and bus clock enabled"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(STOPOW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline]
    pub fn stopo(&self) -> STOPOR {
        STOPOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 6:7 - Stop Option"]
    #[inline]
    pub fn stopo(&mut self) -> _STOPOW {
        _STOPOW { w: self }
    }
}
