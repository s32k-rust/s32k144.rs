#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EICHEN {
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
#[doc = "Possible values of the field `EICH1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH1ENR {
    #[doc = "Error injection is disabled on Error Injection Channel 1"] _0,
    #[doc = "Error injection is enabled on Error Injection Channel 1"] _1,
}
impl EICH1ENR {
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
            EICH1ENR::_0 => false,
            EICH1ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EICH1ENR {
        match value {
            false => EICH1ENR::_0,
            true => EICH1ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EICH1ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EICH1ENR::_1
    }
}
#[doc = "Possible values of the field `EICH0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH0ENR {
    #[doc = "Error injection is disabled on Error Injection Channel 0"] _0,
    #[doc = "Error injection is enabled on Error Injection Channel 0"] _1,
}
impl EICH0ENR {
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
            EICH0ENR::_0 => false,
            EICH0ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EICH0ENR {
        match value {
            false => EICH0ENR::_0,
            true => EICH0ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EICH0ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EICH0ENR::_1
    }
}
#[doc = "Values that can be written to the field `EICH1EN`"]
pub enum EICH1ENW {
    #[doc = "Error injection is disabled on Error Injection Channel 1"] _0,
    #[doc = "Error injection is enabled on Error Injection Channel 1"] _1,
}
impl EICH1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EICH1ENW::_0 => false,
            EICH1ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EICH1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EICH1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EICH1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error injection is disabled on Error Injection Channel 1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EICH1ENW::_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EICH1ENW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EICH0EN`"]
pub enum EICH0ENW {
    #[doc = "Error injection is disabled on Error Injection Channel 0"] _0,
    #[doc = "Error injection is enabled on Error Injection Channel 0"] _1,
}
impl EICH0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EICH0ENW::_0 => false,
            EICH0ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EICH0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EICH0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EICH0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error injection is disabled on Error Injection Channel 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EICH0ENW::_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EICH0ENW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline]
    pub fn eich1en(&self) -> EICH1ENR {
        EICH1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline]
    pub fn eich0en(&self) -> EICH0ENR {
        EICH0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline]
    pub fn eich1en(&mut self) -> _EICH1ENW {
        _EICH1ENW { w: self }
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline]
    pub fn eich0en(&mut self) -> _EICH0ENW {
        _EICH0ENW { w: self }
    }
}
