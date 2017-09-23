#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOSCCFG {
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
#[doc = "Possible values of the field `EREFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EREFSR {
    #[doc = "External reference clock selected"] _0,
    #[doc = "Internal crystal oscillator of OSC selected."] _1,
}
impl EREFSR {
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
            EREFSR::_0 => false,
            EREFSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EREFSR {
        match value {
            false => EREFSR::_0,
            true => EREFSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EREFSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EREFSR::_1
    }
}
#[doc = "Possible values of the field `HGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HGOR {
    #[doc = "Configure crystal oscillator for low-gain operation"] _0,
    #[doc = "Configure crystal oscillator for high-gain operation"] _1,
}
impl HGOR {
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
            HGOR::_0 => false,
            HGOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HGOR {
        match value {
            false => HGOR::_0,
            true => HGOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HGOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HGOR::_1
    }
}
#[doc = "Possible values of the field `RANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGER {
    #[doc = "Low frequency range selected for the crystal oscillator"] _01,
    #[doc = "Medium frequency range selected for the crytstal oscillator"] _10,
    #[doc = "High frequency range selected for the crystal oscillator"] _11,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl RANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RANGER::_01 => 1,
            RANGER::_10 => 2,
            RANGER::_11 => 3,
            RANGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RANGER {
        match value {
            1 => RANGER::_01,
            2 => RANGER::_10,
            3 => RANGER::_11,
            i => RANGER::_Reserved(i),
        }
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
#[doc = "Values that can be written to the field `EREFS`"]
pub enum EREFSW {
    #[doc = "External reference clock selected"] _0,
    #[doc = "Internal crystal oscillator of OSC selected."] _1,
}
impl EREFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EREFSW::_0 => false,
            EREFSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EREFSW<'a> {
    w: &'a mut W,
}
impl<'a> _EREFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EREFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External reference clock selected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFSW::_0)
    }
    #[doc = "Internal crystal oscillator of OSC selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFSW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HGO`"]
pub enum HGOW {
    #[doc = "Configure crystal oscillator for low-gain operation"] _0,
    #[doc = "Configure crystal oscillator for high-gain operation"] _1,
}
impl HGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HGOW::_0 => false,
            HGOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HGOW<'a> {
    w: &'a mut W,
}
impl<'a> _HGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HGOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure crystal oscillator for low-gain operation"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGOW::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGOW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RANGE`"]
pub enum RANGEW {
    #[doc = "Low frequency range selected for the crystal oscillator"] _01,
    #[doc = "Medium frequency range selected for the crytstal oscillator"] _10,
    #[doc = "High frequency range selected for the crystal oscillator"] _11,
}
impl RANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
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
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low frequency range selected for the crystal oscillator"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGEW::_01)
    }
    #[doc = "Medium frequency range selected for the crytstal oscillator"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RANGEW::_10)
    }
    #[doc = "High frequency range selected for the crystal oscillator"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RANGEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs(&self) -> EREFSR {
        EREFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo(&self) -> HGOR {
        HGOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline]
    pub fn range(&self) -> RANGER {
        RANGER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline]
    pub fn erefs(&mut self) -> _EREFSW {
        _EREFSW { w: self }
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline]
    pub fn hgo(&mut self) -> _HGOW {
        _HGOW { w: self }
    }
    #[doc = "Bits 4:5 - System OSC Range Select"]
    #[inline]
    pub fn range(&mut self) -> _RANGEW {
        _RANGEW { w: self }
    }
}
