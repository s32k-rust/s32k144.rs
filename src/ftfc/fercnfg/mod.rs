#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FERCNFG {
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
#[doc = "Possible values of the field `DFDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIER {
    #[doc = "Double bit fault detect interrupt disabled"]
    _0,
    #[doc = "Double bit fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT[DFDIF] flag is set."]
    _1,
}
impl DFDIER {
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
            DFDIER::_0 => false,
            DFDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DFDIER {
        match value {
            false => DFDIER::_0,
            true => DFDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DFDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DFDIER::_1
    }
}
#[doc = "Possible values of the field `FDFD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDFDR {
    #[doc = "FERSTAT[DFDIF] sets only if a double bit fault is detected during read access from the platform flash controller"]
    _0,
    #[doc = "FERSTAT[DFDIF] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    _1,
}
impl FDFDR {
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
            FDFDR::_0 => false,
            FDFDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDFDR {
        match value {
            false => FDFDR::_0,
            true => FDFDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FDFDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FDFDR::_1
    }
}
#[doc = "Values that can be written to the field `DFDIE`"]
pub enum DFDIEW {
    #[doc = "Double bit fault detect interrupt disabled"]
    _0,
    #[doc = "Double bit fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT[DFDIF] flag is set."]
    _1,
}
impl DFDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DFDIEW::_0 => false,
            DFDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DFDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _DFDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DFDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Double bit fault detect interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIEW::_0)
    }
    #[doc = "Double bit fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT[DFDIF] flag is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIEW::_1)
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
#[doc = "Values that can be written to the field `FDFD`"]
pub enum FDFDW {
    #[doc = "FERSTAT[DFDIF] sets only if a double bit fault is detected during read access from the platform flash controller"]
    _0,
    #[doc = "FERSTAT[DFDIF] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    _1,
}
impl FDFDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FDFDW::_0 => false,
            FDFDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDFDW<'a> {
    w: &'a mut W,
}
impl<'a> _FDFDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDFDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FERSTAT[DFDIF] sets only if a double bit fault is detected during read access from the platform flash controller"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDFDW::_0)
    }
    #[doc = "FERSTAT[DFDIF] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDFDW::_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline]
    pub fn dfdie(&self) -> DFDIER {
        DFDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline]
    pub fn fdfd(&self) -> FDFDR {
        FDFDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline]
    pub fn dfdie(&mut self) -> _DFDIEW {
        _DFDIEW { w: self }
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline]
    pub fn fdfd(&mut self) -> _FDFDW {
        _FDFDW { w: self }
    }
}
