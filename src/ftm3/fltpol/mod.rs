#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLTPOL {
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
#[doc = "Possible values of the field `FLT0POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT0POLR {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT0POLR {
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
            FLT0POLR::_0 => false,
            FLT0POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT0POLR {
        match value {
            false => FLT0POLR::_0,
            true => FLT0POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT0POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT0POLR::_1
    }
}
#[doc = "Possible values of the field `FLT1POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT1POLR {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT1POLR {
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
            FLT1POLR::_0 => false,
            FLT1POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT1POLR {
        match value {
            false => FLT1POLR::_0,
            true => FLT1POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT1POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT1POLR::_1
    }
}
#[doc = "Possible values of the field `FLT2POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT2POLR {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT2POLR {
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
            FLT2POLR::_0 => false,
            FLT2POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT2POLR {
        match value {
            false => FLT2POLR::_0,
            true => FLT2POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT2POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT2POLR::_1
    }
}
#[doc = "Possible values of the field `FLT3POL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT3POLR {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT3POLR {
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
            FLT3POLR::_0 => false,
            FLT3POLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT3POLR {
        match value {
            false => FLT3POLR::_0,
            true => FLT3POLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT3POLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT3POLR::_1
    }
}
#[doc = "Values that can be written to the field `FLT0POL`"]
pub enum FLT0POLW {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT0POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT0POLW::_0 => false,
            FLT0POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT0POLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT0POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT0POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT0POLW::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT0POLW::_1)
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
#[doc = "Values that can be written to the field `FLT1POL`"]
pub enum FLT1POLW {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT1POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT1POLW::_0 => false,
            FLT1POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT1POLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT1POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT1POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT1POLW::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT1POLW::_1)
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
#[doc = "Values that can be written to the field `FLT2POL`"]
pub enum FLT2POLW {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT2POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT2POLW::_0 => false,
            FLT2POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT2POLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT2POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT2POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT2POLW::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT2POLW::_1)
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
#[doc = "Values that can be written to the field `FLT3POL`"]
pub enum FLT3POLW {
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."] _0,
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."] _1,
}
impl FLT3POLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT3POLW::_0 => false,
            FLT3POLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT3POLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT3POLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT3POLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The fault input polarity is active high. A 1 at the fault input indicates a fault."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT3POLW::_0)
    }
    #[doc = "The fault input polarity is active low. A 0 at the fault input indicates a fault."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT3POLW::_1)
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
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline]
    pub fn flt0pol(&self) -> FLT0POLR {
        FLT0POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline]
    pub fn flt1pol(&self) -> FLT1POLR {
        FLT1POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline]
    pub fn flt2pol(&self) -> FLT2POLR {
        FLT2POLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline]
    pub fn flt3pol(&self) -> FLT3POLR {
        FLT3POLR::_from({
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
    #[doc = "Bit 0 - Fault Input 0 Polarity"]
    #[inline]
    pub fn flt0pol(&mut self) -> _FLT0POLW {
        _FLT0POLW { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Polarity"]
    #[inline]
    pub fn flt1pol(&mut self) -> _FLT1POLW {
        _FLT1POLW { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Polarity"]
    #[inline]
    pub fn flt2pol(&mut self) -> _FLT2POLW {
        _FLT2POLW { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Polarity"]
    #[inline]
    pub fn flt3pol(&mut self) -> _FLT3POLW {
        _FLT3POLW { w: self }
    }
}
