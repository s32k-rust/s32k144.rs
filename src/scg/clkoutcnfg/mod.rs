#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLKOUTCNFG {
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
#[doc = "Possible values of the field `CLKOUTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSELR {
    #[doc = "SCG SLOW Clock"]
    _0000,
    #[doc = "System OSC (SOSC_CLK)"]
    _0001,
    #[doc = "Slow IRC (SIRC_CLK)"]
    _0010,
    #[doc = "Fast IRC (FIRC_CLK)"]
    _0011,
    #[doc = "System PLL (SPLL_CLK)"]
    _0110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CLKOUTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKOUTSELR::_0000 => 0,
            CLKOUTSELR::_0001 => 1,
            CLKOUTSELR::_0010 => 2,
            CLKOUTSELR::_0011 => 3,
            CLKOUTSELR::_0110 => 6,
            CLKOUTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKOUTSELR {
        match value {
            0 => CLKOUTSELR::_0000,
            1 => CLKOUTSELR::_0001,
            2 => CLKOUTSELR::_0010,
            3 => CLKOUTSELR::_0011,
            6 => CLKOUTSELR::_0110,
            i => CLKOUTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == CLKOUTSELR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == CLKOUTSELR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == CLKOUTSELR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == CLKOUTSELR::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == CLKOUTSELR::_0110
    }
}
#[doc = "Values that can be written to the field `CLKOUTSEL`"]
pub enum CLKOUTSELW {
    #[doc = "SCG SLOW Clock"]
    _0000,
    #[doc = "System OSC (SOSC_CLK)"]
    _0001,
    #[doc = "Slow IRC (SIRC_CLK)"]
    _0010,
    #[doc = "Fast IRC (FIRC_CLK)"]
    _0011,
    #[doc = "System PLL (SPLL_CLK)"]
    _0110,
}
impl CLKOUTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSELW::_0000 => 0,
            CLKOUTSELW::_0001 => 1,
            CLKOUTSELW::_0010 => 2,
            CLKOUTSELW::_0011 => 3,
            CLKOUTSELW::_0110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKOUTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOUTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKOUTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SCG SLOW Clock"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0000)
    }
    #[doc = "System OSC (SOSC_CLK)"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0001)
    }
    #[doc = "Slow IRC (SIRC_CLK)"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0010)
    }
    #[doc = "Fast IRC (FIRC_CLK)"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0011)
    }
    #[doc = "System PLL (SPLL_CLK)"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKOUTSELW::_0110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline]
    pub fn clkoutsel(&self) -> CLKOUTSELR {
        CLKOUTSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50331648 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline]
    pub fn clkoutsel(&mut self) -> _CLKOUTSELW {
        _CLKOUTSELW { w: self }
    }
}
