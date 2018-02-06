#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CDNE {
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
}
#[doc = r" Proxy"]
pub struct _CDNEW<'a> {
    w: &'a mut W,
}
impl<'a> _CDNEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CADN`"]
pub enum CADNW {
    #[doc = "Clears only the TCDn_CSR[DONE] bit specified in the CDNE field"]
    _0,
    #[doc = "Clears all bits in TCDn_CSR[DONE]"]
    _1,
}
impl CADNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CADNW::_0 => false,
            CADNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CADNW<'a> {
    w: &'a mut W,
}
impl<'a> _CADNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CADNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears only the TCDn_CSR[DONE] bit specified in the CDNE field"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CADNW::_0)
    }
    #[doc = "Clears all bits in TCDn_CSR[DONE]"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CADNW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NOP`"]
pub enum NOPW {
    #[doc = "Normal operation"]
    _0,
    #[doc = "No operation, ignore the other bits in this register"]
    _1,
}
impl NOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOPW::_0 => false,
            NOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOPW<'a> {
    w: &'a mut W,
}
impl<'a> _NOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOPW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOPW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
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
    #[doc = "Bits 0:3 - Clear DONE Bit"]
    #[inline]
    pub fn cdne(&mut self) -> _CDNEW {
        _CDNEW { w: self }
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline]
    pub fn cadn(&mut self) -> _CADNW {
        _CADNW { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline]
    pub fn nop(&mut self) -> _NOPW {
        _NOPW { w: self }
    }
}
