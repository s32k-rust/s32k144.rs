#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDER {
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
#[doc = "Possible values of the field `TDDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDDER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled"] _1,
}
impl TDDER {
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
            TDDER::_0 => false,
            TDDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDDER {
        match value {
            false => TDDER::_0,
            true => TDDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDDER::_1
    }
}
#[doc = "Possible values of the field `RDDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDDER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RDDER {
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
            RDDER::_0 => false,
            RDDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDDER {
        match value {
            false => RDDER::_0,
            true => RDDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDDER::_1
    }
}
#[doc = "Possible values of the field `AVDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVDER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl AVDER {
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
            AVDER::_0 => false,
            AVDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVDER {
        match value {
            false => AVDER::_0,
            true => AVDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVDER::_1
    }
}
#[doc = "Values that can be written to the field `TDDE`"]
pub enum TDDEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled"] _1,
}
impl TDDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDDEW::_0 => false,
            TDDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDDEW::_0)
    }
    #[doc = "DMA request enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDDEW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDDE`"]
pub enum RDDEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RDDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDDEW::_0 => false,
            RDDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDDEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDDEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDDEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AVDE`"]
pub enum AVDEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl AVDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVDEW::_0 => false,
            AVDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVDEW<'a> {
    w: &'a mut W,
}
impl<'a> _AVDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVDEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVDEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline]
    pub fn tdde(&self) -> TDDER {
        TDDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline]
    pub fn rdde(&self) -> RDDER {
        RDDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Address Valid DMA Enable"]
    #[inline]
    pub fn avde(&self) -> AVDER {
        AVDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline]
    pub fn tdde(&mut self) -> _TDDEW {
        _TDDEW { w: self }
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline]
    pub fn rdde(&mut self) -> _RDDEW {
        _RDDEW { w: self }
    }
    #[doc = "Bit 2 - Address Valid DMA Enable"]
    #[inline]
    pub fn avde(&mut self) -> _AVDEW {
        _AVDEW { w: self }
    }
}
