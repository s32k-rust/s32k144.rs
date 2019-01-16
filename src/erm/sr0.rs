#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR0 {
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
#[doc = "Possible values of the field `NCE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE1R {
    #[doc = "No non-correctable error event on Memory 1 detected"]
    _0,
    #[doc = "Non-correctable error event on Memory 1 detected"]
    _1,
}
impl NCE1R {
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
            NCE1R::_0 => false,
            NCE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCE1R {
        match value {
            false => NCE1R::_0,
            true => NCE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NCE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NCE1R::_1
    }
}
#[doc = "Possible values of the field `SBC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC1R {
    #[doc = "No single-bit correction event on Memory 1 detected"]
    _0,
    #[doc = "Single-bit correction event on Memory 1 detected"]
    _1,
}
impl SBC1R {
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
            SBC1R::_0 => false,
            SBC1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBC1R {
        match value {
            false => SBC1R::_0,
            true => SBC1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBC1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBC1R::_1
    }
}
#[doc = "Possible values of the field `NCE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE0R {
    #[doc = "No non-correctable error event on Memory 0 detected"]
    _0,
    #[doc = "Non-correctable error event on Memory 0 detected"]
    _1,
}
impl NCE0R {
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
            NCE0R::_0 => false,
            NCE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NCE0R {
        match value {
            false => NCE0R::_0,
            true => NCE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NCE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NCE0R::_1
    }
}
#[doc = "Possible values of the field `SBC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC0R {
    #[doc = "No single-bit correction event on Memory 0 detected"]
    _0,
    #[doc = "Single-bit correction event on Memory 0 detected"]
    _1,
}
impl SBC0R {
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
            SBC0R::_0 => false,
            SBC0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBC0R {
        match value {
            false => SBC0R::_0,
            true => SBC0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBC0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBC0R::_1
    }
}
#[doc = "Values that can be written to the field `NCE1`"]
pub enum NCE1W {
    #[doc = "No non-correctable error event on Memory 1 detected"]
    _0,
    #[doc = "Non-correctable error event on Memory 1 detected"]
    _1,
}
impl NCE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCE1W::_0 => false,
            NCE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCE1W<'a> {
    w: &'a mut W,
}
impl<'a> _NCE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No non-correctable error event on Memory 1 detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCE1W::_0)
    }
    #[doc = "Non-correctable error event on Memory 1 detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCE1W::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBC1`"]
pub enum SBC1W {
    #[doc = "No single-bit correction event on Memory 1 detected"]
    _0,
    #[doc = "Single-bit correction event on Memory 1 detected"]
    _1,
}
impl SBC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBC1W::_0 => false,
            SBC1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SBC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No single-bit correction event on Memory 1 detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBC1W::_0)
    }
    #[doc = "Single-bit correction event on Memory 1 detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBC1W::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NCE0`"]
pub enum NCE0W {
    #[doc = "No non-correctable error event on Memory 0 detected"]
    _0,
    #[doc = "Non-correctable error event on Memory 0 detected"]
    _1,
}
impl NCE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCE0W::_0 => false,
            NCE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCE0W<'a> {
    w: &'a mut W,
}
impl<'a> _NCE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No non-correctable error event on Memory 0 detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCE0W::_0)
    }
    #[doc = "Non-correctable error event on Memory 0 detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCE0W::_1)
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
#[doc = "Values that can be written to the field `SBC0`"]
pub enum SBC0W {
    #[doc = "No single-bit correction event on Memory 0 detected"]
    _0,
    #[doc = "Single-bit correction event on Memory 0 detected"]
    _1,
}
impl SBC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBC0W::_0 => false,
            SBC0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SBC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No single-bit correction event on Memory 0 detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBC0W::_0)
    }
    #[doc = "Single-bit correction event on Memory 0 detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBC0W::_1)
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
    #[doc = "Bit 26 - NCE1"]
    #[inline]
    pub fn nce1(&self) -> NCE1R {
        NCE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - SBC1"]
    #[inline]
    pub fn sbc1(&self) -> SBC1R {
        SBC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - NCE0"]
    #[inline]
    pub fn nce0(&self) -> NCE0R {
        NCE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline]
    pub fn sbc0(&self) -> SBC0R {
        SBC0R::_from({
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
    #[doc = "Bit 26 - NCE1"]
    #[inline]
    pub fn nce1(&mut self) -> _NCE1W {
        _NCE1W { w: self }
    }
    #[doc = "Bit 27 - SBC1"]
    #[inline]
    pub fn sbc1(&mut self) -> _SBC1W {
        _SBC1W { w: self }
    }
    #[doc = "Bit 30 - NCE0"]
    #[inline]
    pub fn nce0(&mut self) -> _NCE0W {
        _NCE0W { w: self }
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline]
    pub fn sbc0(&mut self) -> _SBC0W {
        _SBC0W { w: self }
    }
}
