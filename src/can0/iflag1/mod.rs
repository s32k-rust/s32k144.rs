#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLAG1 {
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
#[doc = "Possible values of the field `BUF0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0IR {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR[RFEN]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR[RFEN]=0."]
    _1,
}
impl BUF0IR {
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
            BUF0IR::_0 => false,
            BUF0IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF0IR {
        match value {
            false => BUF0IR::_0,
            true => BUF0IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF0IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF0IR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BUF4TO1IR {
    bits: u8,
}
impl BUF4TO1IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BUF5I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF5IR {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR[RFEN]=0, or of frame(s) available in the FIFO, when MCR[RFEN]=1"]
    _0,
    #[doc = "MB5 completed transmission/reception when MCR[RFEN]=0, or frame(s) available in the Rx FIFO when MCR[RFEN]=1. It generates a DMA request in case of MCR[RFEN] and MCR[DMA] are enabled."]
    _1,
}
impl BUF5IR {
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
            BUF5IR::_0 => false,
            BUF5IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF5IR {
        match value {
            false => BUF5IR::_0,
            true => BUF5IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF5IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF5IR::_1
    }
}
#[doc = "Possible values of the field `BUF6I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF6IR {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO almost full when MCR[RFEN]=1"]
    _0,
    #[doc = "MB6 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO almost full when MCR[RFEN]=1"]
    _1,
}
impl BUF6IR {
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
            BUF6IR::_0 => false,
            BUF6IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF6IR {
        match value {
            false => BUF6IR::_0,
            true => BUF6IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF6IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF6IR::_1
    }
}
#[doc = "Possible values of the field `BUF7I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF7IR {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO overflow when MCR[RFEN]=1"]
    _0,
    #[doc = "MB7 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO overflow when MCR[RFEN]=1"]
    _1,
}
impl BUF7IR {
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
            BUF7IR::_0 => false,
            BUF7IR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF7IR {
        match value {
            false => BUF7IR::_0,
            true => BUF7IR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BUF7IR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BUF7IR::_1
    }
}
#[doc = r" Value of the field"]
pub struct BUF31TO8IR {
    bits: u32,
}
impl BUF31TO8IR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BUF0I`"]
pub enum BUF0IW {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR[RFEN]=0."]
    _0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR[RFEN]=0."]
    _1,
}
impl BUF0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF0IW::_0 => false,
            BUF0IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF0IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR[RFEN]=0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF0IW::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR[RFEN]=0."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF0IW::_1)
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
#[doc = r" Proxy"]
pub struct _BUF4TO1IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF4TO1IW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUF5I`"]
pub enum BUF5IW {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR[RFEN]=0, or of frame(s) available in the FIFO, when MCR[RFEN]=1"]
    _0,
    #[doc = "MB5 completed transmission/reception when MCR[RFEN]=0, or frame(s) available in the Rx FIFO when MCR[RFEN]=1. It generates a DMA request in case of MCR[RFEN] and MCR[DMA] are enabled."]
    _1,
}
impl BUF5IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF5IW::_0 => false,
            BUF5IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF5IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF5IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF5IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR[RFEN]=0, or of frame(s) available in the FIFO, when MCR[RFEN]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF5IW::_0)
    }
    #[doc = "MB5 completed transmission/reception when MCR[RFEN]=0, or frame(s) available in the Rx FIFO when MCR[RFEN]=1. It generates a DMA request in case of MCR[RFEN] and MCR[DMA] are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF5IW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUF6I`"]
pub enum BUF6IW {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO almost full when MCR[RFEN]=1"]
    _0,
    #[doc = "MB6 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO almost full when MCR[RFEN]=1"]
    _1,
}
impl BUF6IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF6IW::_0 => false,
            BUF6IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF6IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF6IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF6IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO almost full when MCR[RFEN]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF6IW::_0)
    }
    #[doc = "MB6 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO almost full when MCR[RFEN]=1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF6IW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUF7I`"]
pub enum BUF7IW {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO overflow when MCR[RFEN]=1"]
    _0,
    #[doc = "MB7 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO overflow when MCR[RFEN]=1"]
    _1,
}
impl BUF7IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BUF7IW::_0 => false,
            BUF7IW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUF7IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF7IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF7IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR[RFEN]=0, or of Rx FIFO overflow when MCR[RFEN]=1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUF7IW::_0)
    }
    #[doc = "MB7 completed transmission/reception when MCR[RFEN]=0, or Rx FIFO overflow when MCR[RFEN]=1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUF7IW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BUF31TO8IW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF31TO8IW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear FIFO bit"]
    #[inline]
    pub fn buf0i(&self) -> BUF0IR {
        BUF0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i(&self) -> BUF4TO1IR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BUF4TO1IR { bits }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline]
    pub fn buf5i(&self) -> BUF5IR {
        BUF5IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline]
    pub fn buf6i(&self) -> BUF6IR {
        BUF6IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline]
    pub fn buf7i(&self) -> BUF7IR {
        BUF7IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i(&self) -> BUF31TO8IR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        BUF31TO8IR { bits }
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
    #[doc = "Bit 0 - Buffer MB0 Interrupt Or Clear FIFO bit"]
    #[inline]
    pub fn buf0i(&mut self) -> _BUF0IW {
        _BUF0IW { w: self }
    }
    #[doc = "Bits 1:4 - Buffer MB i Interrupt Or \"reserved\""]
    #[inline]
    pub fn buf4to1i(&mut self) -> _BUF4TO1IW {
        _BUF4TO1IW { w: self }
    }
    #[doc = "Bit 5 - Buffer MB5 Interrupt Or \"Frames available in Rx FIFO\""]
    #[inline]
    pub fn buf5i(&mut self) -> _BUF5IW {
        _BUF5IW { w: self }
    }
    #[doc = "Bit 6 - Buffer MB6 Interrupt Or \"Rx FIFO Warning\""]
    #[inline]
    pub fn buf6i(&mut self) -> _BUF6IW {
        _BUF6IW { w: self }
    }
    #[doc = "Bit 7 - Buffer MB7 Interrupt Or \"Rx FIFO Overflow\""]
    #[inline]
    pub fn buf7i(&mut self) -> _BUF7IW {
        _BUF7IW { w: self }
    }
    #[doc = "Bits 8:31 - Buffer MBi Interrupt"]
    #[inline]
    pub fn buf31to8i(&mut self) -> _BUF31TO8IW {
        _BUF31TO8IW { w: self }
    }
}
