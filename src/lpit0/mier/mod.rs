#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIER {
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
#[doc = "Possible values of the field `TIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE0R {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE0R {
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
            TIE0R::_0 => false,
            TIE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIE0R {
        match value {
            false => TIE0R::_0,
            true => TIE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIE0R::_1
    }
}
#[doc = "Possible values of the field `TIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE1R {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE1R {
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
            TIE1R::_0 => false,
            TIE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIE1R {
        match value {
            false => TIE1R::_0,
            true => TIE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIE1R::_1
    }
}
#[doc = "Possible values of the field `TIE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE2R {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE2R {
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
            TIE2R::_0 => false,
            TIE2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIE2R {
        match value {
            false => TIE2R::_0,
            true => TIE2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIE2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIE2R::_1
    }
}
#[doc = "Possible values of the field `TIE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE3R {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE3R {
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
            TIE3R::_0 => false,
            TIE3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIE3R {
        match value {
            false => TIE3R::_0,
            true => TIE3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIE3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIE3R::_1
    }
}
#[doc = "Values that can be written to the field `TIE0`"]
pub enum TIE0W {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIE0W::_0 => false,
            TIE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _TIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE0W::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE0W::_1)
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
#[doc = "Values that can be written to the field `TIE1`"]
pub enum TIE1W {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIE1W::_0 => false,
            TIE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _TIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE1W::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE1W::_1)
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
#[doc = "Values that can be written to the field `TIE2`"]
pub enum TIE2W {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIE2W::_0 => false,
            TIE2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIE2W<'a> {
    w: &'a mut W,
}
impl<'a> _TIE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE2W::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE2W::_1)
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
#[doc = "Values that can be written to the field `TIE3`"]
pub enum TIE3W {
    #[doc = "Interrupt generation is disabled"] _0,
    #[doc = "Interrupt generation is enabled"] _1,
}
impl TIE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIE3W::_0 => false,
            TIE3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIE3W<'a> {
    w: &'a mut W,
}
impl<'a> _TIE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt generation is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE3W::_0)
    }
    #[doc = "Interrupt generation is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE3W::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline]
    pub fn tie0(&self) -> TIE0R {
        TIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline]
    pub fn tie1(&self) -> TIE1R {
        TIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline]
    pub fn tie2(&self) -> TIE2R {
        TIE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline]
    pub fn tie3(&self) -> TIE3R {
        TIE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Enable"]
    #[inline]
    pub fn tie0(&mut self) -> _TIE0W {
        _TIE0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Enable"]
    #[inline]
    pub fn tie1(&mut self) -> _TIE1W {
        _TIE1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Enable"]
    #[inline]
    pub fn tie2(&mut self) -> _TIE2W {
        _TIE2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Enable"]
    #[inline]
    pub fn tie3(&mut self) -> _TIE3W {
        _TIE3W { w: self }
    }
}
