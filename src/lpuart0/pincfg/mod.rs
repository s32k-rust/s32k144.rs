#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PINCFG {
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
#[doc = "Possible values of the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELR {
    #[doc = "Input trigger is disabled."] _00,
    #[doc = "Input trigger is used instead of RXD pin input."] _01,
    #[doc = "Input trigger is used instead of CTS_B pin input."] _10,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    _11,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSELR::_00 => 0,
            TRGSELR::_01 => 1,
            TRGSELR::_10 => 2,
            TRGSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSELR {
        match value {
            0 => TRGSELR::_00,
            1 => TRGSELR::_01,
            2 => TRGSELR::_10,
            3 => TRGSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TRGSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TRGSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TRGSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TRGSELR::_11
    }
}
#[doc = "Values that can be written to the field `TRGSEL`"]
pub enum TRGSELW {
    #[doc = "Input trigger is disabled."] _00,
    #[doc = "Input trigger is used instead of RXD pin input."] _01,
    #[doc = "Input trigger is used instead of CTS_B pin input."] _10,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    _11,
}
impl TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSELW::_00 => 0,
            TRGSELW::_01 => 1,
            TRGSELW::_10 => 2,
            TRGSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input trigger is disabled."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TRGSELW::_00)
    }
    #[doc = "Input trigger is used instead of RXD pin input."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TRGSELW::_01)
    }
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TRGSELW::_10)
    }
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is ANDed with the input trigger."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TRGSELW::_11)
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
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        TRGSELR::_from({
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
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
}
