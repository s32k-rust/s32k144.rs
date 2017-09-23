#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRGMUX_LPIT0 {
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
#[doc = r" Value of the field"]
pub struct SEL0R {
    bits: u8,
}
impl SEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEL1R {
    bits: u8,
}
impl SEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEL2R {
    bits: u8,
}
impl SEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SEL3R {
    bits: u8,
}
impl SEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKR {
    #[doc = "Register can be written."] _0,
    #[doc = "Register cannot be written until the next system Reset."] _1,
}
impl LKR {
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
            LKR::_0 => false,
            LKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LKR {
        match value {
            false => LKR::_0,
            true => LKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LKR::_1
    }
}
#[doc = r" Proxy"]
pub struct _SEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LK`"]
pub enum LKW {
    #[doc = "Register can be written."] _0,
    #[doc = "Register cannot be written until the next system Reset."] _1,
}
impl LKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LKW::_0 => false,
            LKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LKW<'a> {
    w: &'a mut W,
}
impl<'a> _LKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Register can be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LKW::_0)
    }
    #[doc = "Register cannot be written until the next system Reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LKW::_1)
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
    #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
    #[inline]
    pub fn sel0(&self) -> SEL0R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEL0R { bits }
    }
    #[doc = "Bits 8:13 - Trigger MUX Input 1 Source Select"]
    #[inline]
    pub fn sel1(&self) -> SEL1R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEL1R { bits }
    }
    #[doc = "Bits 16:21 - Trigger MUX Input 2 Source Select"]
    #[inline]
    pub fn sel2(&self) -> SEL2R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEL2R { bits }
    }
    #[doc = "Bits 24:29 - Trigger MUX Input 3 Source Select"]
    #[inline]
    pub fn sel3(&self) -> SEL3R {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SEL3R { bits }
    }
    #[doc = "Bit 31 - TRGMUX register lock."]
    #[inline]
    pub fn lk(&self) -> LKR {
        LKR::_from({
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
    #[doc = "Bits 0:5 - Trigger MUX Input 0 Source Select"]
    #[inline]
    pub fn sel0(&mut self) -> _SEL0W {
        _SEL0W { w: self }
    }
    #[doc = "Bits 8:13 - Trigger MUX Input 1 Source Select"]
    #[inline]
    pub fn sel1(&mut self) -> _SEL1W {
        _SEL1W { w: self }
    }
    #[doc = "Bits 16:21 - Trigger MUX Input 2 Source Select"]
    #[inline]
    pub fn sel2(&mut self) -> _SEL2W {
        _SEL2W { w: self }
    }
    #[doc = "Bits 24:29 - Trigger MUX Input 3 Source Select"]
    #[inline]
    pub fn sel3(&mut self) -> _SEL3W {
        _SEL3W { w: self }
    }
    #[doc = "Bit 31 - TRGMUX register lock."]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
}
