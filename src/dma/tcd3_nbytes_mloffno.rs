#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCD3_NBYTES_MLOFFNO {
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
#[doc = r" Value of the field"]
pub struct NBYTESR {
    bits: u32,
}
impl NBYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `DMLOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMLOER {
    #[doc = "The minor loop offset is not applied to the DADDR"]
    _0,
    #[doc = "The minor loop offset is applied to the DADDR"]
    _1,
}
impl DMLOER {
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
            DMLOER::_0 => false,
            DMLOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMLOER {
        match value {
            false => DMLOER::_0,
            true => DMLOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMLOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMLOER::_1
    }
}
#[doc = "Possible values of the field `SMLOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMLOER {
    #[doc = "The minor loop offset is not applied to the SADDR"]
    _0,
    #[doc = "The minor loop offset is applied to the SADDR"]
    _1,
}
impl SMLOER {
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
            SMLOER::_0 => false,
            SMLOER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMLOER {
        match value {
            false => SMLOER::_0,
            true => SMLOER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMLOER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMLOER::_1
    }
}
#[doc = r" Proxy"]
pub struct _NBYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NBYTESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMLOE`"]
pub enum DMLOEW {
    #[doc = "The minor loop offset is not applied to the DADDR"]
    _0,
    #[doc = "The minor loop offset is applied to the DADDR"]
    _1,
}
impl DMLOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMLOEW::_0 => false,
            DMLOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMLOEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMLOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMLOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The minor loop offset is not applied to the DADDR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOEW::_0)
    }
    #[doc = "The minor loop offset is applied to the DADDR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOEW::_1)
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
#[doc = "Values that can be written to the field `SMLOE`"]
pub enum SMLOEW {
    #[doc = "The minor loop offset is not applied to the SADDR"]
    _0,
    #[doc = "The minor loop offset is applied to the SADDR"]
    _1,
}
impl SMLOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMLOEW::_0 => false,
            SMLOEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMLOEW<'a> {
    w: &'a mut W,
}
impl<'a> _SMLOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMLOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The minor loop offset is not applied to the SADDR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMLOEW::_0)
    }
    #[doc = "The minor loop offset is applied to the SADDR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMLOEW::_1)
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
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline]
    pub fn nbytes(&self) -> NBYTESR {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        NBYTESR { bits }
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline]
    pub fn dmloe(&self) -> DMLOER {
        DMLOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline]
    pub fn smloe(&self) -> SMLOER {
        SMLOER::_from({
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
    #[doc = "Bits 0:29 - Minor Byte Transfer Count"]
    #[inline]
    pub fn nbytes(&mut self) -> _NBYTESW {
        _NBYTESW { w: self }
    }
    #[doc = "Bit 30 - Destination Minor Loop Offset enable"]
    #[inline]
    pub fn dmloe(&mut self) -> _DMLOEW {
        _DMLOEW { w: self }
    }
    #[doc = "Bit 31 - Source Minor Loop Offset Enable"]
    #[inline]
    pub fn smloe(&mut self) -> _SMLOEW {
        _SMLOEW { w: self }
    }
}
