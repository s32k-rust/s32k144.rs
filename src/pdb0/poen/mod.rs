#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POEN {
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
#[doc = "Possible values of the field `POEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POENR {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl POENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            POENR::_0 => 0,
            POENR::_1 => 1,
            POENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> POENR {
        match value {
            0 => POENR::_0,
            1 => POENR::_1,
            i => POENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == POENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == POENR::_1
    }
}
#[doc = "Values that can be written to the field `POEN`"]
pub enum POENW {
    #[doc = "PDB Pulse-Out disabled"]
    _0,
    #[doc = "PDB Pulse-Out enabled"]
    _1,
}
impl POENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            POENW::_0 => 0,
            POENW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _POENW<'a> {
    w: &'a mut W,
}
impl<'a> _POENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: POENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB Pulse-Out disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(POENW::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(POENW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen(&self) -> POENR {
        POENR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline]
    pub fn poen(&mut self) -> _POENW {
        _POENW { w: self }
    }
}
