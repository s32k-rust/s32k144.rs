#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INVCTRL {
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
#[doc = "Possible values of the field `INV0EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV0ENR {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV0ENR {
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
            INV0ENR::_0 => false,
            INV0ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV0ENR {
        match value {
            false => INV0ENR::_0,
            true => INV0ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INV0ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INV0ENR::_1
    }
}
#[doc = "Possible values of the field `INV1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV1ENR {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV1ENR {
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
            INV1ENR::_0 => false,
            INV1ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV1ENR {
        match value {
            false => INV1ENR::_0,
            true => INV1ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INV1ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INV1ENR::_1
    }
}
#[doc = "Possible values of the field `INV2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV2ENR {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV2ENR {
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
            INV2ENR::_0 => false,
            INV2ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV2ENR {
        match value {
            false => INV2ENR::_0,
            true => INV2ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INV2ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INV2ENR::_1
    }
}
#[doc = "Possible values of the field `INV3EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV3ENR {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV3ENR {
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
            INV3ENR::_0 => false,
            INV3ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INV3ENR {
        match value {
            false => INV3ENR::_0,
            true => INV3ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INV3ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INV3ENR::_1
    }
}
#[doc = "Values that can be written to the field `INV0EN`"]
pub enum INV0ENW {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV0ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV0ENW::_0 => false,
            INV0ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INV0ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV0ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV0ENW::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV0ENW::_1)
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
#[doc = "Values that can be written to the field `INV1EN`"]
pub enum INV1ENW {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV1ENW::_0 => false,
            INV1ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INV1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV1ENW::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV1ENW::_1)
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
#[doc = "Values that can be written to the field `INV2EN`"]
pub enum INV2ENW {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV2ENW::_0 => false,
            INV2ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INV2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV2ENW::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV2ENW::_1)
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
#[doc = "Values that can be written to the field `INV3EN`"]
pub enum INV3ENW {
    #[doc = "Inverting is disabled."]
    _0,
    #[doc = "Inverting is enabled."]
    _1,
}
impl INV3ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INV3ENW::_0 => false,
            INV3ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INV3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INV3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INV3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV3ENW::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV3ENW::_1)
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
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline]
    pub fn inv0en(&self) -> INV0ENR {
        INV0ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline]
    pub fn inv1en(&self) -> INV1ENR {
        INV1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline]
    pub fn inv2en(&self) -> INV2ENR {
        INV2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline]
    pub fn inv3en(&self) -> INV3ENR {
        INV3ENR::_from({
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
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline]
    pub fn inv0en(&mut self) -> _INV0ENW {
        _INV0ENW { w: self }
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline]
    pub fn inv1en(&mut self) -> _INV1ENW {
        _INV1ENW { w: self }
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline]
    pub fn inv2en(&mut self) -> _INV2ENW {
        _INV2ENW { w: self }
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline]
    pub fn inv3en(&mut self) -> _INV3ENW {
        _INV3ENW { w: self }
    }
}
