#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CBT {
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
pub struct EPSEG2R {
    bits: u8,
}
impl EPSEG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPSEG1R {
    bits: u8,
}
impl EPSEG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPROPSEGR {
    bits: u8,
}
impl EPROPSEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERJWR {
    bits: u8,
}
impl ERJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EPRESDIVR {
    bits: u16,
}
impl EPRESDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `BTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTFR {
    #[doc = "Extended bit time definitions disabled."]
    _0,
    #[doc = "Extended bit time definitions enabled."]
    _1,
}
impl BTFR {
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
            BTFR::_0 => false,
            BTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BTFR {
        match value {
            false => BTFR::_0,
            true => BTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BTFR::_1
    }
}
#[doc = r" Proxy"]
pub struct _EPSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSEG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPSEG1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPROPSEGW<'a> {
    w: &'a mut W,
}
impl<'a> _EPROPSEGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERJWW<'a> {
    w: &'a mut W,
}
impl<'a> _ERJWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPRESDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPRESDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BTF`"]
pub enum BTFW {
    #[doc = "Extended bit time definitions disabled."]
    _0,
    #[doc = "Extended bit time definitions enabled."]
    _1,
}
impl BTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BTFW::_0 => false,
            BTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BTFW<'a> {
    w: &'a mut W,
}
impl<'a> _BTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Extended bit time definitions disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTFW::_0)
    }
    #[doc = "Extended bit time definitions enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTFW::_1)
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
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline]
    pub fn epseg2(&self) -> EPSEG2R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPSEG2R { bits }
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline]
    pub fn epseg1(&self) -> EPSEG1R {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPSEG1R { bits }
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline]
    pub fn epropseg(&self) -> EPROPSEGR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EPROPSEGR { bits }
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline]
    pub fn erjw(&self) -> ERJWR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERJWR { bits }
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline]
    pub fn epresdiv(&self) -> EPRESDIVR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        EPRESDIVR { bits }
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline]
    pub fn btf(&self) -> BTFR {
        BTFR::_from({
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
    #[doc = "Bits 0:4 - Extended Phase Segment 2"]
    #[inline]
    pub fn epseg2(&mut self) -> _EPSEG2W {
        _EPSEG2W { w: self }
    }
    #[doc = "Bits 5:9 - Extended Phase Segment 1"]
    #[inline]
    pub fn epseg1(&mut self) -> _EPSEG1W {
        _EPSEG1W { w: self }
    }
    #[doc = "Bits 10:15 - Extended Propagation Segment"]
    #[inline]
    pub fn epropseg(&mut self) -> _EPROPSEGW {
        _EPROPSEGW { w: self }
    }
    #[doc = "Bits 16:20 - Extended Resync Jump Width"]
    #[inline]
    pub fn erjw(&mut self) -> _ERJWW {
        _ERJWW { w: self }
    }
    #[doc = "Bits 21:30 - Extended Prescaler Division Factor"]
    #[inline]
    pub fn epresdiv(&mut self) -> _EPRESDIVW {
        _EPRESDIVW { w: self }
    }
    #[doc = "Bit 31 - Bit Timing Format Enable"]
    #[inline]
    pub fn btf(&mut self) -> _BTFW {
        _BTFW { w: self }
    }
}
