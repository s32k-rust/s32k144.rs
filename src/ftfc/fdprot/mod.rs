#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FDPROT {
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
#[doc = "Possible values of the field `DPROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPROTR {
    #[doc = "Data Flash region is protected"]
    _00000000,
    #[doc = "Data Flash region is not protected"]
    _00000001,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DPROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DPROTR::_00000000 => 0,
            DPROTR::_00000001 => 1,
            DPROTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DPROTR {
        match value {
            0 => DPROTR::_00000000,
            1 => DPROTR::_00000001,
            i => DPROTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline]
    pub fn is_00000000(&self) -> bool {
        *self == DPROTR::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline]
    pub fn is_00000001(&self) -> bool {
        *self == DPROTR::_00000001
    }
}
#[doc = "Values that can be written to the field `DPROT`"]
pub enum DPROTW {
    #[doc = "Data Flash region is protected"]
    _00000000,
    #[doc = "Data Flash region is not protected"]
    _00000001,
}
impl DPROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DPROTW::_00000000 => 0,
            DPROTW::_00000001 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPROTW<'a> {
    w: &'a mut W,
}
impl<'a> _DPROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPROTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Data Flash region is protected"]
    #[inline]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(DPROTW::_00000000)
    }
    #[doc = "Data Flash region is not protected"]
    #[inline]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(DPROTW::_00000001)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline]
    pub fn dprot(&self) -> DPROTR {
        DPROTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline]
    pub fn dprot(&mut self) -> _DPROTW {
        _DPROTW { w: self }
    }
}
