#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIRCCFG {
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
#[doc = "Possible values of the field `RANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGER {
    #[doc = "Fast IRC is trimmed to 48 MHz"] _00,
    #[doc = "Fast IRC is trimmed to 52 MHz"] _01,
    #[doc = "Fast IRC is trimmed to 56 MHz"] _10,
    #[doc = "Fast IRC is trimmed to 60 MHz"] _11,
}
impl RANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANGER::_00 => 0,
            RANGER::_01 => 1,
            RANGER::_10 => 2,
            RANGER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANGER {
        match value {
            0 => RANGER::_00,
            1 => RANGER::_01,
            2 => RANGER::_10,
            3 => RANGER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RANGER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RANGER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RANGER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RANGER::_11
    }
}
#[doc = "Values that can be written to the field `RANGE`"]
pub enum RANGEW {
    #[doc = "Fast IRC is trimmed to 48 MHz"] _00,
    #[doc = "Fast IRC is trimmed to 52 MHz"] _01,
    #[doc = "Fast IRC is trimmed to 56 MHz"] _10,
    #[doc = "Fast IRC is trimmed to 60 MHz"] _11,
}
impl RANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RANGEW::_00 => 0,
            RANGEW::_01 => 1,
            RANGEW::_10 => 2,
            RANGEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _RANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RANGEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Fast IRC is trimmed to 48 MHz"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGEW::_00)
    }
    #[doc = "Fast IRC is trimmed to 52 MHz"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGEW::_01)
    }
    #[doc = "Fast IRC is trimmed to 56 MHz"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RANGEW::_10)
    }
    #[doc = "Fast IRC is trimmed to 60 MHz"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RANGEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline]
    pub fn range(&self) -> RANGER {
        RANGER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Frequency Range"]
    #[inline]
    pub fn range(&mut self) -> _RANGEW {
        _RANGEW { w: self }
    }
}
