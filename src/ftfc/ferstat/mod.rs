#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FERSTAT {
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
#[doc = "Possible values of the field `DFDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIFR {
    #[doc = "Double bit fault not detected during a valid flash read access from the platform flash controller"]
    _0,
    #[doc = "Double bit fault detected (or FERCNFG[FDFD] is set) during a valid flash read access from the platform flash controller"]
    _1,
}
impl DFDIFR {
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
            DFDIFR::_0 => false,
            DFDIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFDIFR {
        match value {
            false => DFDIFR::_0,
            true => DFDIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFDIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFDIFR::_1
    }
}
#[doc = "Values that can be written to the field `DFDIF`"]
pub enum DFDIFW {
    #[doc = "Double bit fault not detected during a valid flash read access from the platform flash controller"]
    _0,
    #[doc = "Double bit fault detected (or FERCNFG[FDFD] is set) during a valid flash read access from the platform flash controller"]
    _1,
}
impl DFDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFDIFW::_0 => false,
            DFDIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DFDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Double bit fault not detected during a valid flash read access from the platform flash controller"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIFW::_0)
    }
    #[doc = "Double bit fault detected (or FERCNFG[FDFD] is set) during a valid flash read access from the platform flash controller"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIFW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline]
    pub fn dfdif(&self) -> DFDIFR {
        DFDIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Flag"]
    #[inline]
    pub fn dfdif(&mut self) -> _DFDIFW {
        _DFDIFW { w: self }
    }
}
