#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFER {
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
#[doc = "Possible values of the field `DFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFER {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl DFER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            DFER::_0 => 0,
            DFER::_1 => 1,
            DFER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> DFER {
        match value {
            0 => DFER::_0,
            1 => DFER::_1,
            i => DFER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFER::_1
    }
}
#[doc = "Values that can be written to the field `DFE`"]
pub enum DFEW {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0,
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1,
}
impl DFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            DFEW::_0 => 0,
            DFEW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFEW<'a> {
    w: &'a mut W,
}
impl<'a> _DFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFEW::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFEW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline]
    pub fn dfe(&self) -> DFER {
        DFER::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline]
    pub fn dfe(&mut self) -> _DFEW {
        _DFEW { w: self }
    }
}
