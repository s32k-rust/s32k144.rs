#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSR {
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
#[doc = "Possible values of the field `TIF0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF0R {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF0R {
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
            TIF0R::_0 => false,
            TIF0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIF0R {
        match value {
            false => TIF0R::_0,
            true => TIF0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIF0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIF0R::_1
    }
}
#[doc = "Possible values of the field `TIF1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF1R {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF1R {
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
            TIF1R::_0 => false,
            TIF1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIF1R {
        match value {
            false => TIF1R::_0,
            true => TIF1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIF1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIF1R::_1
    }
}
#[doc = "Possible values of the field `TIF2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF2R {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF2R {
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
            TIF2R::_0 => false,
            TIF2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIF2R {
        match value {
            false => TIF2R::_0,
            true => TIF2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIF2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIF2R::_1
    }
}
#[doc = "Possible values of the field `TIF3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIF3R {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF3R {
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
            TIF3R::_0 => false,
            TIF3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIF3R {
        match value {
            false => TIF3R::_0,
            true => TIF3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIF3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIF3R::_1
    }
}
#[doc = "Values that can be written to the field `TIF0`"]
pub enum TIF0W {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIF0W::_0 => false,
            TIF0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIF0W<'a> {
    w: &'a mut W,
}
impl<'a> _TIF0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIF0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF0W::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF0W::_1)
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
#[doc = "Values that can be written to the field `TIF1`"]
pub enum TIF1W {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIF1W::_0 => false,
            TIF1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _TIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF1W::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF1W::_1)
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
#[doc = "Values that can be written to the field `TIF2`"]
pub enum TIF2W {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIF2W::_0 => false,
            TIF2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _TIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF2W::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF2W::_1)
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
#[doc = "Values that can be written to the field `TIF3`"]
pub enum TIF3W {
    #[doc = "Timer has not timed out"] _0,
    #[doc = "Timeout has occurred"] _1,
}
impl TIF3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIF3W::_0 => false,
            TIF3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _TIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer has not timed out"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIF3W::_0)
    }
    #[doc = "Timeout has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIF3W::_1)
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline]
    pub fn tif0(&self) -> TIF0R {
        TIF0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline]
    pub fn tif1(&self) -> TIF1R {
        TIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline]
    pub fn tif2(&self) -> TIF2R {
        TIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline]
    pub fn tif3(&self) -> TIF3R {
        TIF3R::_from({
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
    #[doc = "Bit 0 - Channel 0 Timer Interrupt Flag"]
    #[inline]
    pub fn tif0(&mut self) -> _TIF0W {
        _TIF0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Timer Interrupt Flag"]
    #[inline]
    pub fn tif1(&mut self) -> _TIF1W {
        _TIF1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Timer Interrupt Flag"]
    #[inline]
    pub fn tif2(&mut self) -> _TIF2W {
        _TIF2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Timer Interrupt Flag"]
    #[inline]
    pub fn tif3(&mut self) -> _TIF3W {
        _TIF3W { w: self }
    }
}
